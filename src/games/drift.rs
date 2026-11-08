//! Drift: a slow zen game. A lantern floats over a quiet landscape that scrolls past at walking
//! pace. Steer it to gather motes of light; each one brightens the lantern's halo and plays a
//! soft note that climbs a scale. Nothing can be lost: hills only lift the lantern gently. The
//! journey ends when the player chooses. The light gathered is the score.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::pixels_ui::{Field, Ink, fit, glow, lump};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status};

pub const INFO: Info = Info {
    id: "drift",
    name: "Drift",
    icon: ('✦', '*'),
    hue: Hue::String,
    blurb: "a lantern in a quiet landscape",
    hints: "Arrows steer · Enter ends the journey",
    help: &[
        "# Goal",
        "Float a lantern over a quiet landscape and gather motes of light.",
        "# Rules",
        "- The land scrolls slowly past. Arrows give the lantern a gentle push and it glides to a stop.",
        "- Touching a mote (a spark of light, which is a small butterfly on a light theme) gathers it with a soft note and the lantern's halo grows.",
        "- The hills only lift the lantern; nothing can harm it.",
        "# Scoring",
        "One point for every mote gathered.",
        "# Winning and losing",
        "There is no way to lose. Enter ends the journey whenever you like and keeps the light gathered.",
        "# Keys",
        "- Arrows (or H J K L) steer.",
        "- Enter ends the journey.",
        "Tip: reduced motion halves the pace if the scrolling is too much.",
    ],
    input: Input::Keys,
    unit: "motes",
};

/// A mote of light: a soft spark, a bright point inside a faint glow, like a firefly. On a
/// light theme, where a spark would read as a dark speck, it is a small butterfly instead.
fn mote(f: &mut Field, ink: &Ink, centre: (f32, f32), k: f32) {
    let pal = ink.pal;
    let (x, y) = (centre.0.floor() as i32, centre.1.floor() as i32);
    if ink.light() {
        let palette = [('w', ink.soften(pal.magenta, 0.35)), ('b', pal.magenta)];
        if k >= 2.0 {
            let rows = ["ww.ww", "wwbww", ".wbw.", "..b.."];
            f.sprite(x - 2, y - 2, &rows, &palette);
        } else {
            f.sprite(x - 1, y - 1, &["w.w", "wbw"], &palette);
        }
        return;
    }
    let (core, edge) = (ink.highlight(pal.yellow, 0.5), pal.yellow);
    glow(f, centre, 1.8 * k, pal.yellow, 0.32);
    if k >= 2.0 {
        f.disc(centre.0, centre.1, 0.9 * k, edge);
        f.disc(centre.0, centre.1, 0.45 * k, core);
    } else {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            f.blend(x + dx, y + dy, edge, 0.55);
        }
        f.set(x, y, core);
    }
}

/// The landscape's height at a world column, from 0 to 1: three slow waves.
pub fn ground(seed: u64, x: f32) -> f32 {
    let s = (seed % 1000) as f32;
    let a = (x * 0.045 + s).sin() * 0.5;
    let b = (x * 0.11 + s * 1.7).sin() * 0.3;
    let c = (x * 0.023 + s * 0.3).sin() * 0.2;
    ((a + b + c) * 0.5 + 0.5).clamp(0.0, 1.0)
}

#[derive(Clone, Copy, Debug)]
struct Mote {
    x: f32,
    y: f32,
}

pub struct Drift {
    seed: u64,
    rng: Rng,
    /// How far the world has scrolled, in columns.
    pub travelled: f32,
    speed: f32,
    /// The lantern's place on screen, in cells from the play area's top left.
    pub pos: (f32, f32),
    vel: (f32, f32),
    motes: Vec<Mote>,
    pub gathered: u64,
    ended: bool,
    note: usize,
    size: (f32, f32),
}

/// The scene inside the frame, fixed so the journey is the same at any terminal size.
const SCENE_W: i32 = 70;
const SCENE_H: i32 = 14;
/// The lantern keeps to the left part of the scene, so it fits the smallest terminal too.
const LANTERN_MAX_X: f32 = 44.0;

const PENTA: [i8; 8] = [0, 2, 5, 7, 9, 12, 14, 17];

impl Drift {
    pub fn new(setup: Setup) -> Self {
        let mut d = Self {
            seed: setup.seed,
            rng: Rng::new(setup.seed),
            travelled: 0.0,
            speed: if setup.reduced_motion { 2.0 } else { 4.0 },
            pos: (14.0, 5.0),
            vel: (0.0, 0.0),
            motes: Vec::new(),
            gathered: 0,
            ended: false,
            note: 0,
            size: (SCENE_W as f32, SCENE_H as f32),
        };
        for i in 0..6 {
            let x = 20.0 + i as f32 * 9.0;
            let y = d.rng.range(2.0, 9.0);
            d.motes.push(Mote { x, y });
        }
        d
    }

    /// The top of the hills at a screen column, in rows from the top of the scene.
    fn hill_top(&self, col: f32) -> f32 {
        let h = self.size.1;
        h - 1.0 - ground(self.seed, col + self.travelled) * (h * 0.3)
    }

    /// The sky: a dusk gradient with a shaded moon and a few still stars on a dark theme, a
    /// pale day with a soft sun and slow clouds on a light one. `k` is pixels per logical
    /// pixel, so the scene keeps its shape as it grows with the window.
    fn paint_sky(&self, f: &mut Field, ink: &Ink, k: f32) {
        let pal = ink.pal;
        let (w, h) = (f.w(), f.h());
        let (top, low) = if ink.light() {
            (ink.tint(pal.blue, 0.4), ink.tint(pal.yellow, 0.18))
        } else {
            (ink.tint(pal.blue, 0.45), ink.tint(pal.magenta, 0.3))
        };
        for y in 0..h {
            f.fill_row(y, ink.step(top, low, y as f32 / (h as f32 * 0.8), 10));
        }
        let centre = (w as f32 * 0.84, h as f32 * 0.22);
        if ink.light() {
            let sun = ink.mix(ink.back(), ink.tint(pal.yellow, 0.6), 0.35);
            glow(f, centre, 5.0 * k, ink.back(), 0.5);
            f.disc(centre.0, centre.1, 2.4 * k, sun);
        } else {
            let tones = [
                ink.mix(ink.back(), pal.text, 0.7),
                ink.mix(ink.back(), pal.text, 0.55),
                ink.mix(ink.back(), pal.text, 0.42),
            ];
            glow(f, centre, 5.0 * k, pal.text, 0.12);
            lump(f, centre, 2.4 * k, tones, 0.0, 0.0);
        }
        let seed = self.seed;
        if ink.light() {
            // two soft clouds drifting at a twentieth of the ground's pace
            let cloud = ink.mix(ink.back(), ink.tint(pal.blue, 0.1), 0.3);
            for i in 0..2u64 {
                let span = w as f32 + 24.0 * k;
                let cx = (seed.wrapping_mul(31).wrapping_add(i * 41) % 97) as f32 * k
                    + i as f32 * span / 2.0;
                let x = (cx - self.travelled * 0.05 * k).rem_euclid(span) - 12.0 * k;
                let y = (4.0 + i as f32 * 4.0) * k;
                for (dx, dy, rad) in [(0.0, 0.0, 2.2), (3.0, -1.0, 2.8), (6.5, 0.0, 2.2)] {
                    f.disc(x + dx * k, y + dy * k, rad * k, cloud);
                }
                let (cw, ch) = ((7.0 * k) as i32, (2.0 * k) as i32);
                f.fill_rect(x.round() as i32, y.round() as i32, cw, ch, cloud);
            }
        } else {
            // still stars, drifting at a tenth of the ground's pace; none twinkle
            let star = ink.mix(ink.back(), pal.text, 0.45);
            for i in 0..14u64 {
                let sx = ((i * 7919).wrapping_add(seed) % 997) as f32 * k;
                let x = (sx - self.travelled * 0.1 * k).rem_euclid(w as f32);
                let y =
                    ((i * 131).wrapping_add(seed.wrapping_mul(7)) % (h as u64 / 2).max(1)) as i32;
                f.set(x as i32, y, star);
            }
        }
    }

    /// Far hills in haze with a few small pines, then the near hills: a lit ridge over a
    /// softer body, with reeds that scroll with the ground.
    fn paint_land(&self, f: &mut Field, ink: &Ink, k: f32) {
        let pal = ink.pal;
        let (w, h) = (f.w(), f.h());
        let sh = SCENE_H as f32;
        let haze = if ink.light() {
            ink.tint(pal.cyan, 0.5)
        } else {
            ink.tint(pal.blue, 0.75)
        };
        let haze_dark = ink.mix(haze, ink.tint(pal.green, 0.9), 0.5);
        let far_top = |col: f32| {
            let world = col + self.travelled * 0.5;
            let rows = sh - 2.0 - ground(self.seed.wrapping_add(17), world * 1.6) * (sh * 0.6);
            rows * 2.0 * k
        };
        for x in 0..w {
            let top = far_top(x as f32 / k).round() as i32;
            f.fill_rect(x, top, 1, h - top, haze);
        }
        // a little pine on the far ridge every thirteen columns of the far land
        let start = (self.travelled * 0.5).floor() as i64;
        for world in start - 4..start + SCENE_W as i64 + 4 {
            if world.rem_euclid(13) != 0 {
                continue;
            }
            let col = world as f32 - self.travelled * 0.5;
            let (x, base) = (col * k, far_top(col));
            let tall = 5.0 * k;
            let tiers = 3;
            for t in 0..tiers {
                let y0 = base - tall + t as f32 * tall / 4.0;
                let rows = (tall / 2.5).max(1.0) as i32;
                for r in 0..rows {
                    let half = (r as f32 + 1.0) * (1.0 + t as f32 * 0.5) * k / rows as f32 * 1.4;
                    let y = (y0 + r as f32).round() as i32;
                    let (x0, x1) = ((x - half).round() as i32, (x + half).round() as i32);
                    f.fill_rect(x0, y, (x1 - x0).max(1), 1, haze_dark);
                }
            }
            let trunk = (0.4 * k).round().max(1.0) as i32;
            f.fill_rect(
                x.round() as i32,
                (base - k).round() as i32,
                trunk,
                k.ceil() as i32 + 1,
                haze_dark,
            );
        }
        let ridge = pal.green;
        let body = ink.soften(pal.green, 0.55);
        let deep = ink.soften(pal.green, 0.8);
        let lit = ink.highlight(pal.green, 0.25);
        let edge = (0.5 * k).round().max(1.0) as i32;
        for x in 0..w {
            let top = (self.hill_top(x as f32 / k) * 2.0 * k).round() as i32;
            f.fill_rect(x, top, 1, edge, lit);
            f.fill_rect(x, top + edge, 1, edge, ridge);
            let band = top + 2 * edge + (3.0 * k) as i32;
            f.fill_rect(x, top + 2 * edge, 1, band - top - 2 * edge, body);
            f.fill_rect(x, band, 1, h - band, deep);
        }
        // reeds rooted in the near ridge: a stalk with a seed head
        let reed = ink.soften(pal.string, 0.2);
        let start = self.travelled.floor() as i64;
        for world in start - 1..start + SCENE_W as i64 + 2 {
            if world.rem_euclid(11) != 0 {
                continue;
            }
            let col = world as f32 - self.travelled;
            let x = (col * k).round() as i32;
            let top = (self.hill_top(col.max(0.0)) * 2.0 * k).round() as i32;
            let stalk = (3.0 * k).round() as i32;
            let thick = (0.4 * k).round().max(1.0) as i32;
            f.fill_rect(x, top - stalk, thick, stalk, body);
            f.fill_rect(
                x,
                top - stalk - (2.0 * k) as i32,
                thick,
                (2.0 * k) as i32,
                reed,
            );
            if world.rem_euclid(22) == 0 {
                let side = (2.0 * k).round() as i32;
                f.fill_rect(x + thick + 1, top - side, thick, side, body);
            }
        }
    }

    /// The lantern: a hanger, a roof, a frame around glowing panes with a bright flame and a
    /// base, inside a soft halo that grows with the light gathered.
    fn paint_lantern(&self, f: &mut Field, ink: &Ink, k: f32) {
        let pal = ink.pal;
        let cx = (self.pos.0 + 0.5) * k;
        let cy = (self.pos.1 * 2.0 + 1.0) * k;
        let halo = (3.5 + (self.gathered / 8).min(3) as f32 * 1.5) * k;
        // the halo brightens the sky around it: warm light on a dark theme, a paler haze of
        // daylight on a light one
        let light = if ink.light() { ink.back() } else { pal.yellow };
        glow(
            f,
            (cx, cy),
            halo,
            light,
            if ink.light() { 0.6 } else { 0.36 },
        );
        // on a light theme the yellow role is a deep ochre, so the panes are softened towards
        // the paper colour to read as lit rather than as a dark blot
        let (pane, core, frame) = if ink.light() {
            (
                ink.mix(ink.back(), ink.tint(pal.yellow, 0.9), 0.5),
                ink.back(),
                pal.red,
            )
        } else {
            (
                pal.yellow,
                ink.highlight(pal.yellow, 0.55),
                ink.shadow(pal.red, 0.3),
            )
        };
        let frame_dark = ink.shadow(frame, 0.3);
        let bw = (4.0 * k).round().max(4.0) as i32;
        let bh = (4.0 * k).round().max(4.0) as i32;
        let line = (0.5 * k).round().max(1.0) as i32;
        let x0 = (cx - bw as f32 / 2.0).round() as i32;
        let y0 = (cy - bh as f32 / 2.0).round() as i32;
        // the hanger and the roof, wider than the body
        let hang = (1.2 * k).round().max(1.0) as i32;
        f.fill_rect(
            x0 + bw / 2 - line / 2,
            y0 - line - hang - line,
            line,
            hang,
            pal.muted,
        );
        f.fill_rect(x0 + line, y0 - 2 * line, bw - 2 * line, line, frame);
        f.fill_rect(x0 - line, y0 - line, bw + 2 * line, line, frame);
        // the frame and the panes inside it
        f.fill_rect(x0, y0, bw, bh, frame);
        f.fill_rect(x0 + line, y0, bw - 2 * line, bh, pane);
        if bw >= 6 {
            f.fill_rect(x0 + bw / 2 - line / 2, y0, line, bh, frame_dark);
        }
        f.disc(cx, cy, (0.8 * k).max(0.6), core);
        // the base
        f.fill_rect(x0 - line, y0 + bh, bw + 2 * line, line, frame_dark);
    }

    fn nudge(&mut self, dx: f32, dy: f32) {
        self.vel.0 = (self.vel.0 + dx).clamp(-12.0, 12.0);
        self.vel.1 = (self.vel.1 + dy).clamp(-8.0, 8.0);
    }
}

impl Game for Drift {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn natural_size(&self) -> Option<(i32, i32)> {
        Some((SCENE_W + 2, SCENE_H + 2))
    }

    fn crop_anchor(&self) -> (f32, f32) {
        (0.0, 1.0)
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        match key {
            Key::Up | Key::Char('k' | 'K') => {
                self.nudge(0.0, -4.0);
                fx.sound(Sfx::Tick);
            }
            Key::Down | Key::Char('j' | 'J') => {
                self.nudge(0.0, 4.0);
                fx.sound(Sfx::Tick);
            }
            Key::Left | Key::Char('h' | 'H') => {
                self.nudge(-6.0, 0.0);
                fx.sound(Sfx::Tick);
            }
            Key::Right | Key::Char('l' | 'L') => {
                self.nudge(6.0, 0.0);
                fx.sound(Sfx::Tick);
            }
            Key::Enter => {
                self.ended = true;
                fx.sound(Sfx::Win);
            }
            _ => {}
        }
    }

    fn tick(&mut self, dt: f32, fx: &mut Fx) {
        let (w, h) = self.size;
        self.travelled += self.speed * dt;
        // the lantern glides: velocity eases away so a tap is a gentle push
        self.pos.0 = (self.pos.0 + self.vel.0 * dt).clamp(2.0, LANTERN_MAX_X.min(w - 3.0));
        self.pos.1 = (self.pos.1 + self.vel.1 * dt).clamp(1.0, h - 2.0);
        let ease = (1.0 - 2.5 * dt).max(0.0);
        self.vel.0 *= ease;
        self.vel.1 *= ease;
        // the hills lift the lantern rather than stop it
        let top = self.hill_top(self.pos.0) - 1.0;
        if self.pos.1 > top {
            self.pos.1 = top.max(1.0);
            self.vel.1 = self.vel.1.min(0.0);
        }
        for m in &mut self.motes {
            m.x -= self.speed * dt;
        }
        let (px, py) = self.pos;
        let before = self.motes.len();
        self.motes
            .retain(|m| !((m.x - px).abs() < 1.6 && (m.y - py).abs() < 1.0));
        let caught = before - self.motes.len();
        for _ in 0..caught {
            self.gathered += 1;
            fx.sound(Sfx::Note(PENTA[self.note % PENTA.len()] - 5));
            self.note += 1;
            if self.gathered.is_multiple_of(10) {
                fx.say(format!("{} motes gathered", self.gathered));
            }
        }
        self.motes.retain(|m| m.x > -2.0);
        while self.motes.len() < 7 {
            let x = self.motes.iter().map(|m| m.x).fold(w, f32::max) + self.rng.range(6.0, 12.0);
            let top = self.hill_top(x.min(w - 1.0)) - 2.0;
            let y = self.rng.range(2.0, top.max(3.0));
            self.motes.push(Mote { x, y });
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        // the scene grows with the window, keeping its own grid
        let k = fit((area.w - 2, area.h - 2), SCENE_W, SCENE_H, 3.0);
        let (sw, sh) = (
            (SCENE_W as f32 * k).round() as i32,
            (SCENE_H as f32 * k).round() as i32,
        );
        let r = area.centre((sw + 2).min(area.w), sh + 2);
        c.titled(r, Line::Heavy, pal.string, "Journey", pal.bright);
        let inner = Rect::new(r.x + 1, r.y + 1, r.w - 2, r.h - 2);
        let ink = Ink::new(pal);
        let mut f = Field::new(pal, inner, ink.back());
        self.paint_sky(&mut f, &ink, k);
        self.paint_land(&mut f, &ink, k);
        for m in &self.motes {
            if m.x >= 0.0 && m.x < SCENE_W as f32 {
                mote(&mut f, &ink, ((m.x + 0.5) * k, (m.y * 2.0 + 1.0) * k), k);
            }
        }
        self.paint_lantern(&mut f, &ink, k);
        f.paint(c);
    }

    /// The scene grows with the window by itself.
    fn zoomable(&self) -> bool {
        false
    }

    fn status(&self) -> Status {
        vec![
            ("Motes".into(), self.gathered.to_string()),
            ("Travelled".into(), format!("{:.0} m", self.travelled)),
        ]
    }

    fn score(&self) -> u64 {
        self.gathered
    }

    fn outcome(&self) -> Option<Outcome> {
        self.ended.then(|| {
            Outcome::Won(format!(
                "A journey of {:.0} metres, {} motes of light.",
                self.travelled, self.gathered
            ))
        })
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        vec![(Key::Up, 1.0), (Key::Right, 1.5), (Key::Down, 1.0)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(reduced: bool) -> Setup {
        Setup {
            seed: 8,
            reduced_motion: reduced,
            day: 0,
            practice: false,
        }
    }

    #[test]
    fn the_ground_stays_in_range_and_is_fixed_by_the_seed() {
        for i in 0..500 {
            let v = ground(3, i as f32 * 0.7);
            assert!((0.0..=1.0).contains(&v));
            assert_eq!(v, ground(3, i as f32 * 0.7));
        }
    }

    #[test]
    fn the_lantern_never_sinks_into_the_hills() {
        let mut d = Drift::new(setup(false));
        let mut fx = Fx::default();
        for _ in 0..600 {
            d.key(Key::Down, &mut fx);
            d.tick(1.0 / 30.0, &mut fx);
            assert!(d.pos.1 <= d.hill_top(d.pos.0) - 1.0 + 1e-3);
        }
        assert!(d.outcome().is_none(), "there is no way to lose");
    }

    #[test]
    fn touching_a_mote_gathers_it_with_a_note() {
        let mut d = Drift::new(setup(false));
        d.motes = vec![Mote {
            x: d.pos.0,
            y: d.pos.1,
        }];
        let mut fx = Fx::default();
        d.tick(0.01, &mut fx);
        assert_eq!(d.gathered, 1);
        assert!(matches!(fx.sounds[0], Sfx::Note(_)));
    }

    #[test]
    fn reduced_motion_halves_the_pace() {
        let mut a = Drift::new(setup(false));
        let mut b = Drift::new(setup(true));
        let mut fx = Fx::default();
        a.tick(1.0, &mut fx);
        b.tick(1.0, &mut fx);
        assert!((a.travelled - 2.0 * b.travelled).abs() < 1e-3);
    }

    #[test]
    fn enter_ends_the_journey_as_a_win() {
        let mut d = Drift::new(setup(false));
        d.key(Key::Enter, &mut Fx::default());
        assert!(matches!(d.outcome(), Some(Outcome::Won(_))));
    }
}
