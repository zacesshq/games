//! Through Ball: a turn-based football puzzle seen from above. Three attackers bring the ball
//! towards goal against three defenders and a keeper. Each turn the player moves the carrier one
//! cell, passes along a straight lane that no defender stands in or shoots at one of the goal's
//! three rows. Then the other two attackers make their runs, one defender presses the ball and
//! the others step into the passing lanes. A defender reaching the carrier wins the ball. The
//! keeper covers the row it stands on. From outside the box it covers the rows either side too,
//! so a shot has to be worked close. Five attacks, each on a shot clock of twelve turns.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, g};

pub const INFO: Info = Info {
    id: "through-ball",
    name: "Through Ball",
    icon: ('⊙', 'o'),
    hue: Hue::Keyword,
    blurb: "a turn-based football puzzle",
    hints: "Arrows run · Tab pick · Enter pass · 1 2 3 shoot",
    help: &[
        "# Goal",
        "Score as many goals as you can from five attacks.",
        "# Rules",
        "- Your three attackers are (1), (2) and (3); the one with the ball shows ●. Three defenders (✕) and a keeper ([K]) stand in the way.",
        "- Each turn you do one thing: run one cell with the ball, pass or shoot.",
        "- A pass goes to the teammate marked <2> (Tab picks another). It only goes if no defender stands in the straight lane between you; the lane is dotted when open.",
        "- Shoot with 1, 2 or 3 at the top, middle or low row of goal, from column 9 on. A defender in the way blocks it. The keeper covers its own row. From outside the box it covers the rows either side too.",
        "- After your turn your teammates run forward, the nearest defender presses the ball and the others step into the passing lanes. A defender reaching the ball wins it.",
        "- Each attack has a shot clock of 12 turns.",
        "# Scoring",
        "100 points for each goal plus 10 for each turn left on the clock.",
        "# Winning and losing",
        "The match ends after five attacks. An attack ends with a goal, a save, a blocked shot, a tackle or the clock running out.",
        "# Keys",
        "- Arrows (or H J K L) run with the ball.",
        "- Tab picks a teammate. Enter passes.",
        "- 1, 2 and 3 shoot.",
        "Tip: draw the keeper to one side, then shoot to the far row from inside the box.",
    ],
    input: Input::Keys,
    unit: "points",
};

pub const W: i32 = 15;
pub const H: i32 = 9;
/// The goal mouth: the rows a shot can go in, just past the right edge.
pub const GOAL: [i32; 3] = [3, 4, 5];
const ATTACKS: u32 = 5;
const CLOCK: u32 = 12;
/// The first column a shot can be taken from.
const SHOOT_FROM: i32 = 8;
/// The edge of the box: from here on the keeper only covers its own row.
const BOX: i32 = 11;

pub type Cell = (i32, i32);

/// The cells strictly between two cells on a straight line, by Bresenham's method.
pub fn between(a: Cell, b: Cell) -> Vec<Cell> {
    let (mut x, mut y) = a;
    let dx = (b.0 - a.0).abs();
    let dy = -(b.1 - a.1).abs();
    let sx = (b.0 - a.0).signum();
    let sy = (b.1 - a.1).signum();
    let mut err = dx + dy;
    let mut cells = Vec::new();
    loop {
        if (x, y) == b {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
        if (x, y) != b {
            cells.push((x, y));
        }
    }
    cells
}

/// One step from `from` towards `to` along the longer way. The other way when that is taken.
fn step_towards(from: Cell, to: Cell, free: impl Fn(Cell) -> bool) -> Option<Cell> {
    let dx = (to.0 - from.0).signum();
    let dy = (to.1 - from.1).signum();
    let horizontal = (from.0 + dx, from.1);
    let vertical = (from.0, from.1 + dy);
    let order = if (to.0 - from.0).abs() >= (to.1 - from.1).abs() {
        [horizontal, vertical]
    } else {
        [vertical, horizontal]
    };
    order.into_iter().filter(|c| *c != from).find(|c| free(*c))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Result {
    Goal,
    Saved,
    Blocked,
    Tackled,
    ClockOut,
}

pub struct ThroughBall {
    pub attackers: [Cell; 3],
    pub carrier: usize,
    pub target: usize,
    pub defenders: [Cell; 3],
    pub keeper: Cell,
    pub attack: u32,
    pub clock: u32,
    pub goals: u32,
    score: u64,
    rng: Rng,
    /// The result of the attack just ended and how long it stays before the next kick-off.
    pub ended: Option<(Result, f32)>,
    finished: bool,
    reduced: bool,
}

impl ThroughBall {
    pub fn new(setup: Setup) -> Self {
        let mut t = Self {
            attackers: [(0, 0); 3],
            carrier: 0,
            target: 1,
            defenders: [(0, 0); 3],
            keeper: (W - 1, 4),
            attack: 1,
            clock: CLOCK,
            goals: 0,
            score: 0,
            rng: Rng::new(setup.seed),
            ended: None,
            finished: false,
            reduced: setup.reduced_motion,
        };
        t.kick_off();
        t
    }

    /// Lines the sides up for a new attack, with the defenders spread a little differently each
    /// time.
    fn kick_off(&mut self) {
        let r = self.rng.below(2) as i32;
        self.attackers = [(2, 4), (1, 1 + r), (1, 7 - r)];
        self.carrier = 0;
        self.target = 1;
        let mut defenders = [(0, 0); 3];
        let rows = [
            1 + self.rng.below(2) as i32,
            3 + self.rng.below(3) as i32,
            6 + self.rng.below(2) as i32,
        ];
        for (i, d) in defenders.iter_mut().enumerate() {
            *d = (6 + self.rng.below(4) as i32, rows[i]);
        }
        self.defenders = defenders;
        self.keeper = (W - 1, 4);
        self.clock = CLOCK;
        self.ended = None;
    }

    fn occupied(&self, c: Cell) -> bool {
        self.attackers.contains(&c) || self.defenders.contains(&c) || self.keeper == c
    }

    fn inside(c: Cell) -> bool {
        (0..W).contains(&c.0) && (0..H).contains(&c.1)
    }

    pub fn ball(&self) -> Cell {
        self.attackers[self.carrier]
    }

    /// Whether a pass to the chosen teammate would get through.
    pub fn lane_open(&self, to: usize) -> bool {
        to != self.carrier
            && between(self.ball(), self.attackers[to])
                .iter()
                .all(|c| !self.defenders.contains(c) && *c != self.keeper)
    }

    /// The goal rows the keeper can reach from where it stands for a shot from column `x`.
    pub fn covered(&self, x: i32) -> Vec<i32> {
        let k = self.keeper.1;
        if x < BOX {
            GOAL.iter()
                .copied()
                .filter(|r| (r - k).abs() <= 1)
                .collect()
        } else {
            vec![k]
        }
    }

    fn end(&mut self, result: Result, fx: &mut Fx) {
        match result {
            Result::Goal => {
                let gained = 100 + 10 * u64::from(self.clock);
                self.goals += 1;
                self.score += gained;
                fx.sound(Sfx::Win);
                fx.say(format!("Goal, +{gained}"));
            }
            Result::Saved => {
                fx.sound(Sfx::Low);
                fx.say("Saved by the keeper");
            }
            Result::Blocked => {
                fx.sound(Sfx::Low);
                fx.say("The shot was blocked");
            }
            Result::Tackled => {
                fx.sound(Sfx::Low);
                fx.say("Tackled: the ball is lost");
            }
            Result::ClockOut => {
                fx.sound(Sfx::Low);
                fx.say("The shot clock ran out");
            }
        }
        let wait = if self.reduced { 1.2 } else { 1.8 };
        self.ended = Some((result, wait));
        if self.attack >= ATTACKS {
            self.finished = true;
        }
    }

    /// The other side's turn after the player's: runs, the press and the lanes, then the keeper.
    fn respond(&mut self, fx: &mut Fx) {
        // the two attackers without the ball run towards goal while the way is clear
        for i in 0..3 {
            if i == self.carrier {
                continue;
            }
            let (x, y) = self.attackers[i];
            let ahead = (x + 1, y);
            if x < BOX && Self::inside(ahead) && !self.occupied(ahead) {
                self.attackers[i] = ahead;
            }
        }
        let ball = self.ball();
        // the nearest defender presses the ball
        let mut order: Vec<usize> = (0..3).collect();
        order.sort_by_key(|&i| {
            let d = self.defenders[i];
            ((d.0 - ball.0).abs() + (d.1 - ball.1).abs(), i)
        });
        let presser = order[0];
        let press_free =
            |c: Cell, me: &Self| Self::inside(c) && (c == ball || !me.occupied(c)) && c.0 < W - 1;
        if let Some(next) = step_towards(self.defenders[presser], ball, |c| press_free(c, self)) {
            self.defenders[presser] = next;
            if next == ball {
                self.end(Result::Tackled, fx);
                return;
            }
        }
        // the others each step into the lane to the attacker nearest them
        let mates: Vec<usize> = (0..3).filter(|&i| i != self.carrier).collect();
        for (k, &d) in order.iter().skip(1).enumerate() {
            let mate = mates[k.min(mates.len() - 1)];
            let lane = between(ball, self.attackers[mate]);
            let spot = lane
                .get(lane.len() / 2)
                .copied()
                .unwrap_or(self.attackers[mate]);
            let free = |c: Cell| Self::inside(c) && !self.occupied(c) && c.0 < W - 1;
            if self.defenders[d] != spot
                && let Some(next) = step_towards(self.defenders[d], spot, free)
            {
                self.defenders[d] = next;
            }
        }
        // the keeper shuffles along the goal line towards the ball's row
        let want = ball.1.clamp(GOAL[0], GOAL[2]);
        let k = self.keeper;
        if k.1 != want {
            self.keeper = (k.0, k.1 + (want - k.1).signum());
        }
        self.clock -= 1;
        if self.clock == 0 {
            self.end(Result::ClockOut, fx);
        }
    }

    fn run(&mut self, dx: i32, dy: i32, fx: &mut Fx) {
        let (x, y) = self.ball();
        let to = (x + dx, y + dy);
        if !Self::inside(to) || to.0 >= W - 1 {
            fx.say("The edge of the pitch");
            return;
        }
        if self.occupied(to) {
            fx.say("Someone is in the way");
            return;
        }
        self.attackers[self.carrier] = to;
        fx.sound(Sfx::Tap);
        self.respond(fx);
    }

    fn pass(&mut self, fx: &mut Fx) {
        if self.target == self.carrier {
            self.cycle();
        }
        if !self.lane_open(self.target) {
            fx.say("That lane is blocked: pick another teammate or run");
            return;
        }
        self.carrier = self.target;
        self.cycle();
        fx.sound(Sfx::Good);
        fx.say(format!("Passed to {}", self.carrier + 1));
        self.respond(fx);
    }

    fn cycle(&mut self) {
        self.target = (self.target + 1) % 3;
        if self.target == self.carrier {
            self.target = (self.target + 1) % 3;
        }
    }

    pub fn shoot(&mut self, row: usize, fx: &mut Fx) {
        let ball = self.ball();
        if ball.0 < SHOOT_FROM {
            fx.say(format!("Too far out: shoot from column {}", SHOOT_FROM + 1));
            return;
        }
        let aim = (W, GOAL[row]);
        let blocked = between(ball, aim)
            .iter()
            .any(|c| self.defenders.contains(c));
        let result = if blocked {
            Result::Blocked
        } else if self.covered(ball.0).contains(&GOAL[row]) {
            Result::Saved
        } else {
            Result::Goal
        };
        self.end(result, fx);
    }
}

impl Game for ThroughBall {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn animated(&self) -> bool {
        self.ended.is_some()
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        if self.ended.is_some() {
            return;
        }
        match key {
            Key::Up | Key::Char('k' | 'K') => self.run(0, -1, fx),
            Key::Down | Key::Char('j' | 'J') => self.run(0, 1, fx),
            Key::Left | Key::Char('h' | 'H') => self.run(-1, 0, fx),
            Key::Right | Key::Char('l' | 'L') => self.run(1, 0, fx),
            Key::Tab => {
                self.cycle();
                fx.sound(Sfx::Tick);
            }
            Key::Enter => self.pass(fx),
            Key::Char('1') => self.shoot(0, fx),
            Key::Char('2') => self.shoot(1, fx),
            Key::Char('3') => self.shoot(2, fx),
            _ => {}
        }
    }

    fn tick(&mut self, dt: f32, _fx: &mut Fx) {
        if let Some((_, left)) = &mut self.ended {
            *left -= dt;
            if *left <= 0.0 && !self.finished {
                self.attack += 1;
                self.kick_off();
            }
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        let board_w = W * 3 + 2;
        let board_h = H + 2;
        // the side panel is left out when the window is too narrow for it; the status bar
        // carries what matters
        let panel_w = if area.w >= board_w + 3 + 22 { 22 } else { 0 };
        let r = area.centre(board_w + 3 + panel_w, board_h.max(15));
        let pitch = Rect::new(r.x, r.y, board_w, board_h);
        c.titled(
            pitch,
            Line::Heavy,
            pal.keyword,
            &format!("Attack {} of {ATTACKS}", self.attack),
            pal.bright,
        );
        let cx = |x: i32| pitch.x + 1 + x * 3;
        let cy = |y: i32| pitch.y + 1 + y;
        // the grass: a dot in every cell, with the box drawn as a dotted line
        for y in 0..H {
            for x in 0..W {
                c.put(cx(x) + 1, cy(y), '·', pal.muted);
            }
            c.put(cx(BOX) - 1, cy(y), '┊', pal.muted);
        }
        for x in BOX..W {
            c.text(cx(x), cy(0), "┄┄┄", pal.muted);
            c.text(cx(x), cy(H - 1), "┄┄┄", pal.muted);
        }
        // the goal: posts on the frame and the net just beyond it
        let gx = pitch.x + pitch.w - 1;
        c.put(gx, cy(GOAL[0]) - 1, '┳', pal.bright);
        c.put(gx, cy(GOAL[2]) + 1, '┻', pal.bright);
        for row in GOAL {
            c.put(gx, cy(row), ' ', pal.bright);
            c.put(gx + 1, cy(row), '▒', pal.muted);
        }
        // the lane of the chosen pass, open or blocked
        let open = self.lane_open(self.target);
        if self.ended.is_none() {
            for (x, y) in between(self.ball(), self.attackers[self.target]) {
                if !self.occupied((x, y)) {
                    let mark = if open { '•' } else { '·' };
                    c.put(
                        cx(x) + 1,
                        cy(y),
                        mark,
                        if open { pal.keyword } else { pal.muted },
                    );
                }
            }
        }
        for (i, &(x, y)) in self.defenders.iter().enumerate() {
            let _ = i;
            c.text(cx(x), cy(y), &format!(" {} ", g('✕', 'x')), pal.red);
        }
        let (kx, ky) = self.keeper;
        c.text(cx(kx), cy(ky), "[K]", pal.yellow);
        for (i, &(x, y)) in self.attackers.iter().enumerate() {
            let n = char::from(b'1' + i as u8);
            if i == self.carrier {
                c.put(cx(x), cy(y), '(', pal.bright);
                c.put(cx(x) + 1, cy(y), n, pal.bright);
                c.put(cx(x) + 2, cy(y), g('●', '*'), pal.bright);
            } else if i == self.target && self.ended.is_none() {
                c.text(cx(x), cy(y), &format!("<{n}>"), pal.keyword);
            } else {
                c.text(cx(x), cy(y), &format!("({n})"), pal.green);
            }
        }

        let p = Rect::new(pitch.x + board_w + 3, pitch.y, panel_w, board_h.max(15));
        if panel_w == 0 {
            // without the panel, one line under the pitch says what the panel would
            let open_text = if open { "open" } else { "blocked" };
            let line = format!(
                "Pass to {}: {open_text} · keeper {}",
                self.target + 1,
                if self.ball().0 >= BOX {
                    "own row"
                } else {
                    "three rows"
                }
            );
            c.text(
                pitch.x + 1,
                pitch.y + board_h,
                &line,
                if open { pal.green } else { pal.warning },
            );
            return;
        }
        c.titled(p, Line::Light, pal.muted, "Match", pal.bright);
        c.text(p.x + 2, p.y + 2, &format!("Goals {}", self.goals), pal.text);
        c.text(p.x + 2, p.y + 3, "Shot clock", pal.muted);
        c.meter(
            p.x + 2,
            p.y + 4,
            12,
            self.clock as f32 / CLOCK as f32,
            pal.keyword,
            pal.muted,
        );
        c.text(p.x + 15, p.y + 4, &self.clock.to_string(), pal.text);
        let lane = if self.ended.is_some() {
            String::new()
        } else if open {
            format!("Pass to {}: open", self.target + 1)
        } else {
            format!("Pass to {}: blocked", self.target + 1)
        };
        c.text(
            p.x + 2,
            p.y + 6,
            &lane,
            if open { pal.green } else { pal.warning },
        );
        let names = ["top", "mid", "low"];
        let covered: Vec<&str> = self
            .covered(self.ball().0)
            .iter()
            .filter_map(|r| GOAL.iter().position(|g| g == r).map(|i| names[i]))
            .collect();
        c.text(p.x + 2, p.y + 7, "Keeper covers", pal.muted);
        c.text(p.x + 2, p.y + 8, &covered.join(" "), pal.yellow);
        if self.ball().0 >= SHOOT_FROM {
            c.text(p.x + 2, p.y + 9, "In range: 1 2 3", pal.green);
        } else {
            c.text(p.x + 2, p.y + 9, "Out of range", pal.muted);
        }
        if let Some((result, _)) = &self.ended {
            let text = match result {
                Result::Goal => "GOAL",
                Result::Saved => "Saved",
                Result::Blocked => "Blocked",
                Result::Tackled => "Tackled",
                Result::ClockOut => "Clock out",
            };
            c.text(
                p.x + 2,
                p.y + 11,
                &format!("{} {text}", g('◆', '*')),
                pal.bright,
            );
        }
        c.text(
            p.x + 2,
            p.y + 13,
            &format!("{} ball  {} defender", g('●', '*'), g('✕', 'x')),
            pal.muted,
        );
    }

    fn status(&self) -> Status {
        vec![
            ("Score".into(), self.score.to_string()),
            ("Goals".into(), self.goals.to_string()),
            ("Attack".into(), format!("{}/{ATTACKS}", self.attack)),
            ("Clock".into(), self.clock.to_string()),
        ]
    }

    fn score(&self) -> u64 {
        self.score
    }

    fn outcome(&self) -> Option<Outcome> {
        let done = self.finished && self.ended.as_ref().is_none_or(|(_, t)| *t <= 0.0);
        done.then(|| Outcome::Won(format!("{} goals from {ATTACKS} attacks.", self.goals)))
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        vec![(Key::Right, 0.0), (Key::Right, 0.0), (Key::Tab, 0.0)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game(seed: u64) -> ThroughBall {
        ThroughBall::new(Setup {
            seed,
            reduced_motion: false,
            day: 0,
            practice: false,
        })
    }

    #[test]
    fn lanes_run_between_two_cells() {
        assert_eq!(between((0, 0), (3, 0)), vec![(1, 0), (2, 0)]);
        assert_eq!(between((0, 0), (2, 2)), vec![(1, 1)]);
        assert!(between((1, 1), (2, 1)).is_empty());
    }

    #[test]
    fn a_defender_in_the_lane_blocks_the_pass() {
        let mut t = game(1);
        t.attackers = [(2, 4), (6, 4), (2, 8)];
        t.defenders = [(4, 4), (10, 0), (10, 8)];
        assert!(!t.lane_open(1));
        t.defenders[0] = (4, 2);
        assert!(t.lane_open(1));
        let mut fx = Fx::default();
        t.target = 1;
        t.pass(&mut fx);
        assert_eq!(t.carrier, 1);
        assert_eq!(t.clock, CLOCK - 1);
    }

    #[test]
    fn the_keeper_covers_more_from_distance() {
        let mut t = game(2);
        t.keeper = (W - 1, 4);
        assert_eq!(t.covered(9), vec![3, 4, 5]);
        assert_eq!(t.covered(12), vec![4]);
    }

    #[test]
    fn a_close_shot_away_from_the_keeper_scores() {
        let mut t = game(3);
        t.attackers = [(12, 3), (1, 1), (1, 7)];
        t.carrier = 0;
        t.defenders = [(5, 1), (5, 4), (5, 7)];
        t.keeper = (W - 1, 5);
        let mut fx = Fx::default();
        t.shoot(0, &mut fx);
        assert_eq!(t.ended.as_ref().map(|e| e.0.clone()), Some(Result::Goal));
        assert_eq!(t.goals, 1);
        let mut t = game(3);
        t.attackers = [(9, 4), (1, 1), (1, 7)];
        t.defenders = [(5, 1), (5, 4), (5, 7)];
        t.keeper = (W - 1, 4);
        t.shoot(0, &mut fx);
        assert_eq!(t.ended.as_ref().map(|e| e.0.clone()), Some(Result::Saved));
    }

    #[test]
    fn a_pressing_defender_can_win_the_ball() {
        let mut t = game(4);
        t.attackers = [(5, 4), (1, 1), (1, 7)];
        t.defenders = [(7, 4), (12, 0), (12, 8)];
        let mut fx = Fx::default();
        t.run(1, 0, &mut fx);
        assert_eq!(t.ended.as_ref().map(|e| e.0.clone()), Some(Result::Tackled));
    }

    #[test]
    fn five_attacks_make_a_match() {
        let mut t = game(5);
        let mut fx = Fx::default();
        for _ in 0..ATTACKS {
            t.end(Result::Saved, &mut fx);
            assert!(t.ended.is_some());
            t.tick(5.0, &mut fx);
        }
        assert!(matches!(t.outcome(), Some(Outcome::Won(_))));
    }
}
