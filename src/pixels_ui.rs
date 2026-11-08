//! Pixel art play fields, on the half-block canvas from Scenes: two pixels to a cell, the top one
//! drawn as the foreground of an upper half block and the bottom one as its background. A game
//! paints its ground, pieces and sprites into a [`Field`] the size of a rectangle of cells, sets
//! it into the frame, then draws its text on top as text.
//!
//! Ground is a tint of the background kept under the imagery ceiling and, where text can sit on
//! it, under the tint that still keeps every text colour at the theme's floor, so scores, labels
//! and typed words stay as readable as they are on the plain background. Pieces are solid: at or
//! above the contrast that keeps their outline with colour off, so a shape carries its meaning as
//! well as its colour.

use std::ops::{Deref, DerefMut};

use zacess_scenes::colour::Rgb;
use zacess_scenes::frame::Cell;
use zacess_scenes::pixels::{Canvas as Pixels, IMAGERY_CEILING, Shades, mix};

use crate::ui::{Canvas, Palette, Rect};

/// The contrast a solid pixel keeps against the background, a little over the ceiling so a
/// shape never sits on the line between showing and vanishing with colour off.
pub const SOLID: f64 = IMAGERY_CEILING + 0.2;

/// A pixel canvas covering a rectangle of cells, two pixels to a cell from top to bottom.
pub struct Field {
    pub px: Pixels,
    pub at: Rect,
    pub pal: Palette,
}

impl Field {
    /// A field over `at`, filled with `fill`.
    pub fn new(pal: Palette, at: Rect, fill: Rgb) -> Self {
        Self {
            px: Pixels::new(at.w.max(1) as usize, at.h.max(1) as usize, fill),
            at,
            pal,
        }
    }

    /// Width in pixels.
    pub fn w(&self) -> i32 {
        self.px.width() as i32
    }

    /// Height in pixels.
    pub fn h(&self) -> i32 {
        self.px.height() as i32
    }

    /// Sets the field into the frame as pixel cells, inside the canvas clip only. A cell that
    /// falls outside is counted, so the tests catch a field that spills past its area.
    pub fn paint(&self, c: &mut Canvas) {
        let clip = c.clip;
        for y in 0..self.at.h {
            for x in 0..self.at.w {
                let (fx, fy) = (self.at.x + x, self.at.y + y);
                if fx < clip.x || fy < clip.y || fx >= clip.x + clip.w || fy >= clip.y + clip.h {
                    c.outside += 1;
                    continue;
                }
                let top = self.px.get(x, 2 * y).unwrap_or(self.pal.theme.background);
                let bottom = self.px.get(x, 2 * y + 1).unwrap_or(top);
                c.frame.put_cell(fx, fy, Cell::pixels(top, bottom));
            }
        }
    }
}

impl Deref for Field {
    type Target = Pixels;
    fn deref(&self) -> &Pixels {
        &self.px
    }
}

impl DerefMut for Field {
    fn deref_mut(&mut self) -> &mut Pixels {
        &mut self.px
    }
}

/// How many pixels a game's logical pixel takes when its field may grow to `room` cells: the
/// largest step of a quarter that fits both ways, from 1 up to `most`. A logical grid of `w` by
/// `h` cells is `w` by `2h` logical pixels, so the field keeps its shape at every scale.
pub fn fit(room: (i32, i32), w: i32, h: i32, most: f32) -> f32 {
    let across = room.0 as f32 / w as f32;
    let down = room.1 as f32 / h as f32;
    let k = across.min(down).min(most);
    ((k * 4.0).floor() / 4.0).max(1.0)
}

/// A shaded lump lit from the upper left, such as a rock, a moon or a ball: `tones` are the
/// lit, middle and shaded colours. `wobble` roughens the outline by up to that share of the
/// radius and `spin` turns the bumps, so a rock can tumble slowly.
pub fn lump(f: &mut Field, centre: (f32, f32), r: f32, tones: [Rgb; 3], wobble: f32, spin: f32) {
    let reach = (r * (1.0 + wobble)).ceil() as i32 + 1;
    let (cx, cy) = centre;
    let (x0, y0) = (cx.floor() as i32, cy.floor() as i32);
    for y in y0 - reach..=y0 + reach {
        for x in x0 - reach..=x0 + reach {
            let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - cy);
            let d = (dx * dx + dy * dy).sqrt();
            let a = dy.atan2(dx);
            let edge = r
                * (1.0
                    + wobble * (0.6 * (3.0 * a + spin).sin() + 0.4 * (5.0 * a - 1.7 * spin).sin()));
            if d > edge {
                continue;
            }
            let light = (-dx - dy) / (edge.max(0.5) * std::f32::consts::SQRT_2);
            let tone = if light > 0.3 {
                tones[0]
            } else if light < -0.25 || (d > edge - 1.0 && light < 0.0) {
                tones[2]
            } else {
                tones[1]
            };
            f.set(x, y, tone);
        }
    }
}

/// A one pixel ring of radius `r`.
pub fn ring(f: &mut Field, centre: (f32, f32), r: f32, colour: Rgb) {
    let reach = r.ceil() as i32 + 1;
    let (x0, y0) = (centre.0.floor() as i32, centre.1.floor() as i32);
    for y in y0 - reach..=y0 + reach {
        for x in x0 - reach..=x0 + reach {
            let (dx, dy) = (x as f32 + 0.5 - centre.0, y as f32 + 0.5 - centre.1);
            if ((dx * dx + dy * dy).sqrt() - r).abs() < 0.5 {
                f.set(x, y, colour);
            }
        }
    }
}

/// A soft glow: `colour` blended over the field, strongest at the centre and gone at `r`,
/// in a few even steps so it reads as calm rings of light rather than a smear.
pub fn glow(f: &mut Field, centre: (f32, f32), r: f32, colour: Rgb, strength: f32) {
    let reach = r.ceil() as i32 + 1;
    let (x0, y0) = (centre.0.floor() as i32, centre.1.floor() as i32);
    for y in y0 - reach..=y0 + reach {
        for x in x0 - reach..=x0 + reach {
            let (dx, dy) = (x as f32 + 0.5 - centre.0, y as f32 + 0.5 - centre.1);
            let d = (dx * dx + dy * dy).sqrt();
            if d < r {
                let amount = (strength * (1.0 - d / r) * 6.0).round() / 6.0;
                f.blend(x, y, colour, amount);
            }
        }
    }
}

/// Sets a run of `n` cells to one flat colour, under text that is drawn next, such as a field
/// to type into. The colour should come from [`Ink::surface`].
pub fn slot(c: &mut Canvas, x: i32, y: i32, n: i32, colour: Rgb) {
    let clip = c.clip;
    for i in 0..n {
        let x = x + i;
        if x >= clip.x && y >= clip.y && x < clip.x + clip.w && y < clip.y + clip.h {
            c.frame.put_cell(x, y, Cell::pixels(colour, colour));
        }
    }
}

/// Text over pixels. Each character keeps the background of the pixel cell under it while the
/// text colour holds the theme's floor against it. It falls back to the plain background where
/// it would not, so text is never harder to read for sitting on a board.
pub fn text_on(c: &mut Canvas, x: i32, y: i32, text: &str, colour: Rgb) {
    let floor = c.pal.theme.floor();
    for (i, ch) in text.chars().enumerate() {
        let (cx, cy) = (x + i as i32, y);
        let inside = cx >= c.clip.x
            && cy >= c.clip.y
            && cx < c.clip.x + c.clip.w
            && cy < c.clip.y + c.clip.h;
        if !inside {
            if ch != ' ' {
                c.outside += 1;
            }
            continue;
        }
        let under = c.frame.get(cx, cy).and_then(|cell| cell.pixel_pair());
        let bg = under.and_then(|(top, bottom)| {
            // the half that holds the text better, so a tile edge never takes its letter's place
            let pick = if colour.contrast(top) >= colour.contrast(bottom) {
                top
            } else {
                bottom
            };
            (colour.contrast(pick) >= floor).then_some(pick)
        });
        c.frame.put_cell(
            cx,
            cy,
            Cell {
                ch,
                colour: Some(colour),
                bg,
            },
        );
    }
}

/// The colours text is drawn in, which every surface under text must keep the floor against.
fn text_colours(pal: &Palette) -> [Rgb; 19] {
    [
        pal.text,
        pal.muted,
        pal.bright,
        pal.accent,
        pal.error,
        pal.warning,
        pal.info,
        pal.red,
        pal.green,
        pal.yellow,
        pal.blue,
        pal.magenta,
        pal.cyan,
        pal.keyword,
        pal.number,
        pal.link,
        pal.property,
        pal.function,
        pal.string,
    ]
}

/// Colours for pixel art, all made from the theme's own colours so every game matches the
/// chosen theme.
#[derive(Clone, Copy)]
pub struct Ink {
    pub pal: Palette,
    shades: Shades<'static>,
}

impl Ink {
    pub fn new(pal: Palette) -> Self {
        Self {
            pal,
            shades: Shades::new(pal.theme),
        }
    }

    /// Whether the theme is light, so a scene can choose its daylight counterpart.
    pub fn light(&self) -> bool {
        !self.pal.theme.dark
    }

    /// The theme background.
    pub fn back(&self) -> Rgb {
        self.pal.theme.background
    }

    /// Background imagery: the background tinted `strength` of the way towards `hue`, held
    /// below the ceiling so it never competes with the pieces.
    pub fn tint(&self, hue: Rgb, strength: f32) -> Rgb {
        self.shades.tint(hue, strength, IMAGERY_CEILING)
    }

    /// A surface any text may sit on: the background moved up to `strength` towards `hue`, but
    /// never so far that any text colour of the palette drops below the theme's floor on it.
    pub fn surface(&self, hue: Rgb, strength: f32) -> Rgb {
        let text = text_colours(&self.pal);
        self.shades
            .under_text(hue, strength, &text, self.pal.theme.floor())
    }

    /// Imagery that only the colours in `text` sit on: a tint that keeps each of them at the
    /// floor, so a board with one or two text colours can take a stronger ground.
    pub fn under(&self, hue: Rgb, strength: f32, text: &[Rgb]) -> Rgb {
        let theme = self.pal.theme;
        let tile = self.shades.under_text(hue, strength, text, theme.floor());
        if theme.contrast_of(tile) < IMAGERY_CEILING {
            tile
        } else {
            self.tint(hue, strength)
        }
    }

    /// The shaded side of a shape: `colour` moved towards the theme's darkest colour while it
    /// still stands out from the background as a shape.
    pub fn shadow(&self, colour: Rgb, t: f32) -> Rgb {
        let to = self.shades.darkest();
        self.shades.held(colour, to, t, IMAGERY_CEILING)
    }

    /// The lit side of a shape, held the same way.
    pub fn highlight(&self, colour: Rgb, t: f32) -> Rgb {
        let to = self.shades.lightest();
        self.shades.held(colour, to, t, IMAGERY_CEILING)
    }

    /// A shape's body softened towards the background, stopping where it would stop reading
    /// as a shape. On a dark theme that is a darker tone, on a light theme a paler one.
    pub fn soften(&self, colour: Rgb, t: f32) -> Rgb {
        let to = self.pal.theme.background;
        self.shades.held(colour, to, t, IMAGERY_CEILING)
    }

    /// A step of a gradient between two colours, picked from `steps` even steps so a large
    /// gradient uses only a few colours.
    pub fn step(&self, a: Rgb, b: Rgb, t: f32, steps: usize) -> Rgb {
        let steps = steps.max(2) as f32 - 1.0;
        mix(a, b, (t.clamp(0.0, 1.0) * steps).round() / steps)
    }

    /// A plain perceptual mix, for gradients inside one surface.
    pub fn mix(&self, a: Rgb, b: Rgb, t: f32) -> Rgb {
        mix(a, b, t)
    }

    /// The theme's colour furthest from the background.
    fn far(&self) -> Rgb {
        if self.pal.theme.dark {
            self.shades.lightest()
        } else {
            self.shades.darkest()
        }
    }

    /// A solid shade of `colour`. A positive `t` moves it away from the background (lighter on a
    /// dark theme, darker on a light one), which only ever gains contrast; a negative `t` moves it
    /// towards the background but stops at the solid contrast, so the shape keeps its outline.
    pub fn shade(&self, colour: Rgb, t: f32) -> Rgb {
        let theme = self.pal.theme;
        let colour = self.solid(colour);
        if t >= 0.0 {
            let moved = mix(colour, self.far(), t);
            if theme.contrast_of(moved) < theme.contrast_of(colour) {
                colour
            } else {
                moved
            }
        } else {
            self.shades.held(colour, theme.background, -t, SOLID)
        }
    }

    /// A lit face and a shaded face for a rounded or bevelled shape: the light always falls from
    /// the top left, so the upper edge is lighter and the lower edge deeper on every theme.
    pub fn lit(&self, colour: Rgb) -> (Rgb, Rgb) {
        let s = &self.shades;
        let colour = self.solid(colour);
        let up = s.held(colour, s.lightest(), 0.25, SOLID);
        let down = s.held(colour, s.darkest(), 0.3, SOLID);
        (up, down)
    }

    /// `colour` pushed away from the background, if it needs it, to the solid contrast.
    pub fn solid(&self, colour: Rgb) -> Rgb {
        let theme = self.pal.theme;
        if theme.contrast_of(colour) >= SOLID {
            return colour;
        }
        let far = self.far();
        (1..=20)
            .map(|i| mix(colour, far, i as f32 / 20.0))
            .find(|&c| theme.contrast_of(c) >= SOLID)
            .unwrap_or(far)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::app::headless;
    use crate::game::Setup;
    use crate::games;
    use crate::term::Key;
    use zacess_scenes::frame::Frame;
    use zacess_scenes::pixels::plain_glyph;
    use zacess_scenes::theme::{self, ThemeData};

    fn setup() -> Setup {
        Setup {
            seed: 3,
            reduced_motion: false,
            day: crate::SNAP_DAY,
            practice: false,
        }
    }

    /// Holds a board drawn with pixels to the rules the scenes keep: text on a tile keeps the
    /// floor against that tile. Every pixel is either imagery under the ceiling or a solid shape
    /// at or over the solid contrast, never a muddy colour in between that would half show with
    /// colour off.
    pub fn check_frame(frame: &Frame, theme: &ThemeData, what: &str) -> Vec<String> {
        let mut problems = Vec::new();
        for cell in frame.cells() {
            if let Some((top, bottom)) = cell.pixel_pair() {
                for p in [top, bottom] {
                    let ratio = theme.contrast_of(p);
                    if (IMAGERY_CEILING..SOLID - 0.01).contains(&ratio) {
                        problems.push(format!(
                            "{what} in {}: pixel {} at {ratio:.2}:1 is neither ground nor shape",
                            theme.name,
                            p.hex()
                        ));
                    }
                }
            } else if let (Some(fg), Some(bg)) = (cell.colour, cell.bg)
                && fg.contrast(bg) < theme.floor()
            {
                problems.push(format!(
                    "{what} in {}: text {} on {} is {:.2}:1",
                    theme.name,
                    fg.hex(),
                    bg.hex(),
                    fg.contrast(bg)
                ));
            }
        }
        problems.sort();
        problems.dedup();
        problems
    }

    /// Every text colour in a frame with the contrast it keeps against what it is drawn on: its
    /// own cell's background where it has one, such as a number on a shirt or a letter on a
    /// tile. Everywhere else it is the theme's background.
    pub fn text_contrasts(frame: &Frame, theme: &ThemeData) -> Vec<(Rgb, f64)> {
        let mut out: Vec<(Rgb, f64)> = frame
            .cells()
            .iter()
            .filter(|c| !c.is_pixel())
            .filter_map(|c| {
                let fg = c.colour?;
                Some((fg, fg.contrast(c.bg.unwrap_or(theme.background))))
            })
            .collect();
        out.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.total_cmp(&b.1)));
        out.dedup();
        out
    }

    #[test]
    fn surfaces_keep_every_text_colour_at_the_floor() {
        for t in theme::all() {
            let pal = Palette::new(t);
            let ink = Ink::new(pal);
            for hue in [pal.blue, pal.green, pal.red, pal.yellow, pal.text] {
                let s = ink.surface(hue, 0.5);
                for colour in text_colours(&pal) {
                    assert!(colour.contrast(s) >= t.floor(), "{}", t.name);
                }
                assert!(t.contrast_of(ink.tint(hue, 0.6)) < IMAGERY_CEILING);
            }
        }
    }

    #[test]
    fn shades_keep_their_outline_and_tints_stay_ground() {
        for t in theme::all() {
            let pal = Palette::new(t);
            let ink = Ink::new(pal);
            for colour in pal.all() {
                for amount in [-0.8, -0.3, 0.0, 0.3, 0.8] {
                    assert!(t.contrast_of(ink.shade(colour, amount)) >= SOLID - 0.01);
                }
                let (up, down) = ink.lit(colour);
                assert!(t.contrast_of(up) >= SOLID - 0.01, "{}", t.name);
                assert!(t.contrast_of(down) >= SOLID - 0.01, "{}", t.name);
                assert!(t.contrast_of(ink.tint(colour, 0.5)) < IMAGERY_CEILING);
                let tile = ink.under(colour, 0.5, &[pal.text, pal.bright]);
                assert!(t.contrast_of(tile) < IMAGERY_CEILING);
                assert!(pal.text.contrast(tile) >= t.floor(), "{}", t.name);
            }
        }
    }

    #[test]
    fn a_field_paints_inside_its_rectangle_only() {
        let t = theme::default_theme();
        let pal = Palette::new(t);
        let mut frame = Frame::new(6, 4);
        let mut c = Canvas::new(&mut frame, pal);
        let mut f = Field::new(pal, Rect::new(1, 1, 3, 2), pal.red);
        f.set(0, 0, pal.blue);
        f.paint(&mut c);
        assert_eq!(
            frame.get(1, 1).unwrap().pixel_pair(),
            Some((pal.blue, pal.red))
        );
        assert_eq!(frame.get(0, 0).unwrap(), Cell::BLANK);
        assert_eq!(frame.get(4, 1).unwrap(), Cell::BLANK);
    }

    #[test]
    fn text_on_pixel_imagery_keeps_the_floor_in_every_game_and_theme() {
        // text drawn over a play field takes the field's colour as its background, so it is held
        // to the floor against that colour as well as against the theme background
        let mut problems = Vec::new();
        for t in theme::all() {
            for info in games::ALL {
                let game = games::make(info.id, setup()).unwrap();
                let mut keys = game.demo();
                keys.push((Key::None, 0.5));
                for size in [(80, 24), (207, 48)] {
                    let (frame, _) = headless(info.id, setup(), &keys, size, t, None).unwrap();
                    for cell in frame.cells() {
                        if cell.is_pixel() || cell.ch == ' ' {
                            continue;
                        }
                        if let (Some(fg), Some(bg)) = (cell.colour, cell.bg)
                            && fg.contrast(bg) < t.floor()
                        {
                            problems.push(format!(
                                "{} in {} at {size:?}: {} on {} is {:.2}:1",
                                info.id,
                                t.name,
                                fg.hex(),
                                bg.hex(),
                                fg.contrast(bg)
                            ));
                        }
                    }
                }
            }
        }
        problems.dedup();
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }

    /// The games whose boards are held to the solid shape rule as well as the text floor. The
    /// other seven shade their pieces with [`Ink::shadow`] and [`Ink::highlight`], which stop at
    /// the imagery ceiling itself rather than the solid contrast over it.
    pub const SOLID_GAMES: [&str; 7] = [
        "signal",
        "slipstream",
        "stackline",
        "termle",
        "through-ball",
        "touch-typing",
        "trace",
    ];

    #[test]
    fn every_solid_board_keeps_the_imagery_rules_in_every_theme_and_size() {
        let setup = Setup {
            seed: 7,
            reduced_motion: false,
            day: 20_400,
            practice: false,
        };
        let mut problems = Vec::new();
        for t in theme::all() {
            for id in SOLID_GAMES {
                let game = games::make(id, setup).unwrap();
                let mut keys = game.demo();
                keys.push((Key::None, 0.4));
                for size in [(60, 20), (80, 24), (120, 34), (207, 48)] {
                    let (frame, _) = headless(id, setup, &keys, size, t, None).unwrap();
                    let what = format!("{id} at {}x{}", size.0, size.1);
                    problems.extend(check_frame(&frame, t, &what));
                    let painted = frame.cells().iter().filter(|c| c.is_pixel()).count();
                    // with colour off the ground drops away and the pieces keep their outline
                    let ink = frame
                        .cells()
                        .iter()
                        .filter_map(|c| c.pixel_pair())
                        .filter(|&(a, b)| plain_glyph(a, b, t.background) != ' ')
                        .count();
                    // a board whose pieces are letters on tiles, such as the keyboard, carries
                    // its shapes as text instead
                    let lettered = frame
                        .cells()
                        .iter()
                        .filter(|c| !c.is_pixel() && c.bg.is_some() && c.ch != ' ')
                        .count();
                    // the smallest window leaves the keyboard out, keeping the line to type
                    let keyboard_left_out = id == "touch-typing" && size.1 < 24;
                    if !keyboard_left_out && (painted < 40 || (ink < 8 && lettered < 12)) {
                        problems.push(format!(
                            "{what} in {} paints {painted} cells, {ink} of them shapes",
                            t.name
                        ));
                    }
                }
            }
        }
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }
}
