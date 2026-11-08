//! Relay: move a packet one cell at a time to collect every data bit, then reach the exit. Gates
//! keep time with the move count: `x` gates are open on even moves and `y` gates on odd ones. A
//! step taken now decides which gates the next step can pass. Waiting a move changes the beat.
//! Portals with the same letter are linked. Arrow tiles can only be entered moving their way.
//! Levels are Lua files.

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::levels::{self, Level};
use zacess_scenes::colour::Rgb;

use crate::pixels_ui::{Field, Ink, lump};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, g};

pub const INFO: Info = Info {
    id: "relay",
    name: "Relay",
    icon: ('◉', '@'),
    hue: Hue::Function,
    blurb: "a packet through timed gates",
    hints: "Arrows move · W wait · U undo · R restart",
    help: &[
        "# Goal",
        "Steer a packet to collect every data bit (•) and then reach the exit (⇥), in as few moves as you can.",
        "# Rules",
        "- Each arrow moves the packet one cell. W waits a move in place.",
        "- x gates are open on even moves and y gates on odd moves. The board shows each gate as it will be for your next move: a dashed line when open and solid bars when shut.",
        "- Portals are rings with a letter. Two with the same letter, such as A, are linked: step into one to come out of the other.",
        "- Arrow tiles can only be entered moving the way they point.",
        "- The exit opens once every bit is collected.",
        "# Scoring",
        "100 points for each level, plus 5 for every move under twice its par.",
        "# Winning and losing",
        "Deliver every level to win. There is no way to lose: undo or restart a level as often as you like.",
        "# Keys",
        "- Arrows (or H J K L) move.",
        "- W, . or Enter waits a move.",
        "- U or Backspace undoes. R restarts the level.",
        "Tip: waiting is the only way to change the beat, so count the moves to each gate.",
    ],
    input: Input::Keys,
    unit: "points",
};

pub const TILES: &str = "#. @Eoxy<>^vABCD";

pub const BUILT_IN: &[(&str, &str)] = &[(
    "relay/levels.lua",
    include_str!("../../levels/relay/levels.lua"),
)];

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    got: Vec<bool>,
    moves: u32,
}

pub struct Relay {
    levels: Vec<Level>,
    problems: Vec<String>,
    pub index: usize,
    bits: Vec<(usize, usize)>,
    state: State,
    history: Vec<State>,
    score: u64,
    finished: bool,
    pause: f32,
    reduced: bool,
    total_moves: u32,
}

/// Whether a gate tile is open on move number `moves`.
pub fn gate_open(tile: char, moves: u32) -> bool {
    match tile {
        'x' => moves.is_multiple_of(2),
        'y' => moves % 2 == 1,
        _ => true,
    }
}

impl Relay {
    pub fn new(setup: Setup) -> Self {
        let (levels, problems) = levels::load("relay", BUILT_IN, TILES);
        Self::with_levels(levels, problems, setup)
    }

    pub fn with_levels(levels: Vec<Level>, problems: Vec<String>, setup: Setup) -> Self {
        let mut r = Self {
            levels,
            problems,
            index: 0,
            bits: Vec::new(),
            state: State {
                pos: (0, 0),
                got: Vec::new(),
                moves: 0,
            },
            history: Vec::new(),
            score: 0,
            finished: false,
            pause: 0.0,
            reduced: setup.reduced_motion,
            total_moves: 0,
        };
        r.start();
        r
    }

    fn level(&self) -> &Level {
        &self.levels[self.index]
    }

    fn tile(&self, (x, y): (usize, usize)) -> char {
        self.level().map[y][x]
    }

    fn start(&mut self) {
        if self.levels.is_empty() {
            return;
        }
        let mut bits = Vec::new();
        let mut pos = (1, 1);
        for (y, row) in self.level().map.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                match ch {
                    '@' => pos = (x, y),
                    'o' => bits.push((x, y)),
                    _ => {}
                }
            }
        }
        self.bits = bits;
        self.state = State {
            pos,
            got: vec![false; self.bits.len()],
            moves: 0,
        };
        self.history.clear();
        self.pause = 0.0;
    }

    fn twin(&self, at: (usize, usize)) -> Option<(usize, usize)> {
        let letter = self.tile(at);
        for (y, row) in self.level().map.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                if ch == letter && (x, y) != at {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn bits_left(&self) -> usize {
        self.state.got.iter().filter(|g| !**g).count()
    }

    /// The state after one move. An error says why there is none.
    fn next(&self, s: &State, dx: i32, dy: i32) -> Result<(State, bool), &'static str> {
        if (dx, dy) == (0, 0) {
            // waiting spends a move in place, the only way to change which gates are open
            let waited = State {
                moves: s.moves + 1,
                ..s.clone()
            };
            return Ok((waited, false));
        }
        let (w, h) = (self.level().width() as i32, self.level().height() as i32);
        let (nx, ny) = (s.pos.0 as i32 + dx, s.pos.1 as i32 + dy);
        if nx < 0 || ny < 0 || nx >= w || ny >= h {
            return Err("The edge of the board");
        }
        let mut to = (nx as usize, ny as usize);
        let moves = s.moves + 1;
        let ch = self.tile(to);
        match ch {
            '#' | ' ' => return Err("A wall"),
            'x' | 'y' if !gate_open(ch, moves) => return Err("That gate is shut on this move"),
            '>' if dx != 1 => return Err("That tile only takes a move right"),
            '<' if dx != -1 => return Err("That tile only takes a move left"),
            '^' if dy != -1 => return Err("That tile only takes a move up"),
            'v' if dy != 1 => return Err("That tile only takes a move down"),
            'A'..='D' => {
                if let Some(t) = self.twin(to) {
                    to = t;
                }
            }
            _ => {}
        }
        let mut got = s.got.clone();
        if let Some(i) = self.bits.iter().position(|b| *b == to) {
            got[i] = true;
        }
        let done = self.tile(to) == 'E' && got.iter().all(|g| *g);
        Ok((
            State {
                pos: to,
                got,
                moves,
            },
            done,
        ))
    }

    fn step(&mut self, dx: i32, dy: i32, fx: &mut Fx) {
        match self.next(&self.state, dx, dy) {
            Err(why) => {
                fx.say(why);
                fx.sound(Sfx::Low);
            }
            Ok((next, done)) => {
                let before = self.bits_left();
                let teleported = matches!(self.tile(next.pos), 'A'..='D');
                self.history.push(self.state.clone());
                self.state = next;
                if done {
                    let par = self.level().par;
                    let spare = (2 * par).saturating_sub(self.state.moves);
                    let gained = 100 + 5 * u64::from(spare);
                    self.score += gained;
                    self.total_moves += self.state.moves;
                    fx.sound(Sfx::Win);
                    fx.say(format!(
                        "Delivered in {} moves (par {par}), +{gained}",
                        self.state.moves
                    ));
                    self.pause = if self.reduced { 0.8 } else { 1.5 };
                    if self.index + 1 >= self.levels.len() {
                        self.finished = true;
                    }
                } else if self.bits_left() < before {
                    fx.sound(Sfx::Good);
                    fx.say(match self.bits_left() {
                        0 => "Every bit collected: the exit is open".to_string(),
                        n => format!("Bit collected, {n} to go"),
                    });
                } else if teleported {
                    fx.sound(Sfx::Good);
                    fx.say("Through the portal");
                } else {
                    if self.tile(self.state.pos) == 'E' {
                        fx.say(format!(
                            "{} bits left before the exit opens",
                            self.bits_left()
                        ));
                    }
                    fx.sound(Sfx::Tap);
                }
            }
        }
    }

    fn solved(&self) -> bool {
        !self.levels.is_empty() && self.tile(self.state.pos) == 'E' && self.bits_left() == 0
    }
}

/// A data bit: a small cut gem, lit from the upper left, its size following the tiles.
fn gem(f: &mut Field, ink: &Ink, centre: (f32, f32), s: i32) {
    let pal = ink.pal;
    if s == 1 {
        let (x, y) = (centre.0.floor() as i32, centre.1.floor() as i32 - 1);
        f.set(x, y, ink.highlight(pal.cyan, 0.4));
        f.set(x, y + 1, pal.cyan);
        return;
    }
    let r = s;
    let (cx, cy) = (centre.0.floor() as i32, centre.1.floor() as i32);
    for dy in -r..r {
        let half = if dy < 0 { r + dy } else { r - 1 - dy };
        for dx in -half - 1..=half {
            let colour = if dy < 0 && dx < 0 {
                ink.highlight(pal.cyan, 0.45)
            } else if dy >= 0 && dx >= 0 {
                ink.shadow(pal.cyan, 0.35)
            } else {
                pal.cyan
            };
            f.set(cx + dx, cy + dy, colour);
        }
    }
}

/// An oval ring of half-widths `rx` and `ry`, one pixel thick, for a portal.
fn ring_shape(f: &mut Field, centre: (f32, f32), rx: f32, ry: f32, colour: Rgb) {
    let (x0, x1) = ((centre.0 - rx - 1.0) as i32, (centre.0 + rx + 1.0) as i32);
    let (y0, y1) = ((centre.1 - ry - 1.0) as i32, (centre.1 + ry + 1.0) as i32);
    for y in y0..=y1 {
        for x in x0..=x1 {
            let (dx, dy) = (
                (x as f32 + 0.5 - centre.0) / rx,
                (y as f32 + 0.5 - centre.1) / ry,
            );
            let d = (dx * dx + dy * dy).sqrt();
            let step = 1.2 / rx.min(ry);
            if d <= 1.0 && d > 1.0 - step {
                f.set(x, y, colour);
            }
        }
    }
}

impl Game for Relay {
    fn info(&self) -> &'static Info {
        &INFO
    }

    /// The tiles grow with the window by themselves.
    fn zoomable(&self) -> bool {
        false
    }

    fn animated(&self) -> bool {
        self.pause > 0.0
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        if self.levels.is_empty() || self.pause > 0.0 {
            return;
        }
        match key {
            Key::Up | Key::Char('k' | 'K') => self.step(0, -1, fx),
            Key::Down | Key::Char('j' | 'J') => self.step(0, 1, fx),
            Key::Left | Key::Char('h' | 'H') => self.step(-1, 0, fx),
            Key::Right | Key::Char('l' | 'L') => self.step(1, 0, fx),
            Key::Enter | Key::Char('w' | 'W' | '.') => {
                self.step(0, 0, fx);
                fx.say("Waited a move");
            }
            Key::Backspace | Key::Delete | Key::Char('u' | 'U') => {
                if let Some(s) = self.history.pop() {
                    self.state = s;
                }
            }
            Key::Char('r' | 'R') => {
                self.start();
                fx.say("Restarted");
            }
            _ => {}
        }
    }

    fn tick(&mut self, dt: f32, _fx: &mut Fx) {
        if self.pause > 0.0 {
            self.pause -= dt;
            if self.pause <= 0.0 && self.solved() && !self.finished {
                self.index += 1;
                self.start();
            }
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        if self.levels.is_empty() {
            let mut lines = vec!["No Relay levels could be loaded.".to_string()];
            lines.extend(self.problems.iter().cloned());
            for (i, l) in lines.iter().enumerate() {
                c.text(area.x + 2, area.y + 1 + i as i32, l, pal.warning);
            }
            return;
        }
        let level = self.level();
        // a wall all round the map is the board's own frame, so it is not drawn twice
        let ring = i32::from(level.walled());
        let (w, h) = (level.width() as i32, level.height() as i32);
        let panel_w = 24;
        // each tile grows by a whole step while the board and the panel still fit
        let (tiles_w, tiles_h) = (w - 2 * ring, h - 2 * ring);
        let s = (1..=4)
            .rev()
            .find(|&s| tiles_w * 3 * s + 4 + panel_w <= area.w && tiles_h * s + 2 <= area.h)
            .unwrap_or(1);
        let (tw, th) = (3 * s, 2 * s);
        let board_w = tiles_w * tw + 2;
        let board_h = tiles_h * s + 2;
        let r = area.centre(board_w + 2 + panel_w, board_h.max(14));
        let board = Rect::new(r.x, r.y, board_w, board_h);
        c.titled(board, Line::Heavy, pal.function, &level.name, pal.bright);
        let next = self.state.moves + 1;
        let ink = Ink::new(pal);
        let inner = Rect::new(board.x + 1, board.y + 1, board_w - 2, board_h - 2);
        let mut f = Field::new(pal, inner, ink.back());
        // floor tiles in a quiet checker, so the grid can be counted without lines
        let floor = [
            ink.surface(pal.function, 0.06),
            ink.surface(pal.function, 0.14),
        ];
        let wall_top = ink.soften(pal.muted, 0.25);
        let wall_low = ink.soften(pal.muted, 0.6);
        let mortar = ink.soften(pal.muted, 0.75);
        let bar = (s / 2).max(1);
        for y in ring as usize..(h - ring) as usize {
            for x in ring as usize..(w - ring) as usize {
                let px = (x as i32 - ring) * tw;
                let py = (y as i32 - ring) * th;
                let ch = level.map[y][x];
                f.fill_rect(px, py, tw, th, floor[(x + y) % 2]);
                // the sides of a tile in one colour, leaving its middle for a letter
                let sides = |f: &mut Field, colour| {
                    f.fill_rect(px, py, bar, th, colour);
                    f.fill_rect(px + tw - bar, py, bar, th, colour);
                };
                let centre = (px as f32 + tw as f32 / 2.0, py as f32 + th as f32 / 2.0);
                match ch {
                    '#' => {
                        // a block lit from above; larger tiles show their bricks
                        f.fill_rect(px, py, tw, th / 2, wall_top);
                        f.fill_rect(px, py + th / 2, tw, th - th / 2, wall_low);
                        if s >= 2 {
                            f.fill_rect(px, py + th / 2 - 1, tw, 1, mortar);
                            let joint = if y % 2 == 0 { tw / 2 } else { tw / 4 };
                            f.fill_rect(px + joint, py, 1, th / 2 - 1, mortar);
                            f.fill_rect(px + (joint + tw / 2) % tw, py + th / 2, 1, th / 2, mortar);
                        }
                    }
                    'E' => {
                        let open = self.bits_left() == 0;
                        let colour = if open {
                            pal.green
                        } else {
                            ink.soften(pal.muted, 0.3)
                        };
                        sides(&mut f, colour);
                        if s >= 2 {
                            f.fill_rect(px, py, tw, bar, colour);
                        }
                    }
                    'A'..='D' => {
                        if s >= 2 {
                            ring_shape(
                                &mut f,
                                centre,
                                tw as f32 / 2.0 - 0.5,
                                th as f32 / 2.0 - 0.5,
                                pal.magenta,
                            );
                        } else {
                            sides(&mut f, pal.magenta);
                        }
                    }
                    'o' => {
                        let taken = self
                            .bits
                            .iter()
                            .position(|b| *b == (x, y))
                            .is_some_and(|i| self.state.got[i]);
                        if !taken {
                            gem(&mut f, &ink, centre, s);
                        }
                    }
                    'x' | 'y' => {
                        // the letter keeps the tile's top left cell; the gate is drawn beside it
                        if gate_open(ch, next) {
                            let dash = ink.soften(pal.green, 0.2);
                            for k in (0..th).step_by(2) {
                                f.fill_rect(px + tw / 2, py + k, bar, 1, dash);
                            }
                        } else {
                            let lit = ink.highlight(pal.warning, 0.25);
                            for k in 0..3 {
                                let bx = px + s + k * (tw - s - bar) / 2;
                                f.fill_rect(bx, py, bar, th, pal.warning);
                                f.fill_rect(bx, py, bar, 1, lit);
                            }
                            f.fill_rect(px + s, py + th / 2, tw - s, 1, pal.warning);
                        }
                    }
                    _ => {}
                }
                if (x, y) == self.state.pos {
                    if s >= 2 {
                        let tones = [
                            ink.highlight(pal.function, 0.4),
                            pal.function,
                            ink.shadow(pal.function, 0.4),
                        ];
                        let r = (th as f32 * 0.42).max(1.5);
                        lump(&mut f, centre, r, tones, 0.0, 0.0);
                    } else {
                        sides(&mut f, pal.function);
                    }
                }
            }
        }
        f.paint(c);
        // letters, arrows and the packet's mark are text on the floor
        for y in ring as usize..(h - ring) as usize {
            for x in ring as usize..(w - ring) as usize {
                let tx = board.x + 1 + (x as i32 - ring) * tw;
                let ty = board.y + 1 + (y as i32 - ring) * s;
                let (cx, cy) = (tx + tw / 2, ty + s / 2);
                let ch = level.map[y][x];
                match ch {
                    'E' => {
                        let open = self.bits_left() == 0;
                        let colour = if open { pal.green } else { pal.muted };
                        c.put(cx, cy, g('⇥', 'E'), colour);
                    }
                    'x' | 'y' => {
                        let colour = if gate_open(ch, next) {
                            pal.muted
                        } else {
                            pal.warning
                        };
                        c.put(tx, ty, ch, colour);
                    }
                    '>' | '<' | '^' | 'v' => {
                        let arrow = match ch {
                            '>' => g('→', '>'),
                            '<' => g('←', '<'),
                            '^' => g('↑', '^'),
                            _ => g('↓', 'v'),
                        };
                        c.put(cx, cy, arrow, pal.blue);
                    }
                    'A'..='D' => c.put(cx, cy, ch, pal.magenta),
                    _ => {}
                }
                if (x, y) == self.state.pos && s == 1 {
                    c.put(cx, cy, g('◉', '@'), pal.function);
                }
            }
        }
        let p = Rect::new(board.x + board_w + 2, board.y, panel_w, board_h.max(14));
        c.titled(p, Line::Light, pal.muted, "Packet", pal.bright);
        c.text(
            p.x + 2,
            p.y + 2,
            &format!("Level {} of {}", self.index + 1, self.levels.len()),
            pal.text,
        );
        c.text(
            p.x + 2,
            p.y + 3,
            &format!("Moves {}  par {}", self.state.moves, level.par),
            pal.text,
        );
        let parity = if next.is_multiple_of(2) {
            "even"
        } else {
            "odd"
        };
        c.text(
            p.x + 2,
            p.y + 5,
            &format!("Next move: {parity}"),
            pal.function,
        );
        let (xo, yo) = if next.is_multiple_of(2) {
            ("open", "shut")
        } else {
            ("shut", "open")
        };
        c.text(p.x + 2, p.y + 6, &format!("x gates {xo}"), pal.text);
        c.text(p.x + 2, p.y + 7, &format!("y gates {yo}"), pal.text);
        c.text(
            p.x + 2,
            p.y + 9,
            &format!("{} {} bits left", g('•', 'o'), self.bits_left()),
            pal.cyan,
        );
        let note = super::trace::wrap(&level.note, panel_w as usize - 4);
        for (y, line) in (p.y + 10..).zip(note.iter().take(3)) {
            c.text(p.x + 2, y, line, pal.muted);
        }
    }

    fn status(&self) -> Status {
        vec![
            ("Score".into(), self.score.to_string()),
            (
                "Level".into(),
                format!("{}/{}", self.index + 1, self.levels.len().max(1)),
            ),
            ("Moves".into(), self.state.moves.to_string()),
        ]
    }

    fn score(&self) -> u64 {
        self.score
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.levels.is_empty() {
            return Some(Outcome::Lost("No levels to play.".into()));
        }
        (self.finished && self.pause <= 0.0).then(|| {
            Outcome::Won(format!(
                "All {} relays delivered in {} moves.",
                self.levels.len(),
                self.total_moves
            ))
        })
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        // deliver the first packet, then start on the second level's gates
        let mut keys = vec![(Key::Right, 0.0); 7];
        keys.push((Key::Right, 1.8));
        keys.extend([(Key::Right, 0.0), (Key::Right, 0.0), (Key::Char('w'), 0.0)]);
        keys
    }
}

/// The fewest moves that finish a level, by a search over position, bits held and the move
/// count's parity. Used by the tests to prove every shipped level can be solved within par.
#[cfg(test)]
pub fn solve(level: &Level) -> Option<u32> {
    use std::collections::{HashSet, VecDeque};
    let setup = Setup {
        seed: 0,
        reduced_motion: true,
        day: 0,
        practice: false,
    };
    let r = Relay::with_levels(vec![level.clone()], Vec::new(), setup);
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(r.state.clone());
    seen.insert((r.state.pos, r.state.got.clone(), 0));
    while let Some(s) = queue.pop_front() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1), (0, 0)] {
            if let Ok((n, done)) = r.next(&s, dx, dy) {
                if done {
                    return Some(n.moves);
                }
                if seen.insert((n.pos, n.got.clone(), n.moves % 2)) {
                    queue.push_back(n);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Setup {
        Setup {
            seed: 0,
            reduced_motion: false,
            day: 0,
            practice: false,
        }
    }

    fn level(map: &[&str]) -> Level {
        Level {
            name: "t".into(),
            note: String::new(),
            map: map.iter().map(|r| r.chars().collect()).collect(),
            par: 5,
            budget: 0,
        }
    }

    #[test]
    fn every_built_in_level_can_be_solved_within_par() {
        let (levels, problems) = levels::load("relay-test-none", BUILT_IN, TILES);
        assert!(problems.is_empty(), "{problems:?}");
        assert!(levels.len() >= 6);
        for l in &levels {
            let best = solve(l).unwrap_or_else(|| panic!("{} cannot be solved", l.name));
            assert!(best <= l.par, "{}: best {best}, par {}", l.name, l.par);
        }
    }

    #[test]
    fn the_exit_waits_for_every_bit() {
        let mut r = Relay::with_levels(vec![level(&["@Eo"])], vec![], setup());
        let mut fx = Fx::default();
        r.step(1, 0, &mut fx);
        assert!(!r.solved());
        r.step(1, 0, &mut fx);
        r.step(-1, 0, &mut fx);
        assert!(r.solved());
    }

    #[test]
    fn gates_keep_time_with_the_moves() {
        assert!(!gate_open('x', 1));
        assert!(gate_open('x', 2));
        assert!(gate_open('y', 1));
        let mut r = Relay::with_levels(vec![level(&["@x.E"])], vec![], setup());
        let mut fx = Fx::default();
        r.step(1, 0, &mut fx);
        assert_eq!(r.state.pos, (0, 0), "move 1 is odd, so x is shut");
    }

    #[test]
    fn portals_link_and_arrows_point_one_way() {
        let mut r = Relay::with_levels(vec![level(&["@A#.A.E"])], vec![], setup());
        let mut fx = Fx::default();
        r.step(1, 0, &mut fx);
        assert_eq!(r.state.pos, (4, 0));
        let mut r = Relay::with_levels(vec![level(&["@<E"])], vec![], setup());
        r.step(1, 0, &mut fx);
        assert_eq!(r.state.pos, (0, 0));
    }

    #[test]
    fn undo_steps_back() {
        let mut r = Relay::with_levels(vec![level(&["@..E"])], vec![], setup());
        let mut fx = Fx::default();
        r.step(1, 0, &mut fx);
        r.key(Key::Char('u'), &mut fx);
        assert_eq!(r.state.pos, (0, 0));
        assert_eq!(r.state.moves, 0);
    }
}
