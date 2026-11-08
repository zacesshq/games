//! Stackline: pieces made of circuit parts fall into two wells side by side. Each part carries
//! wire one way: across (`━`), up and down (`┃`) or both (`╋`). Turning a piece turns its parts
//! too. A row clears when it is full and every part carries wire across, joining the two rails.
//! Each clear sends a feedback row into the bottom of the other well: wire all the way across but
//! for one break. Tab moves the falling piece to the other well. A well that overflows ends it.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::pixels_ui::{Field, Ink};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Palette, Rect, Status};

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
        "- Pieces of one to three circuit parts fall into one of two wells. Each part is a chip whose copper carries wire across, up and down or both, so its shape shows which.",
        "- Turning a piece turns it and its parts: an across part becomes upright and back.",
        "- A full row clears only when every part in it carries wire across, joining the + rail on the left to the - rail on the right. A full row that will not clear is marked with a cross on both rails.",
        "- Each clear pushes a feedback row of wire into the bottom of the other well. It has one break, marked with a small cross, that a part carrying wire across can fill.",
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

    /// Whether the part carries wire up and down.
    pub fn upright(self) -> bool {
        matches!(self, Part::Upright | Part::Cross)
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

    fn zoomable(&self) -> bool {
        // the wells grow their own tiles with the window, which keeps the copper sharp
        false
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
        // each part is a tile that grows with the window: as tall as the rows allow and a
        // little wider than tall, with room left for the side panel
        let size = Size::fit(area);
        let (tile_w, tile_h, rail) = (size.w, size.h, size.rail);
        let rows = (H as i32 * tile_h + 1) / 2;
        let inner_w = W as i32 * tile_w + 2 * rail;
        let well_w = inner_w + 2;
        let well_h = rows + 2;
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
            let on = active == Some(well);
            let title = if well == 0 { "Well A" } else { "Well B" };
            c.titled(
                frame,
                if on { Line::Heavy } else { Line::Light },
                if on { pal.cyan } else { pal.muted },
                title,
                if on { pal.bright } else { pal.muted },
            );
            let inner = Rect::new(wx + 1, r.y + 1, inner_w, rows);
            self.paint_well(c, inner, well, size);
        }
        let p = Rect::new(r.x + well_w * 2 + gap + 2, r.y, panel_w, well_h);
        if panel_w == 0 {
            return;
        }
        c.titled(p, Line::Light, pal.muted, "Next", pal.bright);
        let mut next = Field::new(
            pal,
            Rect::new(p.x + 2, p.y + 2, 12, 2),
            pal.theme.background,
        );
        for &(dx, dy, part) in &self.next {
            tile(
                &mut next,
                &Ink::new(pal),
                &pal,
                3 + dx * 3,
                dy * 2,
                3,
                2,
                part,
                Look::Placed,
            );
        }
        next.paint(c);
        c.text(p.x + 2, p.y + 5, "Rows", pal.muted);
        c.text(p.x + 8, p.y + 5, &self.rows.to_string(), pal.text);
        c.text(p.x + 2, p.y + 6, "Chain", pal.muted);
        c.text(p.x + 8, p.y + 6, &self.chain.to_string(), pal.text);
        c.text(p.x + 2, p.y + 8, "Parts", pal.muted);
        let legend = [
            (Part::Across, "across"),
            (Part::Upright, "upright"),
            (Part::Cross, "both"),
            (Part::Feedback, "feedback"),
        ];
        for (i, (part, name)) in legend.iter().enumerate() {
            let y = p.y + 9 + i as i32;
            let mut key = Field::new(pal, Rect::new(p.x + 2, y, 3, 1), pal.theme.background);
            tile(
                &mut key,
                &Ink::new(pal),
                &pal,
                0,
                0,
                3,
                2,
                *part,
                Look::Placed,
            );
            key.paint(c);
            c.text(p.x + 6, y, name, pal.text);
        }
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
/// The size of a part's tile in pixels and of the rail each side of a well.
#[derive(Clone, Copy, Debug)]
struct Size {
    w: i32,
    h: i32,
    rail: i32,
}

impl Size {
    fn fit(area: Rect) -> Self {
        let h = ((area.h - 2) * 2 / H as i32).clamp(2, 7);
        // two wells, the gap between them and the panel share the width
        let fits = |w: i32| {
            let rail = (w / 2 + 1).max(3);
            2 * (W as i32 * w + 2 * rail + 2) + 3 + 2 + 18 <= area.w
        };
        let w = (3..=h + 3).rev().find(|&w| fits(w)).unwrap_or(3);
        Self {
            w,
            h,
            rail: (w / 2 + 1).max(3),
        }
    }
}

impl Stackline {
    /// One well as pixel art: a circuit board with a solder pad where each part can go, the +
    /// rail on the left and the - rail on the right, the parts as chips whose copper runs the
    /// way they carry wire and the falling piece with a faint copy where it will land.
    fn paint_well(&self, c: &mut Canvas, inner: Rect, well: usize, size: Size) {
        let pal = c.pal;
        let ink = Ink::new(pal);
        let (tw, th, rail) = (size.w, size.h, size.rail);
        let board = ink.tint(pal.green, 0.1);
        let pad = ink.tint(pal.green, 0.4);
        let mut layer = Field::new(pal, inner, board);
        let lh = layer.h();
        let tx = |x: i32| rail + x * tw;
        let ty = |y: i32| y * th;
        // the rails, a copper strip each side in their own colours
        let plus = ink.solid(pal.red);
        let minus = ink.solid(pal.blue);
        let right = rail + W as i32 * tw;
        let strip = if rail >= 5 { 2 } else { 1 };
        layer.px.fill_rect((rail - strip) / 2, 0, strip, lh, plus);
        layer
            .px
            .fill_rect(right + (rail - strip) / 2, 0, strip, lh, minus);
        let clearing: &[usize] = match &self.clearing {
            Some((w, rows, _)) if *w == well => rows,
            _ => &[],
        };
        let small = th <= 2;
        let cross: &[&str] = if small {
            &["x.x", ".x."]
        } else {
            &["x.x", ".x.", "x.x"]
        };
        let diamond: &[&str] = if small {
            &["xxx", ".x."]
        } else {
            &[".x.", "xxx", ".x."]
        };
        let mark_y = |y: i32| ty(y) + (th - if small { 2 } else { 3 }) / 2;
        for y in 0..H {
            let row = &self.wells[well][y];
            let feedback = row.contains(&Some(Part::Feedback));
            for (x, cell) in row.iter().enumerate() {
                let (px, py) = (tx(x as i32), ty(y as i32));
                if clearing.contains(&y) {
                    continue;
                }
                match *cell {
                    Some(part) => tile(&mut layer, &ink, &pal, px, py, tw, th, part, Look::Placed),
                    None if feedback => {
                        // the one break in a feedback row, marked with a small cross
                        let mark = ink.solid(pal.warning);
                        layer.sprite(px + (tw - 3) / 2, mark_y(y as i32), cross, &[('x', mark)]);
                    }
                    None => {
                        // a solder pad, a ring with a hole on the larger tiles
                        let side = (tw.min(th) / 3).max(1);
                        let (pw, ph) = (side + i32::from(tw > th + 2), side);
                        let (ox, oy) = (px + (tw - pw) / 2, py + (th - ph) / 2);
                        layer.px.fill_rect(ox, oy, pw, ph, pad);
                        if pw >= 3 && ph >= 3 {
                            layer.px.set(ox + pw / 2, oy + ph / 2, board);
                        }
                    }
                }
            }
            let full = row.iter().all(Option::is_some);
            let mark = if clearing.contains(&y) {
                Some((diamond, ink.solid(pal.bright)))
            } else if full && !Self::joined(row) {
                // a full row that will not clear is marked on both rails, by shape and colour
                Some((cross, ink.solid(pal.warning)))
            } else {
                None
            };
            if let Some((rows, colour)) = mark {
                for rx in [(rail - 3) / 2, right + (rail - 3) / 2] {
                    layer.sprite(rx, mark_y(y as i32), rows, &[('x', colour)]);
                }
            }
        }
        // a clearing row glows and carries one wire from rail to rail
        for &y in clearing {
            let py = ty(y as i32);
            layer
                .px
                .fill_rect(rail, py, W as i32 * tw, th, ink.tint(pal.bright, 0.55));
            let thick = (th / 3).max(1);
            layer.px.fill_rect(
                0,
                py + (th - thick) / 2,
                right + rail,
                thick,
                ink.solid(pal.bright),
            );
        }
        if let Some(p) = &self.piece
            && p.well == well
        {
            let mut ghost = p.clone();
            while self.fits(&Piece {
                y: ghost.y + 1,
                ..ghost.clone()
            }) {
                ghost.y += 1;
            }
            if ghost.y != p.y {
                for (x, y, part) in ghost.at() {
                    if y >= 0 {
                        tile(
                            &mut layer,
                            &ink,
                            &pal,
                            tx(x),
                            ty(y),
                            tw,
                            th,
                            part,
                            Look::Ghost,
                        );
                    }
                }
            }
            for (x, y, part) in p.at() {
                if y >= 0 {
                    tile(
                        &mut layer,
                        &ink,
                        &pal,
                        tx(x),
                        ty(y),
                        tw,
                        th,
                        part,
                        Look::Falling,
                    );
                }
            }
        }
        layer.paint(c);
    }
}

/// How a tile is drawn: settled in the well, falling or the faint copy where it will land.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Look {
    Placed,
    Falling,
    Ghost,
}

/// One part as a chip `tw` by `th` pixels with copper running the way the part carries wire,
/// across, up and down or both, so the wire reads by shape as well as colour and a row of
/// across parts shows one wire from rail to rail. The chip is lit along its top and shaded
/// along its lower and right edges so neighbouring chips stay apart; the copper keeps a bright
/// upper edge where it is thick enough.
#[allow(clippy::too_many_arguments)]
fn tile(
    layer: &mut Field,
    ink: &Ink,
    pal: &Palette,
    x: i32,
    y: i32,
    tw: i32,
    th: i32,
    part: Part,
    look: Look,
) {
    let hue = match part {
        Part::Across => pal.cyan,
        Part::Upright => pal.magenta,
        Part::Cross => pal.yellow,
        Part::Feedback => pal.warning,
    };
    let (body, top, edge, wire, shine) = match look {
        Look::Placed => (
            ink.shade(hue, -0.45),
            ink.shade(hue, -0.3),
            ink.shade(hue, -0.7),
            ink.shade(hue, 0.3),
            ink.lit(ink.shade(hue, 0.3)).0,
        ),
        Look::Falling => (
            ink.solid(hue),
            ink.lit(hue).0,
            ink.shade(hue, -0.35),
            ink.shade(hue, 0.65),
            ink.lit(ink.shade(hue, 0.65)).0,
        ),
        Look::Ghost => {
            let t = ink.tint(hue, 0.3);
            (t, t, t, ink.tint(hue, 0.6), ink.tint(hue, 0.6))
        }
    };
    let thick_h = (th / 3).max(1);
    let thick_w = (tw / 4).max(1);
    let (wy, wx) = ((th - thick_h) / 2, (tw - thick_w) / 2);
    for dy in 0..th {
        for dx in 0..tw {
            let across = part.across() && (wy..wy + thick_h).contains(&dy);
            let upright = part.upright() && (wx..wx + thick_w).contains(&dx);
            let colour = if across || upright {
                let first =
                    (across && dy == wy && thick_h >= 2) || (upright && dx == wx && thick_w >= 2);
                if first { shine } else { wire }
            } else if dy == th - 1 || (dx == tw - 1 && tw >= 5) {
                edge
            } else if dy == 0 && th >= 3 {
                top
            } else {
                body
            };
            layer.px.set(x + dx, y + dy, colour);
        }
    }
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
