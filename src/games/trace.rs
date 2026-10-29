//! Trace: route wire from the source to light every node without crossing. A wire starts at a
//! lit point (the source or a node already lit) and runs cell by cell; entering a node lights it
//! and the wire may carry on from there. Wires never cross or share a cell, except on a via (`+`),
//! where one wire may run straight across another. Arrow nodes only accept wire arriving in the
//! arrow's direction. Each level has a wire budget; using less scores more. Levels are Lua files.

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::levels::{self, Level};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, g};

pub const INFO: Info = Info {
    id: "trace",
    name: "Trace",
    icon: ('╋', '+'),
    hue: Hue::Green,
    blurb: "light every node, no crossings",
    hints: "Arrows lay wire · Tab next lit point · U undo · R restart",
    help: &[
        "# Goal",
        "Light every node on the board by running wire to it from the source (◆).",
        "# Rules",
        "- The pen starts at the source. Each arrow lays one cell of wire from the pen.",
        "- Entering a node lights it. A lit node is a junction: Tab moves the pen to the next lit point so a new wire can branch from it.",
        "- Wires never cross or share a cell, except on a via (┼), where one wire may run straight across another.",
        "- Arrow nodes (△ ▽ ◁ ▷) only take wire arriving the way they point.",
        "- Each level has a wire budget.",
        "# Scoring",
        "100 points for each circuit lit, plus 10 for each cell of wire left in the budget.",
        "# Winning and losing",
        "Light every circuit to win. There is no way to lose: undo or restart a level as often as you like.",
        "# Keys",
        "- Arrows (or H J K L) lay wire.",
        "- Tab or Enter moves the pen to the next lit point.",
        "- U or Backspace undoes. R restarts the level.",
        "Tip: lay the long straight runs first and branch from the nodes they light.",
    ],
    input: Input::Keys,
    unit: "points",
};

/// Tiles a Trace level may use.
pub const TILES: &str = "#. SN+^v<>";

pub const BUILT_IN: &[(&str, &str)] = &[(
    "trace/levels.lua",
    include_str!("../../levels/trace/levels.lua"),
)];

const N: u8 = 1;
const E: u8 = 2;
const S: u8 = 4;
const W: u8 = 8;

fn dir_bits(dx: i32, dy: i32) -> (u8, u8) {
    // the side left by and the side entered by
    match (dx, dy) {
        (1, 0) => (E, W),
        (-1, 0) => (W, E),
        (0, 1) => (S, N),
        _ => (N, S),
    }
}

/// One step the player took, kept for undo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Step {
    Wire {
        from: (usize, usize),
        to: (usize, usize),
        lit: bool,
    },
    Pen {
        from: (usize, usize),
    },
}

pub struct Trace {
    levels: Vec<Level>,
    problems: Vec<String>,
    pub index: usize,
    /// Connection sides per cell.
    links: Vec<Vec<u8>>,
    lit: Vec<Vec<bool>>,
    pub pen: (usize, usize),
    steps: Vec<Step>,
    used: u32,
    score: u64,
    finished: bool,
    /// Seconds the solved board stays before the next level.
    pause: f32,
    reduced: bool,
}

impl Trace {
    pub fn new(setup: Setup) -> Self {
        let (levels, problems) = levels::load("trace", BUILT_IN, TILES);
        Self::with_levels(levels, problems, setup)
    }

    pub fn with_levels(levels: Vec<Level>, problems: Vec<String>, setup: Setup) -> Self {
        let mut t = Self {
            levels,
            problems,
            index: 0,
            links: Vec::new(),
            lit: Vec::new(),
            pen: (0, 0),
            steps: Vec::new(),
            used: 0,
            score: 0,
            finished: false,
            pause: 0.0,
            reduced: setup.reduced_motion,
        };
        t.start();
        t
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
        let (w, h) = (self.level().width(), self.level().height());
        self.links = vec![vec![0; w]; h];
        self.lit = vec![vec![false; w]; h];
        self.steps.clear();
        self.used = 0;
        self.pause = 0.0;
        for y in 0..h {
            for x in 0..w {
                if self.level().map[y][x] == 'S' {
                    self.pen = (x, y);
                    self.lit[y][x] = true;
                }
            }
        }
    }

    fn is_node(ch: char) -> bool {
        matches!(ch, 'N' | '^' | 'v' | '<' | '>')
    }

    pub fn nodes_left(&self) -> usize {
        let mut n = 0;
        for (y, row) in self.level().map.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                if Self::is_node(ch) && !self.lit[y][x] {
                    n += 1;
                }
            }
        }
        n
    }

    fn lit_points(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::new();
        for (y, row) in self.lit.iter().enumerate() {
            for (x, &on) in row.iter().enumerate() {
                if on {
                    points.push((x, y));
                }
            }
        }
        points
    }

    /// Lays wire one step. Returns why it cannot, if it cannot.
    pub fn step(&mut self, dx: i32, dy: i32, fx: &mut Fx) -> Result<(), &'static str> {
        let (w, h) = (self.level().width() as i32, self.level().height() as i32);
        let (nx, ny) = (self.pen.0 as i32 + dx, self.pen.1 as i32 + dy);
        if nx < 0 || ny < 0 || nx >= w || ny >= h {
            return Err("The edge of the board");
        }
        let to = (nx as usize, ny as usize);
        let ch = self.tile(to);
        let (out_side, in_side) = dir_bits(dx, dy);
        // a wire that runs through a via must go straight on
        if self.tile(self.pen) == '+' {
            let came = self.links[self.pen.1][self.pen.0];
            let horizontal = came & (E | W);
            let vertical = came & (N | S);
            let along = if dx != 0 { horizontal } else { vertical };
            if along == 0 || along == (E | W) || along == (N | S) {
                return Err("A wire crosses a via in a straight line");
            }
        }
        self.target_ok(to, dx, dy)?;
        if self.used >= self.level().budget {
            return Err("The wire budget is spent: U undoes");
        }
        self.links[self.pen.1][self.pen.0] |= out_side;
        self.links[to.1][to.0] |= in_side;
        let lit = Self::is_node(ch);
        if lit {
            self.lit[to.1][to.0] = true;
            fx.sound(Sfx::Good);
            let left = self.nodes_left();
            if left > 0 {
                fx.say(format!("Node lit, {left} to go"));
            }
        } else {
            fx.sound(Sfx::Tap);
        }
        self.steps.push(Step::Wire {
            from: self.pen,
            to,
            lit,
        });
        self.pen = to;
        self.used += 1;
        if self.nodes_left() == 0 {
            let spare = self.level().budget.saturating_sub(self.used);
            let gained = 100 + 10 * u64::from(spare);
            self.score += gained;
            fx.sound(Sfx::Win);
            fx.say(format!("{} solved, +{gained}", self.level().name));
            self.pause = if self.reduced { 0.8 } else { 1.5 };
            if self.index + 1 >= self.levels.len() {
                self.finished = true;
            }
        }
        Ok(())
    }

    /// Whether wire may enter `to` moving by `dx`, `dy`, judged on the tile and the wire there.
    fn target_ok(&self, to: (usize, usize), dx: i32, dy: i32) -> Result<(), &'static str> {
        let ch = self.tile(to);
        match ch {
            '#' | ' ' => Err("A wall"),
            'S' => Err("That is the source"),
            _ if self.lit[to.1][to.0] => Err("That node is already lit"),
            '^' if dy != -1 => Err("That node takes wire going up"),
            'v' if dy != 1 => Err("That node takes wire going down"),
            '<' if dx != -1 => Err("That node takes wire going left"),
            '>' if dx != 1 => Err("That node takes wire going right"),
            '+' => {
                let along = if dx != 0 { E | W } else { N | S };
                if self.links[to.1][to.0] & along != 0 {
                    Err("That via already carries a wire this way")
                } else {
                    Ok(())
                }
            }
            _ if self.links[to.1][to.0] != 0 => Err("Wires never cross"),
            _ => Ok(()),
        }
    }

    #[cfg(test)]
    /// The shortest wire from `from` to each unlit node it can reach, as lists of steps.
    fn routes(&self, from: (usize, usize)) -> Vec<Vec<(i32, i32)>> {
        use std::collections::{HashMap, VecDeque};
        let (w, h) = (self.level().width() as i32, self.level().height() as i32);
        type Visit = ((usize, usize), (i32, i32));
        let mut prev: HashMap<Visit, Visit> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut routes = Vec::new();
        queue.push_back((from, (0, 0)));
        prev.insert((from, (0, 0)), (from, (0, 0)));
        while let Some((pos, came)) = queue.pop_front() {
            let dirs: Vec<(i32, i32)> = if pos != from && self.tile(pos) == '+' {
                vec![came]
            } else {
                vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
            };
            for (dx, dy) in dirs {
                let (nx, ny) = (pos.0 as i32 + dx, pos.1 as i32 + dy);
                if nx < 0 || ny < 0 || nx >= w || ny >= h {
                    continue;
                }
                let to = (nx as usize, ny as usize);
                if self.target_ok(to, dx, dy).is_err() || prev.contains_key(&(to, (dx, dy))) {
                    continue;
                }
                // a plain cell is entered once whatever the direction; a via once per axis
                if self.tile(to) != '+' && prev.keys().any(|(p, _)| *p == to) {
                    continue;
                }
                prev.insert((to, (dx, dy)), (pos, came));
                if Self::is_node(self.tile(to)) {
                    let mut path = vec![(dx, dy)];
                    let mut at = (pos, came);
                    while at.0 != from || at.1 != (0, 0) {
                        path.push(at.1);
                        at = prev[&at];
                    }
                    path.reverse();
                    routes.push(path);
                } else {
                    queue.push_back((to, (dx, dy)));
                }
            }
        }
        routes
    }

    fn cycle_pen(&mut self) {
        let points = self.lit_points();
        if points.is_empty() {
            return;
        }
        let i = points.iter().position(|p| *p == self.pen);
        let next = points[i.map_or(0, |i| (i + 1) % points.len())];
        if next != self.pen {
            self.steps.push(Step::Pen { from: self.pen });
            self.pen = next;
        }
    }

    fn undo(&mut self) {
        match self.steps.pop() {
            Some(Step::Wire { from, to, lit }) => {
                let dx = to.0 as i32 - from.0 as i32;
                let dy = to.1 as i32 - from.1 as i32;
                let (out_side, in_side) = dir_bits(dx, dy);
                self.links[from.1][from.0] &= !out_side;
                self.links[to.1][to.0] &= !in_side;
                if lit {
                    self.lit[to.1][to.0] = false;
                }
                self.pen = from;
                self.used -= 1;
            }
            Some(Step::Pen { from }) => self.pen = from,
            None => {}
        }
    }

    fn solved(&self) -> bool {
        !self.levels.is_empty() && self.nodes_left() == 0
    }
}

/// The wire glyph for a set of sides.
fn wire(bits: u8) -> char {
    match bits {
        b if b == N | S => '┃',
        b if b == E | W => '━',
        b if b == N | E => '┗',
        b if b == N | W => '┛',
        b if b == S | E => '┏',
        b if b == S | W => '┓',
        b if b == N | S | E => '┣',
        b if b == N | S | W => '┫',
        b if b == E | W | N => '┻',
        b if b == E | W | S => '┳',
        b if b == N | S | E | W => '╋',
        b if b == N || b == S => '╹',
        _ => '╸',
    }
}

impl Game for Trace {
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
        let dir = match key {
            Key::Up | Key::Char('k' | 'K') => Some((0, -1)),
            Key::Down | Key::Char('j' | 'J') => Some((0, 1)),
            Key::Left | Key::Char('h' | 'H') => Some((-1, 0)),
            Key::Right | Key::Char('l' | 'L') => Some((1, 0)),
            _ => None,
        };
        if let Some((dx, dy)) = dir {
            if let Err(why) = self.step(dx, dy, fx) {
                fx.say(why);
                fx.sound(Sfx::Low);
            }
            return;
        }
        match key {
            Key::Tab | Key::Enter => self.cycle_pen(),
            Key::Backspace | Key::Char('u' | 'U') => self.undo(),
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
            let mut lines = vec!["No Trace levels could be loaded.".to_string()];
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
        c.titled(board, Line::Heavy, pal.green, &level.name, pal.bright);
        for y in ring as usize..(h - ring) as usize {
            for x in ring as usize..(w - ring) as usize {
                let cx = board.x + 1 + (x as i32 - ring) * 3;
                let cy = board.y + 1 + y as i32 - ring;
                let ch = level.map[y][x];
                let bits = self.links[y][x];
                let lit = self.lit[y][x];
                let pen = (x, y) == self.pen;
                match ch {
                    '#' => c.text(cx, cy, "▓▓▓", pal.muted),
                    ' ' => {}
                    'S' => {
                        c.put(cx, cy, if bits & W != 0 { '━' } else { ' ' }, pal.green);
                        c.put(cx + 1, cy, g('◆', 'S'), pal.yellow);
                        c.put(cx + 2, cy, if bits & E != 0 { '━' } else { ' ' }, pal.green);
                    }
                    _ if Self::is_node(ch) => {
                        let glyph = match (ch, lit) {
                            ('N', false) => g('○', 'o'),
                            ('N', true) => g('●', '@'),
                            ('^', false) => g('△', '^'),
                            ('^', true) => g('▲', '^'),
                            ('v', false) => g('▽', 'v'),
                            ('v', true) => g('▼', 'v'),
                            ('<', false) => g('◁', '<'),
                            ('<', true) => g('◀', '<'),
                            ('>', false) => g('▷', '>'),
                            _ => g('▶', '>'),
                        };
                        // a lit node is drawn filled and wired in; an unlit one hollow
                        let colour = if lit { pal.green } else { pal.text };
                        let left = if bits & W != 0 {
                            '━'
                        } else if lit {
                            '['
                        } else {
                            ' '
                        };
                        let right = if bits & E != 0 {
                            '━'
                        } else if lit {
                            ']'
                        } else {
                            ' '
                        };
                        c.put(cx, cy, left, pal.green);
                        c.put(cx + 1, cy, glyph, colour);
                        c.put(cx + 2, cy, right, pal.green);
                    }
                    '+' => {
                        let centre = if bits == 0 {
                            '┼'
                        } else {
                            wire(
                                bits | if bits & (E | W) != 0 { E | W } else { 0 }
                                    | if bits & (N | S) != 0 { N | S } else { 0 },
                            )
                        };
                        c.put(
                            cx,
                            cy,
                            if bits & W != 0 { '━' } else { '·' },
                            if bits & W != 0 { pal.green } else { pal.muted },
                        );
                        c.put(
                            cx + 1,
                            cy,
                            centre,
                            if bits == 0 { pal.cyan } else { pal.green },
                        );
                        c.put(
                            cx + 2,
                            cy,
                            if bits & E != 0 { '━' } else { '·' },
                            if bits & E != 0 { pal.green } else { pal.muted },
                        );
                    }
                    _ => {
                        if bits == 0 {
                            c.put(cx + 1, cy, '·', pal.muted);
                        } else {
                            c.put(cx, cy, if bits & W != 0 { '━' } else { ' ' }, pal.green);
                            c.put(cx + 1, cy, wire(bits), pal.green);
                            c.put(cx + 2, cy, if bits & E != 0 { '━' } else { ' ' }, pal.green);
                        }
                    }
                }
                if pen && self.pause <= 0.0 {
                    // the pen: brackets around its cell in the brightest colour
                    c.put(cx, cy, g('❰', '['), pal.bright);
                    c.put(cx + 2, cy, g('❱', ']'), pal.bright);
                }
            }
        }
        let p = Rect::new(board.x + board_w + 2, board.y, panel_w, board_h.max(14));
        c.titled(p, Line::Light, pal.muted, "Circuit", pal.bright);
        c.text(
            p.x + 2,
            p.y + 2,
            &format!("Level {} of {}", self.index + 1, self.levels.len()),
            pal.text,
        );
        let budget = level.budget.max(1);
        c.text(p.x + 2, p.y + 4, "Wire left", pal.muted);
        c.meter(
            p.x + 2,
            p.y + 5,
            12,
            1.0 - self.used as f32 / budget as f32,
            pal.green,
            pal.muted,
        );
        c.text(
            p.x + 15,
            p.y + 5,
            &format!("{}", budget.saturating_sub(self.used)),
            pal.text,
        );
        c.text(
            p.x + 2,
            p.y + 7,
            &format!("{} {} nodes to light", g('○', 'o'), self.nodes_left()),
            pal.text,
        );
        for (y, line) in (p.y + 9..).zip(wrap(&level.note, panel_w as usize - 4).iter().take(3)) {
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
            ("Wire".into(), self.used.to_string()),
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
                "Every one of the {} circuits is lit.",
                self.levels.len()
            ))
        })
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        // finish the first level, then branch part of the way through the second
        let mut keys = vec![(Key::Right, 0.0); 5];
        keys.push((Key::Right, 1.8));
        keys.extend([
            (Key::Right, 0.0),
            (Key::Right, 0.0),
            (Key::Right, 0.0),
            (Key::Right, 0.0),
            (Key::Tab, 0.0),
            (Key::Down, 0.0),
            (Key::Down, 0.0),
            (Key::Right, 0.0),
        ]);
        keys
    }
}

pub use crate::ui::wrap;

/// A search for a solution within the budget, used by the tests to prove every shipped level
/// can be solved. From every lit point it tries the shortest wire to each unlit node, with
/// backtracking, so a level it solves is certainly solvable.
#[cfg(test)]
pub fn solve(level: &Level) -> Option<u32> {
    let setup = Setup {
        seed: 0,
        reduced_motion: true,
        day: 0,
        practice: false,
    };
    let mut t = Trace::with_levels(vec![level.clone()], Vec::new(), setup);
    fn go(t: &mut Trace, seen: &mut std::collections::HashSet<Vec<Vec<u8>>>) -> bool {
        if t.nodes_left() == 0 {
            return true;
        }
        if !seen.insert(t.links.clone()) {
            return false;
        }
        for start in t.lit_points() {
            for route in t.routes(start) {
                let saved = t.pen;
                t.pen = start;
                let mut laid = 0;
                let mut ok = true;
                for &(dx, dy) in &route {
                    if t.step(dx, dy, &mut Fx::default()).is_err() {
                        ok = false;
                        break;
                    }
                    laid += 1;
                }
                if ok && go(t, seen) {
                    return true;
                }
                for _ in 0..laid {
                    t.undo();
                }
                t.pen = saved;
            }
        }
        false
    }
    let mut seen = std::collections::HashSet::new();
    go(&mut t, &mut seen).then_some(t.used)
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

    fn level(map: &[&str], budget: u32) -> Level {
        Level {
            name: "t".into(),
            note: String::new(),
            map: map.iter().map(|r| r.chars().collect()).collect(),
            par: 0,
            budget,
        }
    }

    #[test]
    fn the_built_in_levels_load_and_can_all_be_solved() {
        let (levels, problems) = levels::load("trace-test-none", BUILT_IN, TILES);
        assert!(problems.is_empty(), "{problems:?}");
        assert!(levels.len() >= 8);
        for l in &levels {
            assert!(l.budget > 0, "{}", l.name);
            let used = solve(l).unwrap_or_else(|| panic!("{} has no solution", l.name));
            assert!(used <= l.budget, "{}", l.name);
        }
    }

    #[test]
    fn wire_lights_a_node_and_cannot_cross() {
        let mut t = Trace::with_levels(vec![level(&["S.N", "..."], 10)], vec![], setup());
        let mut fx = Fx::default();
        t.step(1, 0, &mut fx).unwrap();
        t.step(1, 0, &mut fx).unwrap();
        assert_eq!(t.nodes_left(), 0);
        let mut t = Trace::with_levels(vec![level(&["S..", "N.N", "..."], 20)], vec![], setup());
        t.step(1, 0, &mut fx).unwrap();
        t.step(0, 1, &mut fx).unwrap();
        t.step(-1, 0, &mut fx).unwrap(); // reaches the left node
        t.step(0, 1, &mut fx).unwrap();
        t.step(1, 0, &mut fx).unwrap();
        assert_eq!(t.step(0, -1, &mut fx), Err("Wires never cross"));
    }

    #[test]
    fn a_via_lets_one_wire_cross_straight() {
        let mut t = Trace::with_levels(
            vec![level(&["#N#", "S+>", "#.#", "#.#"], 20)],
            vec![],
            setup(),
        );
        let mut fx = Fx::default();
        t.step(1, 0, &mut fx).unwrap();
        assert!(
            t.step(0, -1, &mut fx).is_err(),
            "a turn on a via is refused"
        );
        t.step(1, 0, &mut fx).unwrap();
        assert_eq!(t.nodes_left(), 1);
    }

    #[test]
    fn arrow_nodes_only_take_wire_their_way() {
        let mut t = Trace::with_levels(vec![level(&["S<"], 5)], vec![], setup());
        assert!(t.step(1, 0, &mut Fx::default()).is_err());
        let mut t = Trace::with_levels(vec![level(&["S>"], 5)], vec![], setup());
        assert!(t.step(1, 0, &mut Fx::default()).is_ok());
    }

    #[test]
    fn undo_takes_back_wire_and_light() {
        let mut t = Trace::with_levels(vec![level(&["SN.", "..N"], 9)], vec![], setup());
        let mut fx = Fx::default();
        t.step(1, 0, &mut fx).unwrap();
        assert_eq!(t.nodes_left(), 1);
        t.undo();
        assert_eq!(t.nodes_left(), 2);
        assert_eq!(t.used, 0);
        assert_eq!(t.pen, (0, 0));
    }

    #[test]
    fn the_budget_stops_extra_wire() {
        let mut t = Trace::with_levels(vec![level(&["S...N"], 2)], vec![], setup());
        let mut fx = Fx::default();
        t.step(1, 0, &mut fx).unwrap();
        t.step(1, 0, &mut fx).unwrap();
        assert!(t.step(1, 0, &mut fx).is_err());
    }
}
