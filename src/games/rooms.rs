//! Rooms: a short roguelike through the rooms of a broken server. Each room has faults (a loose
//! cable, a dead fan, a blown fuse or a failed disk) and the parts to fix them lie somewhere in
//! the same room. Hands hold two parts. Fixing every fault in a room opens its doors. Glitches
//! wander the racks; touching one costs uptime, walking into one clears it. Fix every room before
//! uptime runs out. Turn based: nothing moves until the player does.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, g};

pub const INFO: Info = Info {
    id: "rooms",
    name: "Rooms",
    icon: ('▦', '#'),
    hue: Hue::Red,
    blurb: "mend a broken server's rooms",
    hints: "Arrows walk · D drops a part",
    help: &[
        "# Goal",
        "Fix every fault in the six rooms of a broken server before its uptime runs out.",
        "# Rules",
        "- Faults are shown as a capital letter and !: C cable, F fan, U fuse and D disk. The parts that fix them (small letters c, f, u and d) lie in the same room.",
        "- Walk onto a part to pick it up. Hands hold two parts; D drops one.",
        "- Walk into a fault holding the matching part to fix it (✓).",
        "- When every fault in a room is fixed its doors open and you can walk through to the next room. The map shows the rooms fixed.",
        "- Glitches (g) wander the racks. One that walks into you costs 8% uptime. Walking into a glitch clears it.",
        "# Scoring",
        "100 points for each fault, 200 more for each room, 50 for each new room reached and 20 for each glitch cleared. Fixing every room adds 10 points for each percent of uptime left.",
        "# Winning and losing",
        "Fix every room to win. The game ends if uptime reaches 0%.",
        "# Keys",
        "- Arrows (or H J K L) walk, in upper or lower case.",
        "- D drops the first part you hold.",
        "Tip: glitches only move when you do, so stop and plan.",
    ],
    input: Input::Keys,
    unit: "points",
};

pub const RW: usize = 22;
pub const RH: usize = 11;
const COLS: usize = 3;
const ROWS: usize = 2;
const HANDS: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Cable,
    Fan,
    Fuse,
    Disk,
}

impl Kind {
    const ALL: [Kind; 4] = [Kind::Cable, Kind::Fan, Kind::Fuse, Kind::Disk];

    pub fn letter(self) -> char {
        match self {
            Kind::Cable => 'c',
            Kind::Fan => 'f',
            Kind::Fuse => 'u',
            Kind::Disk => 'd',
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Kind::Cable => "cable",
            Kind::Fan => "fan",
            Kind::Fuse => "fuse",
            Kind::Disk => "disk",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    Rack,
    Door,
    Fault(Kind),
    Fixed,
    Part(Kind),
}

pub struct Room {
    pub tiles: [[Tile; RW]; RH],
    pub glitches: Vec<(usize, usize)>,
    pub visited: bool,
}

impl Room {
    pub fn faults(&self) -> usize {
        self.tiles
            .iter()
            .flatten()
            .filter(|t| matches!(t, Tile::Fault(_)))
            .count()
    }
}

pub struct Rooms {
    pub rooms: Vec<Room>,
    pub at: usize,
    pub pos: (usize, usize),
    pub hands: Vec<Kind>,
    pub uptime: i32,
    turns: u32,
    score: u64,
    rng: Rng,
}

fn generate(rng: &mut Rng, index: usize) -> Room {
    let mut tiles = [[Tile::Floor; RW]; RH];
    tiles[0] = [Tile::Wall; RW];
    tiles[RH - 1] = [Tile::Wall; RW];
    for row in tiles.iter_mut() {
        row[0] = Tile::Wall;
        row[RW - 1] = Tile::Wall;
    }
    // rows of racks with gaps, kept off the ring of floor inside the walls so every cell joins up
    for y in (3..RH - 3).step_by(2) {
        let mut x = 3;
        while x < RW - 3 {
            let len = 2 + rng.below(4);
            for i in 0..len {
                if x + i < RW - 3 {
                    tiles[y][x + i] = Tile::Rack;
                }
            }
            x += len + 2 + rng.below(2);
        }
    }
    let (cx, cy) = (index % COLS, index / COLS);
    if cx > 0 {
        tiles[RH / 2][0] = Tile::Door;
    }
    if cx + 1 < COLS {
        tiles[RH / 2][RW - 1] = Tile::Door;
    }
    if cy > 0 {
        tiles[0][RW / 2] = Tile::Door;
    }
    if cy + 1 < ROWS {
        tiles[RH - 1][RW / 2] = Tile::Door;
    }
    let free = |tiles: &[[Tile; RW]; RH], rng: &mut Rng, ring_ok: bool| loop {
        let x = 1 + rng.below(RW - 2);
        let y = 1 + rng.below(RH - 2);
        let on_ring = x == 1 || y == 1 || x == RW - 2 || y == RH - 2;
        // the middle of each wall's ring is left free, so no door is ever blocked
        let by_door =
            (y == RH / 2 && (x == 1 || x == RW - 2)) || (x == RW / 2 && (y == 1 || y == RH - 2));
        if tiles[y][x] == Tile::Floor && (ring_ok || !on_ring) && !by_door {
            return (x, y);
        }
    };
    let faults = 1 + usize::from(index >= 2) + usize::from(index >= 4);
    for _ in 0..faults {
        let kind = *rng.pick(&Kind::ALL);
        // faults sit off the ring, so they never cut a path
        let (x, y) = free(&tiles, rng, false);
        tiles[y][x] = Tile::Fault(kind);
        let (px, py) = free(&tiles, rng, true);
        tiles[py][px] = Tile::Part(kind);
    }
    // a spare part to make the hands matter
    let spare = *rng.pick(&Kind::ALL);
    let (sx, sy) = free(&tiles, rng, true);
    tiles[sy][sx] = Tile::Part(spare);
    let mut glitches = Vec::new();
    for _ in 0..index.min(3) {
        glitches.push(free(&tiles, rng, true));
    }
    Room {
        tiles,
        glitches,
        visited: index == 0,
    }
}

impl Rooms {
    pub fn new(setup: Setup) -> Self {
        let mut rng = Rng::new(setup.seed);
        let rooms = (0..COLS * ROWS).map(|i| generate(&mut rng, i)).collect();
        Self {
            rooms,
            at: 0,
            pos: (2, RH / 2),
            hands: Vec::new(),
            uptime: 100,
            turns: 0,
            score: 0,
            rng,
        }
    }

    fn room(&self) -> &Room {
        &self.rooms[self.at]
    }

    pub fn rooms_fixed(&self) -> usize {
        self.rooms.iter().filter(|r| r.faults() == 0).count()
    }

    fn walk(&mut self, dx: i32, dy: i32, fx: &mut Fx) {
        let (nx, ny) = (self.pos.0 as i32 + dx, self.pos.1 as i32 + dy);
        if nx < 0 || ny < 0 || nx >= RW as i32 || ny >= RH as i32 {
            return;
        }
        let to = (nx as usize, ny as usize);
        if let Some(i) = self.room().glitches.iter().position(|g| *g == to) {
            self.rooms[self.at].glitches.remove(i);
            self.score += 20;
            fx.say("Glitch cleared, +20");
            fx.sound(Sfx::Good);
            self.turn(fx);
            return;
        }
        match self.room().tiles[to.1][to.0] {
            Tile::Wall | Tile::Rack | Tile::Fixed => return,
            Tile::Door => {
                if self.room().faults() > 0 {
                    fx.say(format!(
                        "The door is locked: {} fault{} left here",
                        self.room().faults(),
                        if self.room().faults() == 1 { "" } else { "s" }
                    ));
                    fx.sound(Sfx::Low);
                    return;
                }
                self.through_door(to, fx);
                return;
            }
            Tile::Fault(kind) => {
                if let Some(i) = self.hands.iter().position(|k| *k == kind) {
                    self.hands.remove(i);
                    self.rooms[self.at].tiles[to.1][to.0] = Tile::Fixed;
                    self.score += 100;
                    if self.room().faults() == 0 {
                        self.score += 200;
                        fx.say("Room fixed: the doors are open, +300");
                        fx.sound(Sfx::Win);
                    } else {
                        fx.say(format!("The {} is fixed, +100", kind.name()));
                        fx.sound(Sfx::Good);
                    }
                } else {
                    fx.say(format!("This needs a {} ({})", kind.name(), kind.letter()));
                    fx.sound(Sfx::Low);
                }
                self.turn(fx);
                return;
            }
            Tile::Part(kind) => {
                if self.hands.len() < HANDS {
                    self.hands.push(kind);
                    self.rooms[self.at].tiles[to.1][to.0] = Tile::Floor;
                    fx.say(format!("Picked up a {}", kind.name()));
                    fx.sound(Sfx::Tap);
                } else {
                    fx.say("Hands full: D drops a part");
                }
            }
            Tile::Floor => fx.sound(Sfx::Tick),
        }
        self.pos = to;
        self.turn(fx);
    }

    fn through_door(&mut self, door: (usize, usize), fx: &mut Fx) {
        let (cx, cy) = (self.at % COLS, self.at / COLS);
        let (next, pos) = if door.0 == 0 {
            (self.at - 1, (RW - 2, RH / 2))
        } else if door.0 == RW - 1 {
            (self.at + 1, (1, RH / 2))
        } else if door.1 == 0 {
            (self.at - COLS, (RW / 2, RH - 2))
        } else {
            (self.at + COLS, (RW / 2, 1))
        };
        let _ = (cx, cy);
        self.at = next;
        self.pos = pos;
        // a glitch waiting on the threshold steps aside
        self.rooms[next].glitches.retain(|g| *g != pos);
        if !self.rooms[next].visited {
            self.rooms[next].visited = true;
            self.score += 50;
            fx.say(format!("Server room {} reached, +50", next + 1));
        }
        fx.sound(Sfx::Good);
    }

    fn drop_part(&mut self, fx: &mut Fx) {
        if self.hands.is_empty() {
            fx.say("Nothing to drop");
            return;
        }
        let (x, y) = self.pos;
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            let free = self.room().tiles[ny][nx] == Tile::Floor
                && !self.room().glitches.contains(&(nx, ny));
            if free {
                let kind = self.hands.remove(0);
                self.rooms[self.at].tiles[ny][nx] = Tile::Part(kind);
                fx.say(format!("Dropped the {}", kind.name()));
                self.turn(fx);
                return;
            }
        }
        fx.say("No room to drop it here");
    }

    /// The glitches move after the player: a step towards the player half the time, otherwise
    /// a random step. One that walks into the player costs uptime.
    fn turn(&mut self, fx: &mut Fx) {
        self.turns += 1;
        let pos = self.pos;
        let room = &self.rooms[self.at];
        let mut moved = room.glitches.clone();
        let mut hurt = 0;
        for i in 0..moved.len() {
            let (gx, gy) = moved[i];
            let toward = (
                (pos.0 as i32 - gx as i32).signum(),
                (pos.1 as i32 - gy as i32).signum(),
            );
            let dir = if self.rng.chance(0.5) {
                if self.rng.chance(0.5) && toward.0 != 0 {
                    (toward.0, 0)
                } else {
                    (0, toward.1)
                }
            } else {
                *self.rng.pick(&[(1, 0), (-1, 0), (0, 1), (0, -1)])
            };
            let to = ((gx as i32 + dir.0) as usize, (gy as i32 + dir.1) as usize);
            if to == pos {
                hurt += 1;
                continue;
            }
            let ok = room.tiles[to.1][to.0] == Tile::Floor && !moved.contains(&to);
            if ok {
                moved[i] = to;
            }
        }
        self.rooms[self.at].glitches = moved;
        if hurt > 0 {
            self.uptime -= 8 * hurt;
            fx.say(format!("A glitch hit: uptime {}%", self.uptime.max(0)));
            fx.sound(Sfx::Low);
        }
    }
}

impl Game for Rooms {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn animated(&self) -> bool {
        false
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        match key {
            Key::Up | Key::Char('k' | 'K') => self.walk(0, -1, fx),
            Key::Down | Key::Char('j' | 'J') => self.walk(0, 1, fx),
            Key::Left | Key::Char('h' | 'H') => self.walk(-1, 0, fx),
            Key::Right | Key::Char('l' | 'L') => self.walk(1, 0, fx),
            Key::Char('d' | 'D') => self.drop_part(fx),
            _ => {}
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        let cw = 2;
        // the room's walls are the frame itself; doors are set into it
        let room_w = (RW as i32 - 2) * cw + 2;
        let room_h = RH as i32;
        // the side panel is left out when the window is too narrow for it; the status bar
        // carries what matters
        let panel_w = if area.w >= room_w + 2 + 22 { 22 } else { 0 };
        let r = area.centre(room_w + 2 + panel_w, room_h.max(15));
        let frame = Rect::new(r.x, r.y, room_w, room_h);
        let fixed = self.room().faults() == 0;
        c.titled(
            frame,
            Line::Heavy,
            pal.red,
            &format!("Server room {}", self.at + 1),
            pal.bright,
        );
        let cell_x = |x: usize| frame.x + 1 + (x as i32 - 1) * cw;
        let cell_y = |y: usize| frame.y + y as i32;
        for y in 0..RH {
            for x in 0..RW {
                let tile = self.room().tiles[y][x];
                if tile == Tile::Door {
                    let colour = if fixed { pal.green } else { pal.warning };
                    if x == 0 || x == RW - 1 {
                        let fx = if x == 0 {
                            frame.x
                        } else {
                            frame.x + frame.w - 1
                        };
                        c.put(fx, cell_y(y), if fixed { g('⇋', '=') } else { '▌' }, colour);
                    } else {
                        let glyph = if fixed { g('⇅', '=') } else { '▀' };
                        c.put(cell_x(x), cell_y(y), glyph, colour);
                        c.put(cell_x(x) + 1, cell_y(y), glyph, colour);
                    }
                    continue;
                }
                if x == 0 || y == 0 || x == RW - 1 || y == RH - 1 {
                    continue;
                }
                let (a, b, colour) = match tile {
                    Tile::Floor => ('·', ' ', pal.muted),
                    Tile::Rack => ('▤', '▤', pal.blue),
                    Tile::Fault(k) => (k.letter().to_ascii_uppercase(), '!', pal.error),
                    Tile::Fixed => (g('✓', '+'), ' ', pal.green),
                    Tile::Part(k) => (k.letter(), ' ', pal.yellow),
                    Tile::Wall | Tile::Door => ('▓', '▓', pal.muted),
                };
                c.put(cell_x(x), cell_y(y), a, colour);
                c.put(cell_x(x) + 1, cell_y(y), b, colour);
            }
        }
        for &(gx, gy) in &self.room().glitches {
            c.put(cell_x(gx), cell_y(gy), 'g', pal.magenta);
            c.put(cell_x(gx) + 1, cell_y(gy), g('≈', '~'), pal.magenta);
        }
        let (px, py) = self.pos;
        c.put(cell_x(px), cell_y(py), '@', pal.bright);
        c.put(cell_x(px) + 1, cell_y(py), ' ', pal.bright);

        // the panel: uptime, hands and a map of the rooms
        let p = Rect::new(frame.x + room_w + 2, frame.y, panel_w, room_h.max(15));
        if panel_w == 0 {
            return;
        }
        c.titled(p, Line::Light, pal.muted, "Status", pal.bright);
        c.text(p.x + 2, p.y + 2, "Uptime", pal.muted);
        let up_colour = if self.uptime > 40 {
            pal.green
        } else {
            pal.warning
        };
        c.meter(
            p.x + 2,
            p.y + 3,
            12,
            self.uptime as f32 / 100.0,
            up_colour,
            pal.muted,
        );
        c.text(
            p.x + 15,
            p.y + 3,
            &format!("{}%", self.uptime.max(0)),
            pal.text,
        );
        c.text(p.x + 2, p.y + 5, "Hands", pal.muted);
        for i in 0..HANDS {
            let (text, colour) = match self.hands.get(i) {
                Some(k) => (format!("[{}] {}", k.letter(), k.name()), pal.yellow),
                None => ("[ ] empty".to_string(), pal.muted),
            };
            c.text(p.x + 2, p.y + 6 + i as i32, &text, colour);
        }
        c.text(p.x + 2, p.y + 9, "Map", pal.muted);
        for (i, room) in self.rooms.iter().enumerate() {
            let mx = p.x + 2 + (i % COLS) as i32 * 5;
            let my = p.y + 10 + (i / COLS) as i32 * 2;
            let (text, colour) = if i == self.at {
                (format!("[{}]", '@'), pal.bright)
            } else if room.faults() == 0 {
                (format!("[{}]", g('✓', '+')), pal.green)
            } else if room.visited {
                (format!("[{}]", room.faults()), pal.error)
            } else {
                ("[?]".to_string(), pal.muted)
            };
            c.text(mx, my, &text, colour);
        }
    }

    fn status(&self) -> Status {
        vec![
            ("Score".into(), self.score.to_string()),
            (
                "Fixed".into(),
                format!("{}/{}", self.rooms_fixed(), self.rooms.len()),
            ),
            ("Uptime".into(), format!("{}%", self.uptime.max(0))),
            (
                "Hands".into(),
                if self.hands.is_empty() {
                    "-".into()
                } else {
                    self.hands.iter().map(|k| k.letter()).collect()
                },
            ),
            ("Turn".into(), self.turns.to_string()),
        ]
    }

    fn score(&self) -> u64 {
        self.score
            + if self.rooms_fixed() == self.rooms.len() {
                self.uptime.max(0) as u64 * 10
            } else {
                0
            }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.uptime <= 0 {
            Some(Outcome::Lost(format!(
                "The server went down with {} of {} rooms fixed.",
                self.rooms_fixed(),
                self.rooms.len()
            )))
        } else if self.rooms_fixed() == self.rooms.len() {
            Some(Outcome::Won(format!(
                "Every room is fixed with {}% uptime left.",
                self.uptime
            )))
        } else {
            None
        }
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        vec![
            (Key::Right, 0.0),
            (Key::Right, 0.0),
            (Key::Up, 0.0),
            (Key::Up, 0.0),
            (Key::Right, 0.0),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn game(seed: u64) -> Rooms {
        Rooms::new(Setup {
            seed,
            reduced_motion: false,
            day: 0,
            practice: false,
        })
    }

    /// Every floor, part and door cell of a room can be reached from its ring.
    fn connected(room: &Room) -> bool {
        let open = |t: Tile| matches!(t, Tile::Floor | Tile::Part(_) | Tile::Door);
        let mut seen = [[false; RW]; RH];
        let mut queue = VecDeque::from([(1usize, 1usize)]);
        seen[1][1] = true;
        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || ny < 0 || nx >= RW as i32 || ny >= RH as i32 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if !seen[ny][nx] && open(room.tiles[ny][nx]) {
                    seen[ny][nx] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
        (0..RH).all(|y| (0..RW).all(|x| !open(room.tiles[y][x]) || seen[y][x]))
    }

    #[test]
    fn every_room_is_connected_and_holds_its_own_parts() {
        for seed in 0..40 {
            let g = game(seed);
            for room in &g.rooms {
                assert!(connected(room), "seed {seed}");
                for kind in Kind::ALL {
                    let faults = room
                        .tiles
                        .iter()
                        .flatten()
                        .filter(|t| **t == Tile::Fault(kind))
                        .count();
                    let parts = room
                        .tiles
                        .iter()
                        .flatten()
                        .filter(|t| **t == Tile::Part(kind))
                        .count();
                    assert!(parts >= faults, "seed {seed}");
                }
            }
        }
    }

    #[test]
    fn a_door_stays_locked_until_the_room_is_fixed() {
        let mut g = game(1);
        g.rooms[0].glitches.clear();
        g.pos = (RW - 2, RH / 2);
        let mut fx = Fx::default();
        g.walk(1, 0, &mut fx);
        assert_eq!(g.at, 0);
        for row in g.rooms[0].tiles.iter_mut() {
            for t in row.iter_mut() {
                if matches!(t, Tile::Fault(_)) {
                    *t = Tile::Fixed;
                }
            }
        }
        g.walk(1, 0, &mut fx);
        assert_eq!(g.at, 1);
        assert_eq!(g.pos, (1, RH / 2));
    }

    #[test]
    fn the_right_part_fixes_a_fault_and_hands_hold_two() {
        let mut g = game(2);
        g.rooms[0].glitches.clear();
        g.pos = (5, 1);
        g.rooms[0].tiles[1][6] = Tile::Fault(Kind::Fan);
        g.hands = vec![Kind::Disk];
        let mut fx = Fx::default();
        g.walk(1, 0, &mut fx);
        assert_eq!(g.rooms[0].tiles[1][6], Tile::Fault(Kind::Fan));
        g.hands.push(Kind::Fan);
        g.rooms[0].tiles[2][5] = Tile::Part(Kind::Cable);
        g.walk(0, 1, &mut fx);
        assert_eq!(g.hands.len(), HANDS, "hands were already full");
        assert_eq!(g.rooms[0].tiles[2][5], Tile::Part(Kind::Cable));
        g.walk(0, -1, &mut fx);
        g.walk(1, 0, &mut fx);
        assert_eq!(g.rooms[0].tiles[1][6], Tile::Fixed);
        assert_eq!(g.hands, vec![Kind::Disk]);
    }

    #[test]
    fn walking_into_a_glitch_clears_it_and_zero_uptime_loses() {
        let mut g = game(3);
        g.rooms[0].glitches = vec![(3, 1)];
        g.pos = (2, 1);
        let mut fx = Fx::default();
        g.walk(1, 0, &mut fx);
        assert!(g.rooms[0].glitches.is_empty());
        g.uptime = 0;
        assert!(matches!(g.outcome(), Some(Outcome::Lost(_))));
    }
}
