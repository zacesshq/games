//! Merge Gates: tiles carry four-bit values. Push one tile at a time: it slides until it meets a
//! wall or another tile. Meeting a tile combines the two through the gate at the front of the
//! gate track (AND, OR, XOR, NAND, NOR or XNOR, bit by bit). Every push moves the track on by one
//! gate, so the order matters. Make a tile equal to the target output to send it out. Each target
//! has a few pushes; missing one is a fault. Three faults end the game.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, g};

pub const INFO: Info = Info {
    id: "merge-gates",
    name: "Merge Gates",
    icon: ('⊕', '+'),
    hue: Hue::Magenta,
    blurb: "push bit tiles through gates",
    hints: "Arrows move · W A S D push · Enter grab, then an arrow",
    help: &[
        "# Goal",
        "Make a tile equal the target output, as often as you can.",
        "# Rules",
        "- Every tile holds four bits, such as 1010.",
        "- Choose a tile with the cursor and push it one way. It slides until it meets a wall or another tile.",
        "- When it meets a tile the two combine through the gate at the front of the gate track (AND, OR, XOR, NAND, NOR or XNOR), bit by bit, into one tile.",
        "- Every push moves the track on by one gate and a new tile arrives on the board.",
        "- A tile that equals the target is sent out and a new target appears.",
        "# Scoring",
        "100 points for each tile sent out, plus 50 for every merge that went into it.",
        "# Winning and losing",
        "Each target allows 10 pushes. Missing one is a fault. The game ends at the third fault.",
        "# Keys",
        "- Arrows (or H J K L) move the cursor.",
        "- W A S D push the tile under the cursor up, left, down or right, in upper or lower case.",
        "- Enter grabs the tile; the next arrow pushes it.",
        "Tip: read the next gates on the track before you push.",
    ],
    input: Input::Keys,
    unit: "points",
};

pub const SIZE: usize = 5;
const TRACK: usize = 4;
const PUSHES_PER_TARGET: u32 = 10;
const FAULTS: u32 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gate {
    And,
    Or,
    Xor,
    Nand,
    Nor,
    Xnor,
}

impl Gate {
    pub const ALL: [Gate; 6] = [
        Gate::And,
        Gate::Or,
        Gate::Xor,
        Gate::Nand,
        Gate::Nor,
        Gate::Xnor,
    ];

    pub fn apply(self, a: u8, b: u8) -> u8 {
        let v = match self {
            Gate::And => a & b,
            Gate::Or => a | b,
            Gate::Xor => a ^ b,
            Gate::Nand => !(a & b),
            Gate::Nor => !(a | b),
            Gate::Xnor => !(a ^ b),
        };
        v & 0xF
    }

    pub fn name(self) -> &'static str {
        match self {
            Gate::And => "AND",
            Gate::Or => "OR",
            Gate::Xor => "XOR",
            Gate::Nand => "NAND",
            Gate::Nor => "NOR",
            Gate::Xnor => "XNOR",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {
    pub value: u8,
    /// How many merges made it, worth more when sent out.
    pub depth: u8,
}

pub fn bits(v: u8) -> String {
    format!("{:04b}", v & 0xF)
}

pub struct MergeGates {
    pub board: [[Option<Tile>; SIZE]; SIZE],
    pub cursor: (usize, usize),
    grabbed: bool,
    pub track: Vec<Gate>,
    pub target: u8,
    pushes_left: u32,
    faults: u32,
    score: u64,
    sent: u32,
    rng: Rng,
    /// The cell that last changed and how long its mark stays, for a calm highlight.
    glow: Option<((usize, usize), f32)>,
    reduced: bool,
}

impl MergeGates {
    pub fn new(setup: Setup) -> Self {
        let mut rng = Rng::new(setup.seed);
        let mut game = Self {
            board: [[None; SIZE]; SIZE],
            cursor: (2, 2),
            grabbed: false,
            track: (0..TRACK).map(|_| *rng.pick(&Gate::ALL)).collect(),
            target: 0,
            pushes_left: PUSHES_PER_TARGET,
            faults: 0,
            score: 0,
            sent: 0,
            rng,
            glow: None,
            reduced: setup.reduced_motion,
        };
        for _ in 0..7 {
            game.spawn();
        }
        game.new_target();
        game
    }

    fn empty_cells(&self) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.board[y][x].is_none() {
                    cells.push((x, y));
                }
            }
        }
        cells
    }

    fn tiles(&self) -> Vec<Tile> {
        self.board.iter().flatten().flatten().copied().collect()
    }

    fn spawn(&mut self) {
        let empty = self.empty_cells();
        if empty.is_empty() {
            return;
        }
        let (x, y) = *self.rng.pick(&empty);
        let value = 1 + self.rng.below(15) as u8;
        self.board[y][x] = Some(Tile { value, depth: 0 });
    }

    /// A target two tiles on the board can make through one of the gates on the track, so every
    /// target can be reached, though not always by the obvious pair.
    fn new_target(&mut self) {
        let tiles = self.tiles();
        let mut options = Vec::new();
        for (i, a) in tiles.iter().enumerate() {
            for b in tiles.iter().skip(i + 1) {
                for gate in &self.track {
                    let v = gate.apply(a.value, b.value);
                    if !tiles.iter().any(|t| t.value == v) {
                        options.push(v);
                    }
                }
            }
        }
        self.target = if options.is_empty() {
            1 + self.rng.below(15) as u8
        } else {
            *self.rng.pick(&options)
        };
        self.pushes_left = PUSHES_PER_TARGET;
    }

    /// Pushes the tile at `from` one way. Returns false when it cannot move at all.
    pub fn push(&mut self, from: (usize, usize), dir: (i32, i32), fx: &mut Fx) -> bool {
        let Some(tile) = self.board[from.1][from.0] else {
            fx.say("No tile under the cursor");
            return false;
        };
        let (mut x, mut y) = (from.0 as i32, from.1 as i32);
        let mut hit = None;
        loop {
            let (nx, ny) = (x + dir.0, y + dir.1);
            if !(0..SIZE as i32).contains(&nx) || !(0..SIZE as i32).contains(&ny) {
                break;
            }
            if self.board[ny as usize][nx as usize].is_some() {
                hit = Some((nx as usize, ny as usize));
                break;
            }
            x = nx;
            y = ny;
        }
        let moved = (x as usize, y as usize) != from;
        if !moved && hit.is_none() {
            fx.say("It is against the wall");
            fx.sound(Sfx::Low);
            return false;
        }
        let gate = self.track.remove(0);
        self.track.push(*self.rng.pick(&Gate::ALL));
        self.board[from.1][from.0] = None;
        let landed = if let Some((hx, hy)) = hit {
            let other = self.board[hy][hx].unwrap_or(tile);
            let value = gate.apply(tile.value, other.value);
            let depth = tile.depth.max(other.depth) + 1;
            self.board[hy][hx] = Some(Tile { value, depth });
            fx.say(format!(
                "{} {} {} = {}",
                bits(tile.value),
                gate.name(),
                bits(other.value),
                bits(value)
            ));
            fx.sound(Sfx::Tap);
            (hx, hy)
        } else {
            self.board[y as usize][x as usize] = Some(tile);
            fx.sound(Sfx::Tap);
            (x as usize, y as usize)
        };
        self.cursor = landed;
        self.glow = Some((landed, if self.reduced { 0.0 } else { 0.6 }));
        self.after_push(landed, fx);
        true
    }

    fn after_push(&mut self, landed: (usize, usize), fx: &mut Fx) {
        let hit_target = self.board[landed.1][landed.0].is_some_and(|t| t.value == self.target);
        if hit_target {
            let depth = self.board[landed.1][landed.0].map_or(0, |t| t.depth);
            self.board[landed.1][landed.0] = None;
            let gained = 100 + 50 * u64::from(depth);
            self.score += gained;
            self.sent += 1;
            fx.say(format!("Output {} sent, +{gained}", bits(self.target)));
            fx.sound(Sfx::Good);
            self.spawn();
            self.spawn();
            self.new_target();
            return;
        }
        self.spawn();
        self.pushes_left = self.pushes_left.saturating_sub(1);
        if self.pushes_left == 0 {
            self.faults += 1;
            if self.faults >= FAULTS {
                fx.sound(Sfx::Low);
                return;
            }
            fx.say(format!(
                "Missed {}: fault {} of {FAULTS}",
                bits(self.target),
                self.faults
            ));
            fx.sound(Sfx::Low);
            self.new_target();
        }
    }

    fn move_cursor(&mut self, dx: i32, dy: i32) {
        let x = (self.cursor.0 as i32 + dx).clamp(0, SIZE as i32 - 1);
        let y = (self.cursor.1 as i32 + dy).clamp(0, SIZE as i32 - 1);
        self.cursor = (x as usize, y as usize);
    }

    fn tile_colour(&self, c: &Canvas, t: Tile) -> zacess_scenes::colour::Rgb {
        let pal = c.pal;
        // colour follows the count of ones, shown as the bits themselves too
        match (t.value & 0xF).count_ones() {
            0 => pal.muted,
            1 => pal.cyan,
            2 => pal.magenta,
            3 => pal.yellow,
            _ => pal.green,
        }
    }
}

impl Game for MergeGates {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        let push_dir = match key {
            Key::Char('w' | 'W') => Some((0, -1)),
            Key::Char('a' | 'A') => Some((-1, 0)),
            Key::Char('s' | 'S') => Some((0, 1)),
            Key::Char('d' | 'D') => Some((1, 0)),
            _ => None,
        };
        let arrow = match key {
            Key::Up | Key::Char('k') => Some((0, -1)),
            Key::Down | Key::Char('j') => Some((0, 1)),
            Key::Left | Key::Char('h') => Some((-1, 0)),
            Key::Right | Key::Char('l') => Some((1, 0)),
            _ => None,
        };
        if let Some(dir) = push_dir {
            self.grabbed = false;
            self.push(self.cursor, dir, fx);
        } else if let Some((dx, dy)) = arrow {
            if self.grabbed {
                self.grabbed = false;
                self.push(self.cursor, (dx, dy), fx);
            } else {
                self.move_cursor(dx, dy);
                fx.sound(Sfx::Tick);
            }
        } else if key == Key::Enter {
            if self.board[self.cursor.1][self.cursor.0].is_some() {
                self.grabbed = !self.grabbed;
                if self.grabbed {
                    fx.say("Grabbed: an arrow pushes it");
                }
            } else {
                fx.say("No tile under the cursor");
            }
        }
    }

    fn tick(&mut self, dt: f32, _fx: &mut Fx) {
        if let Some((_, t)) = &mut self.glow {
            *t -= dt;
            if *t <= 0.0 {
                self.glow = None;
            }
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        // a short window draws each tile on one line instead of in a box of three
        let flat = area.h < 17;
        let (cw, ch) = if flat { (7, 1) } else { (8, 3) };
        let board_w = SIZE as i32 * cw + 2;
        let board_h = SIZE as i32 * ch + 2;
        let panel_w = if area.w >= board_w + 2 + 24 { 24 } else { 0 };
        let r = area.centre(board_w + 2 + panel_w, board_h);
        let board = Rect::new(r.x, r.y, board_w, board_h);
        c.titled(board, Line::Heavy, pal.magenta, "Board", pal.bright);
        for y in 0..SIZE {
            for x in 0..SIZE {
                let cell = Rect::new(
                    board.x + 1 + x as i32 * cw,
                    board.y + 1 + y as i32 * ch,
                    cw,
                    ch,
                );
                let here = (x, y) == self.cursor;
                if flat {
                    let (open, close) = if here { ('»', '«') } else { ('[', ']') };
                    match self.board[y][x] {
                        Some(t) => {
                            let colour = if here {
                                pal.bright
                            } else {
                                self.tile_colour(c, t)
                            };
                            c.put(cell.x, cell.y, open, colour);
                            c.text(cell.x + 1, cell.y, &bits(t.value), self.tile_colour(c, t));
                            c.put(cell.x + 5, cell.y, close, colour);
                        }
                        None => {
                            if here {
                                c.put(cell.x, cell.y, open, pal.bright);
                                c.put(cell.x + 5, cell.y, close, pal.bright);
                            }
                            c.put(cell.x + 2, cell.y, '·', pal.muted);
                        }
                    }
                    if here && self.grabbed {
                        c.put(cell.x + 6, cell.y, g('✥', '*'), pal.bright);
                    }
                    continue;
                }
                match self.board[y][x] {
                    Some(t) => {
                        let colour = self.tile_colour(c, t);
                        let glowing = self.glow.is_some_and(|(p, _)| p == (x, y));
                        let line = if here {
                            Line::Double
                        } else if glowing {
                            Line::Heavy
                        } else {
                            Line::Round
                        };
                        let edge = if here { pal.bright } else { colour };
                        c.boxed(cell, line, edge);
                        c.text(cell.x + 2, cell.y + 1, &bits(t.value), colour);
                        // merge depth as small pips on the lower edge
                        for d in 0..t.depth.min(4) {
                            c.put(cell.x + 2 + i32::from(d), cell.y + 2, g('•', 'o'), colour);
                        }
                    }
                    None => {
                        c.put(cell.x + cw / 2 - 1, cell.y + 1, '·', pal.muted);
                        if here {
                            c.boxed(cell, Line::Light, pal.text);
                        }
                    }
                }
                if here && self.grabbed {
                    c.put(cell.x + cw - 2, cell.y, g('✥', '*'), pal.bright);
                }
            }
        }
        // the panel: target, track, pushes and faults
        if panel_w == 0 {
            return;
        }
        let p = Rect::new(board.x + board_w + 2, board.y, panel_w, board_h.max(16));
        c.titled(p, Line::Light, pal.muted, "Output", pal.bright);
        c.text(p.x + 2, p.y + 2, "Target", pal.muted);
        let target: String = bits(self.target)
            .chars()
            .map(|b| {
                if b == '1' {
                    g('■', '1')
                } else {
                    g('□', '0')
                }
            })
            .collect::<Vec<char>>()
            .iter()
            .flat_map(|ch| [*ch, ' '])
            .collect();
        c.text(p.x + 2, p.y + 3, &target, pal.bright);
        c.text(p.x + 13, p.y + 3, &bits(self.target), pal.magenta);
        c.text(p.x + 2, p.y + 5, "Gate track", pal.muted);
        for (i, gate) in self.track.iter().enumerate() {
            let y = p.y + 6 + i as i32;
            if i == 0 {
                c.text(
                    p.x + 2,
                    y,
                    &format!("{} {}", g('▶', '>'), gate.name()),
                    pal.bright,
                );
                c.text(p.x + 10, y, "next", pal.magenta);
            } else {
                c.text(p.x + 4, y, gate.name(), pal.text);
            }
        }
        c.text(p.x + 2, p.y + 11, "Pushes left", pal.muted);
        c.meter(
            p.x + 2,
            p.y + 12,
            10,
            self.pushes_left as f32 / PUSHES_PER_TARGET as f32,
            pal.magenta,
            pal.muted,
        );
        c.text(p.x + 13, p.y + 12, &self.pushes_left.to_string(), pal.text);
        c.text(p.x + 2, p.y + 14, "Faults", pal.muted);
        for i in 0..FAULTS {
            let (glyph, colour) = if i < self.faults {
                (g('✕', 'x'), pal.warning)
            } else {
                (g('○', 'o'), pal.muted)
            };
            c.put(p.x + 9 + i as i32 * 2, p.y + 14, glyph, colour);
        }
    }

    fn status(&self) -> Status {
        vec![
            ("Score".into(), self.score.to_string()),
            ("Target".into(), bits(self.target)),
            ("Gate".into(), self.track[0].name().into()),
            ("Sent".into(), self.sent.to_string()),
        ]
    }

    fn score(&self) -> u64 {
        self.score
    }

    fn outcome(&self) -> Option<Outcome> {
        (self.faults >= FAULTS)
            .then(|| Outcome::Lost(format!("Three targets missed after {} sent.", self.sent)))
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        vec![
            (Key::Char('d'), 0.2),
            (Key::Left, 0.1),
            (Key::Char('s'), 0.2),
            (Key::Up, 0.1),
            (Key::Char('a'), 0.2),
            (Key::Enter, 0.1),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(seed: u64) -> Setup {
        Setup {
            seed,
            reduced_motion: false,
            day: 0,
            practice: false,
        }
    }

    fn empty(seed: u64) -> MergeGates {
        let mut g = MergeGates::new(setup(seed));
        g.board = [[None; SIZE]; SIZE];
        g
    }

    #[test]
    fn gates_work_bit_by_bit_on_four_bits() {
        assert_eq!(Gate::And.apply(0b1100, 0b1010), 0b1000);
        assert_eq!(Gate::Or.apply(0b1100, 0b1010), 0b1110);
        assert_eq!(Gate::Xor.apply(0b1100, 0b1010), 0b0110);
        assert_eq!(Gate::Nand.apply(0b1100, 0b1010), 0b0111);
        assert_eq!(Gate::Nor.apply(0b1100, 0b1010), 0b0001);
        assert_eq!(Gate::Xnor.apply(0b1100, 0b1010), 0b1001);
    }

    #[test]
    fn a_pushed_tile_slides_to_the_wall() {
        let mut g = empty(1);
        g.board[2][0] = Some(Tile { value: 5, depth: 0 });
        g.target = 0xF;
        let mut fx = Fx::default();
        assert!(g.push((0, 2), (1, 0), &mut fx));
        assert_eq!(g.board[2][4].map(|t| t.value), Some(5));
        assert!(g.board[2][0].is_none());
    }

    #[test]
    fn meeting_a_tile_merges_through_the_front_gate() {
        let mut g = empty(2);
        g.board[0][0] = Some(Tile {
            value: 0b1100,
            depth: 0,
        });
        g.board[0][3] = Some(Tile {
            value: 0b1010,
            depth: 1,
        });
        g.track = vec![Gate::Xor, Gate::And, Gate::And, Gate::And];
        g.target = 0b0110;
        let before = g.score;
        let mut fx = Fx::default();
        g.push((0, 0), (1, 0), &mut fx);
        // the merge made the target, so it was sent out and scored with its depth
        assert_eq!(g.score - before, 100 + 50 * 2);
        assert!(g.board[0][3].is_none_or(|t| t.value != 0b0110 || t.depth == 0));
        assert_eq!(g.track.len(), TRACK);
    }

    #[test]
    fn a_tile_against_the_wall_does_not_use_a_gate() {
        let mut g = empty(3);
        g.board[0][4] = Some(Tile { value: 1, depth: 0 });
        let track = g.track.clone();
        let mut fx = Fx::default();
        assert!(!g.push((4, 0), (1, 0), &mut fx));
        assert_eq!(g.track, track);
    }

    #[test]
    fn three_missed_targets_end_the_game() {
        let mut g = MergeGates::new(setup(4));
        g.target = 0xFF; // never reachable: tiles hold four bits
        let mut fx = Fx::default();
        let mut guard = 0;
        while g.outcome().is_none() && guard < 500 {
            guard += 1;
            let tiles: Vec<(usize, usize)> = (0..SIZE * SIZE)
                .map(|i| (i % SIZE, i / SIZE))
                .filter(|&(x, y)| g.board[y][x].is_some())
                .collect();
            let mut moved = false;
            for &(x, y) in &tiles {
                for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    if g.push((x, y), dir, &mut Fx::default()) {
                        moved = true;
                        break;
                    }
                }
                if moved {
                    break;
                }
            }
            if g.target != 0xFF && g.outcome().is_none() {
                g.target = 0xFF;
            }
            let _ = &mut fx;
        }
        assert!(matches!(g.outcome(), Some(Outcome::Lost(_))));
    }

    #[test]
    fn the_first_target_can_be_made_from_the_board() {
        for seed in 0..20 {
            let g = MergeGates::new(setup(seed));
            let tiles = g.tiles();
            let reachable = tiles.iter().enumerate().any(|(i, a)| {
                tiles.iter().skip(i + 1).any(|b| {
                    g.track
                        .iter()
                        .any(|gate| gate.apply(a.value, b.value) == g.target)
                })
            });
            assert!(reachable, "seed {seed}");
        }
    }
}
