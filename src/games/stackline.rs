//! Stackline: pieces made of circuit parts fall into two wells side by side. Each part carries
//! wire one way: across (`━`), up and down (`┃`) or both (`╋`). Turning a piece turns its parts
//! too. A row clears when it is full and every part carries wire across, joining the two rails.
//! Each clear sends a feedback row into the bottom of the other well: wire all the way across but
//! for one break. Tab moves the falling piece to the other well. A well that overflows ends it.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, g};

pub const INFO: Info = Info {
    id: "stackline",
    name: "Stackline",
    icon: ('╋', '+'),
    hue: Hue::Cyan,
    blurb: "join rails across two wells",
    hints: "← → move · ↑ turn · ↓ step · Enter drop · Tab other well",
    help: &[
        "# Goal",
        "Keep both wells clear for as long as possible by joining rows of wire from rail to rail.",
        "# Rules",
        "- Pieces of one to three circuit parts fall into one of two wells. Each part carries wire across (━), up and down (┃) or both (╋).",
        "- Turning a piece turns it and its parts: an across part becomes upright and back.",
        "- A full row clears only when every part in it carries wire across, joining the + rail on the left to the - rail on the right. A full row that will not clear is marked ✕ at both ends.",
        "- Each clear pushes a feedback row (───) into the bottom of the other well. It has one break (×) that a part carrying wire across can fill.",
        "- Tab moves the falling piece to the other well, if it fits there.",
        "# Scoring",
        "100 points times the square of the rows cleared at once, times the chain: clears on pieces in a row raise the chain by one each. The pieces fall a little faster as rows clear.",
        "# Winning and losing",
        "There is no end to win. The game ends when a well overflows at the top.",
        "# Keys",
        "- ← → (or H L) move the piece.",
        "- ↑ (or K or X) turns it.",
        "- ↓ (or J) moves it down one row. Enter drops it.",
        "- Tab moves it to the other well.",
        "Tip: keep upright parts together in columns you do not need to clear soon.",
    ],
    input: Input::Keys,
    unit: "points",
};

pub const W: usize = 6;
pub const H: usize = 13;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Across,
    Upright,
    Cross,
    /// A part of a feedback row, wire across.
    Feedback,
}

impl Part {
    pub fn across(self) -> bool {
        matches!(self, Part::Across | Part::Cross | Part::Feedback)
    }

    fn turned(self) -> Part {
        match self {
            Part::Across => Part::Upright,
            Part::Upright => Part::Across,
            other => other,
        }
    }
}

pub type Well = [[Option<Part>; W]; H];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    /// Cells relative to the pivot, with their parts.
    pub cells: Vec<(i32, i32, Part)>,
    pub x: i32,
    pub y: i32,
    pub well: usize,
}

impl Piece {
    fn at(&self) -> impl Iterator<Item = (i32, i32, Part)> + '_ {
        self.cells
            .iter()
            .map(|&(dx, dy, p)| (self.x + dx, self.y + dy, p))
    }

    fn turned(&self) -> Piece {
        // a quarter turn clockwise about the pivot; each part turns with it
        Piece {
            cells: self
                .cells
                .iter()
                .map(|&(dx, dy, p)| (-dy, dx, p.turned()))
                .collect(),
            ..self.clone()
        }
    }
}

pub struct Stackline {
    pub wells: [Well; 2],
    pub piece: Option<Piece>,
    next: Vec<(i32, i32, Part)>,
    rng: Rng,
    fall: f32,
    clock: f32,
    score: u64,
    rows: u32,
    chain: u32,
    over: bool,
    /// Rows waiting to clear: well, rows and time left, so the clear is seen before it goes.
    clearing: Option<(usize, Vec<usize>, f32)>,
    reduced: bool,
}

fn shapes() -> [Vec<(i32, i32)>; 4] {
    [
        vec![(0, 0), (1, 0)],
        vec![(-1, 0), (0, 0), (1, 0)],
        vec![(0, 0), (1, 0), (0, 1)],
        vec![(0, 0)],
    ]
}

impl Stackline {
    pub fn new(setup: Setup) -> Self {
        let mut rng = Rng::new(setup.seed);
        let next = Self::make(&mut rng);
        let mut s = Self {
            wells: [[[None; W]; H]; 2],
            piece: None,
            next,
            rng,
            fall: if setup.reduced_motion { 1.6 } else { 0.9 },
            clock: 0.0,
            score: 0,
            rows: 0,
            chain: 0,
            over: false,
            clearing: None,
            reduced: setup.reduced_motion,
        };
        s.spawn(0);
        s
    }

    fn make(rng: &mut Rng) -> Vec<(i32, i32, Part)> {
        let all = shapes();
        // the single part is rarer, so most turns are about shape as well as wire
        let shape = if rng.chance(0.15) {
            &all[3]
        } else {
            &all[rng.below(3)]
        };
        shape
            .iter()
            .map(|&(x, y)| {
                let r = rng.unit();
                let part = if r < 0.12 {
                    Part::Cross
                } else if r < 0.6 {
                    Part::Across
                } else {
                    Part::Upright
                };
                (x, y, part)
            })
            .collect()
    }

    fn spawn(&mut self, well: usize) {
        let cells = std::mem::replace(&mut self.next, Self::make(&mut self.rng));
        let piece = Piece {
            cells,
            x: W as i32 / 2 - 1,
            y: 0,
            well,
        };
        if self.fits(&piece) {
            self.piece = Some(piece);
        } else {
            self.piece = None;
            self.over = true;
        }
    }

    fn fits(&self, p: &Piece) -> bool {
        p.at().all(|(x, y, _)| {
            (0..W as i32).contains(&x)
                && y < H as i32
                && (y < 0 || self.wells[p.well][y as usize][x as usize].is_none())
        })
    }

    fn try_move(&mut self, f: impl Fn(&Piece) -> Piece) -> bool {
        let Some(p) = &self.piece else { return false };
        let moved = f(p);
        if self.fits(&moved) {
            self.piece = Some(moved);
            true
        } else {
            false
        }
    }

    /// Whether a row carries wire from rail to rail.
    pub fn joined(row: &[Option<Part>; W]) -> bool {
        row.iter().all(|c| c.is_some_and(Part::across))
    }

    fn lock(&mut self, fx: &mut Fx) {
        let Some(p) = self.piece.take() else { return };
        for (x, y, part) in p.at() {
            if y < 0 {
                self.over = true;
                return;
            }
            self.wells[p.well][y as usize][x as usize] = Some(part);
        }
        let rows: Vec<usize> = (0..H)
            .filter(|&y| Self::joined(&self.wells[p.well][y]))
            .collect();
        if rows.is_empty() {
            self.chain = 0;
            fx.sound(Sfx::Tap);
            self.spawn(p.well);
        } else {
            self.chain += 1;
            fx.sound(Sfx::Good);
            fx.say(if self.chain > 1 {
                format!("{} joined, chain {}", rows.len(), self.chain)
            } else {
                format!("{} joined", rows.len())
            });
            let wait = if self.reduced { 0.0 } else { 0.35 };
            self.clearing = Some((p.well, rows, wait));
            if wait == 0.0 {
                self.finish_clear(fx);
            }
        }
    }

    fn finish_clear(&mut self, fx: &mut Fx) {
        let Some((well, rows, _)) = self.clearing.take() else {
            return;
        };
        for &y in &rows {
            for row in (1..=y).rev() {
                self.wells[well][row] = self.wells[well][row - 1];
            }
            self.wells[well][0] = [None; W];
        }
        let n = rows.len() as u64;
        self.rows += rows.len() as u32;
        self.score += 100 * n * n * u64::from(self.chain.max(1));
        // each clear pushes feedback into the other well, which may overflow it
        let other = 1 - well;
        for _ in 0..rows.len() {
            if self.wells[other][0].iter().any(Option::is_some) {
                self.over = true;
            }
            for y in 0..H - 1 {
                self.wells[other][y] = self.wells[other][y + 1];
            }
            let gap = self.rng.below(W);
            let mut row = [Some(Part::Feedback); W];
            row[gap] = None;
            self.wells[other][H - 1] = row;
        }
        // the falling time shortens gently as rows clear, never below half a second
        let base = if self.reduced { 1.6 } else { 0.9 };
        self.fall = (base - self.rows as f32 * 0.02).max(base * 0.55);
        if self.over {
            fx.sound(Sfx::Low);
            return;
        }
        self.spawn(well);
    }

    fn drop_piece(&mut self, fx: &mut Fx) {
        while self.try_move(|p| Piece {
            y: p.y + 1,
            ..p.clone()
        }) {}
        self.lock(fx);
    }
}

impl Game for Stackline {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        if self.over || self.clearing.is_some() {
            return;
        }
        if matches!(
            key,
            Key::Left
                | Key::Right
                | Key::Up
                | Key::Down
                | Key::Char('h' | 'H' | 'l' | 'L' | 'k' | 'K' | 'j' | 'J' | 'x' | 'X')
        ) {
            fx.sound(Sfx::Tick);
        }
        match key {
            Key::Left | Key::Char('h' | 'H') => {
                self.try_move(|p| Piece {
                    x: p.x - 1,
                    ..p.clone()
                });
            }
            Key::Right | Key::Char('l' | 'L') => {
                self.try_move(|p| Piece {
                    x: p.x + 1,
                    ..p.clone()
                });
            }
            Key::Up | Key::Char('k' | 'K' | 'x' | 'X') => {
                if !self.try_move(Piece::turned) {
                    // a turn against a wall tries one step in from either side
                    let ok = self.try_move(|p| Piece {
                        x: p.x - 1,
                        ..p.turned()
                    }) || self.try_move(|p| Piece {
                        x: p.x + 1,
                        ..p.turned()
                    });
                    if !ok {
                        fx.say("No room to turn");
                    }
                }
            }
            Key::Down | Key::Char('j' | 'J') => {
                if !self.try_move(|p| Piece {
                    y: p.y + 1,
                    ..p.clone()
                }) {
                    self.lock(fx);
                }
                self.clock = 0.0;
            }
            Key::Enter => self.drop_piece(fx),
            Key::Tab => {
                let moved = self.try_move(|p| Piece {
                    well: 1 - p.well,
                    ..p.clone()
                });
                if moved {
                    fx.sound(Sfx::Tap);
                } else {
                    fx.say("The other well has no room there");
                }
            }
            _ => {}
        }
    }

    fn tick(&mut self, dt: f32, fx: &mut Fx) {
        if self.over {
            return;
        }
        if let Some((_, _, left)) = &mut self.clearing {
            *left -= dt;
            if *left <= 0.0 {
                self.finish_clear(fx);
            }
            return;
        }
        self.clock += dt;
        if self.clock >= self.fall {
            self.clock = 0.0;
            if !self.try_move(|p| Piece {
                y: p.y + 1,
                ..p.clone()
            }) {
                self.lock(fx);
            }
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        let cell = 3;
        let well_w = W as i32 * cell + 2;
        let well_h = H as i32 + 2;
        let gap = 3;
        // the side panel is left out when the window is too narrow for it; the status bar
        // carries what matters
        let panel_w = if area.w >= well_w * 2 + gap + 2 + 18 {
            18
        } else {
            0
        };
        let total = well_w * 2 + gap + 2 + panel_w;
        let r = area.centre(total, well_h);
        let active = self.piece.as_ref().map(|p| p.well);
        for well in 0..2 {
            let wx = r.x + well as i32 * (well_w + gap);
            let frame = Rect::new(wx, r.y, well_w, well_h);
            let colour = if active == Some(well) {
                pal.cyan
            } else {
                pal.muted
            };
            let title = if well == 0 { "Well A" } else { "Well B" };
            c.titled(
                frame,
                if active == Some(well) {
                    Line::Heavy
                } else {
                    Line::Light
                },
                colour,
                title,
                if active == Some(well) {
                    pal.bright
                } else {
                    pal.muted
                },
            );
            let clearing: &[usize] = match &self.clearing {
                Some((w, rows, _)) if *w == well => rows,
                _ => &[],
            };
            for y in 0..H {
                let yy = r.y + 1 + y as i32;
                // the rails: + on the left, - on the right
                c.put(wx, yy, if y % 2 == 0 { '+' } else { '┃' }, pal.red);
                c.put(
                    wx + well_w - 1,
                    yy,
                    if y % 2 == 0 { '-' } else { '┃' },
                    pal.blue,
                );
                let full = self.wells[well][y].iter().all(Option::is_some);
                for x in 0..W {
                    let cx = wx + 1 + x as i32 * cell;
                    if clearing.contains(&y) {
                        c.text(cx, yy, "═══", pal.bright);
                        continue;
                    }
                    match self.wells[well][y][x] {
                        Some(p) => draw_part(c, cx, yy, p, false),
                        None if self.wells[well][y].contains(&Some(Part::Feedback)) => {
                            c.text(cx, yy, &format!(" {} ", g('×', 'x')), pal.warning);
                        }
                        None => c.put(cx + 1, yy, '·', pal.muted),
                    }
                }
                if full && !Self::joined(&self.wells[well][y]) && !clearing.contains(&y) {
                    c.put(wx, yy, g('✕', 'x'), pal.warning);
                    c.put(wx + well_w - 1, yy, g('✕', 'x'), pal.warning);
                }
                if clearing.contains(&y) {
                    c.put(wx, yy, g('◆', '*'), pal.bright);
                    c.put(wx + well_w - 1, yy, g('◆', '*'), pal.bright);
                }
            }
            if let Some(p) = &self.piece
                && p.well == well
            {
                // where the piece will land, drawn faintly so the drop can be planned
                let mut ghost = p.clone();
                while self.fits(&Piece {
                    y: ghost.y + 1,
                    ..ghost.clone()
                }) {
                    ghost.y += 1;
                }
                for (x, y, _) in ghost.at() {
                    if y >= 0 && ghost.y != p.y {
                        let cx = wx + 1 + x * cell;
                        c.text(cx, r.y + 1 + y, "┈┈┈", pal.muted);
                    }
                }
                for (x, y, part) in p.at() {
                    if y >= 0 {
                        draw_part(c, wx + 1 + x * cell, r.y + 1 + y, part, true);
                    }
                }
            }
        }
        let p = Rect::new(r.x + well_w * 2 + gap + 2, r.y, panel_w, well_h);
        if panel_w == 0 {
            return;
        }
        c.titled(p, Line::Light, pal.muted, "Next", pal.bright);
        for &(dx, dy, part) in &self.next {
            draw_part(c, p.x + 5 + dx * 3, p.y + 2 + dy, part, false);
        }
        c.text(p.x + 2, p.y + 5, "Rows", pal.muted);
        c.text(p.x + 8, p.y + 5, &self.rows.to_string(), pal.text);
        c.text(p.x + 2, p.y + 6, "Chain", pal.muted);
        c.text(p.x + 8, p.y + 6, &self.chain.to_string(), pal.text);
        c.text(p.x + 2, p.y + 8, "Parts", pal.muted);
        draw_part(c, p.x + 2, p.y + 9, Part::Across, false);
        c.text(p.x + 6, p.y + 9, "across", pal.text);
        draw_part(c, p.x + 2, p.y + 10, Part::Upright, false);
        c.text(p.x + 6, p.y + 10, "upright", pal.text);
        draw_part(c, p.x + 2, p.y + 11, Part::Cross, false);
        c.text(p.x + 6, p.y + 11, "both", pal.text);
        draw_part(c, p.x + 2, p.y + 12, Part::Feedback, false);
        c.text(p.x + 6, p.y + 12, "feedback", pal.text);
    }

    fn status(&self) -> Status {
        vec![
            ("Score".into(), self.score.to_string()),
            ("Rows".into(), self.rows.to_string()),
            (
                "Next".into(),
                self.next
                    .iter()
                    .map(|&(_, _, p)| match p {
                        Part::Across => '━',
                        Part::Upright => '┃',
                        Part::Cross => '╋',
                        Part::Feedback => '─',
                    })
                    .collect(),
            ),
            ("Speed".into(), format!("{:.1}s", self.fall)),
        ]
    }

    fn score(&self) -> u64 {
        self.score
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
            .then(|| Outcome::Lost(format!("A well overflowed after {} rows.", self.rows)))
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        vec![
            (Key::Left, 0.0),
            (Key::Left, 0.0),
            (Key::Enter, 0.4),
            (Key::Tab, 0.0),
            (Key::Right, 0.0),
            (Key::Enter, 0.4),
            (Key::Up, 0.0),
            (Key::Right, 0.0),
            (Key::Right, 0.0),
            (Key::Enter, 0.4),
            (Key::Left, 0.6),
        ]
    }
}

/// A part three cells wide: its wire shape, in a colour of its own.
fn draw_part(c: &mut Canvas, x: i32, y: i32, part: Part, falling: bool) {
    let pal = c.pal;
    let (text, colour) = match part {
        Part::Across => ("━━━", pal.cyan),
        Part::Upright => (" ┃ ", pal.magenta),
        Part::Cross => ("━╋━", pal.yellow),
        Part::Feedback => ("───", pal.warning),
    };
    c.text(x, y, text, if falling { pal.bright } else { colour });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> Stackline {
        Stackline::new(Setup {
            seed: 11,
            reduced_motion: true,
            day: 0,
            practice: false,
        })
    }

    #[test]
    fn a_row_of_across_parts_joins_and_an_upright_breaks_it() {
        let mut row = [Some(Part::Across); W];
        assert!(Stackline::joined(&row));
        row[2] = Some(Part::Cross);
        assert!(Stackline::joined(&row));
        row[3] = Some(Part::Upright);
        assert!(!Stackline::joined(&row));
        row[3] = None;
        assert!(!Stackline::joined(&row));
    }

    #[test]
    fn turning_a_piece_turns_its_parts() {
        let p = Piece {
            cells: vec![(0, 0, Part::Across), (1, 0, Part::Upright)],
            x: 2,
            y: 2,
            well: 0,
        };
        let t = p.turned();
        assert_eq!(t.cells, vec![(0, 0, Part::Upright), (0, 1, Part::Across)]);
        assert_eq!(t.turned().turned().turned(), p);
    }

    #[test]
    fn a_clear_pushes_feedback_into_the_other_well() {
        let mut s = game();
        for x in 0..W - 1 {
            s.wells[0][H - 1][x] = Some(Part::Across);
        }
        s.piece = Some(Piece {
            cells: vec![(0, 0, Part::Across)],
            x: W as i32 - 1,
            y: 0,
            well: 0,
        });
        let mut fx = Fx::default();
        s.drop_piece(&mut fx);
        assert_eq!(s.rows, 1);
        assert!(s.wells[0][H - 1].iter().all(Option::is_none));
        let feedback = s.wells[1][H - 1];
        assert_eq!(feedback.iter().filter(|c| c.is_none()).count(), 1);
        assert!(feedback.iter().flatten().all(|p| *p == Part::Feedback));
        assert!(s.score >= 100);
    }

    #[test]
    fn tab_moves_the_piece_across() {
        let mut s = game();
        let mut fx = Fx::default();
        let before = s.piece.as_ref().unwrap().well;
        s.key(Key::Tab, &mut fx);
        assert_eq!(s.piece.as_ref().unwrap().well, 1 - before);
    }

    #[test]
    fn a_full_well_ends_the_game() {
        let mut s = game();
        let mut fx = Fx::default();
        for _ in 0..200 {
            if s.outcome().is_some() {
                break;
            }
            s.key(Key::Enter, &mut fx);
        }
        assert!(matches!(s.outcome(), Some(Outcome::Lost(_))));
    }

    #[test]
    fn gravity_moves_the_piece_down_in_time() {
        let mut s = Stackline::new(Setup {
            seed: 2,
            reduced_motion: false,
            day: 0,
            practice: false,
        });
        let y = s.piece.as_ref().unwrap().y;
        let mut fx = Fx::default();
        s.tick(0.5, &mut fx);
        assert_eq!(s.piece.as_ref().unwrap().y, y);
        s.tick(0.5, &mut fx);
        assert_eq!(s.piece.as_ref().unwrap().y, y + 1);
    }
}
