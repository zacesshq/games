//! Relay: move a packet one cell at a time to collect every data bit, then reach the exit. Gates
//! keep time with the move count: `x` gates are open on even moves and `y` gates on odd ones. A
//! step taken now decides which gates the next step can pass. Waiting a move changes the beat.
//! Portals with the same letter are linked. Arrow tiles can only be entered moving their way.
//! Levels are Lua files.

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::levels::{self, Level};
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
        "- x gates are open on even moves and y gates on odd moves. The board shows each gate as it will be for your next move: open ┊ or shut █.",
        "- Portals with the same letter, such as (A), are linked: step into one to come out of the other.",
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

impl Game for Relay {
    fn info(&self) -> &'static Info {
        &INFO
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
            Key::Backspace | Key::Char('u' | 'U') => {
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
        let board_w = (w - 2 * ring) * 3 + 2;
        let board_h = h - 2 * ring + 2;
        let panel_w = 24;
        let r = area.centre(board_w + 2 + panel_w, board_h.max(14));
        let board = Rect::new(r.x, r.y, board_w, board_h);
        c.titled(board, Line::Heavy, pal.function, &level.name, pal.bright);
        let next = self.state.moves + 1;
        for y in ring as usize..(h - ring) as usize {
            for x in ring as usize..(w - ring) as usize {
                let cx = board.x + 1 + (x as i32 - ring) * 3;
                let cy = board.y + 1 + y as i32 - ring;
                let ch = level.map[y][x];
                match ch {
                    '#' => c.text(cx, cy, "▓▓▓", pal.muted),
                    'E' => {
                        let open = self.bits_left() == 0;
                        let colour = if open { pal.green } else { pal.muted };
                        c.put(cx, cy, if open { ' ' } else { '[' }, colour);
                        c.put(cx + 1, cy, g('⇥', 'E'), colour);
                        c.put(cx + 2, cy, if open { ' ' } else { ']' }, colour);
                    }
                    'o' => {
                        let taken = self
                            .bits
                            .iter()
                            .position(|b| *b == (x, y))
                            .is_some_and(|i| self.state.got[i]);
                        if taken {
                            c.put(cx + 1, cy, '·', pal.muted);
                        } else {
                            c.put(cx + 1, cy, g('•', 'o'), pal.cyan);
                        }
                    }
                    'x' | 'y' => {
                        if gate_open(ch, next) {
                            c.put(cx, cy, ch, pal.muted);
                            c.put(cx + 1, cy, '┊', pal.green);
                        } else {
                            c.put(cx, cy, ch, pal.warning);
                            c.put(cx + 1, cy, '█', pal.warning);
                            c.put(cx + 2, cy, '█', pal.warning);
                        }
                    }
                    '>' | '<' | '^' | 'v' => {
                        let arrow = match ch {
                            '>' => g('→', '>'),
                            '<' => g('←', '<'),
                            '^' => g('↑', '^'),
                            _ => g('↓', 'v'),
                        };
                        c.put(cx + 1, cy, arrow, pal.blue);
                    }
                    'A'..='D' => {
                        c.put(cx, cy, '(', pal.magenta);
                        c.put(cx + 1, cy, ch, pal.magenta);
                        c.put(cx + 2, cy, ')', pal.magenta);
                    }
                    _ => c.put(cx + 1, cy, '·', pal.muted),
                }
                if (x, y) == self.state.pos {
                    c.put(cx, cy, '‹', pal.bright);
                    c.put(cx + 1, cy, g('◉', '@'), pal.function);
                    c.put(cx + 2, cy, '›', pal.bright);
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
