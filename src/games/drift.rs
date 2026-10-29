//! Drift: a slow zen game. A lantern floats over a quiet landscape that scrolls past at walking
//! pace. Steer it to gather motes of light; each one brightens the lantern's halo and plays a
//! soft note that climbs a scale. Nothing can be lost: hills only lift the lantern gently. The
//! journey ends when the player chooses. The light gathered is the score.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, g};

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
        "- Touching a mote (✧) gathers it with a soft note and the lantern's halo grows.",
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

/// A lower block filled to the part of the cell below `top`, for a smooth ridge.
fn eighth(top: f32) -> char {
    let fill = 1.0 - top.fract();
    let n = (fill * 8.0).round().clamp(1.0, 8.0) as usize;
    ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'][n - 1]
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
        let r = area.centre((SCENE_W + 2).min(area.w), SCENE_H + 2);
        c.titled(r, Line::Heavy, pal.string, "Journey", pal.bright);
        let inner = Rect::new(r.x + 1, r.y + 1, r.w - 2, r.h - 2);
        let (sw, sh) = (inner.w as f32, inner.h as f32);
        // stars: fixed in the sky, drifting at a tenth of the ground's pace
        for i in 0..24u64 {
            let sx = ((i * 7919 + self.seed) % 997) as f32;
            let x = (sx - self.travelled * 0.1).rem_euclid(sw);
            let y = ((i * 31 + self.seed) % (inner.h as u64 / 2).max(1)) as f32;
            let glyph = if i % 5 == 0 { g('✦', '+') } else { '·' };
            c.put(inner.x + x as i32, inner.y + y as i32, glyph, pal.muted);
        }
        // far hills: half speed, only their ridge, so they read as distance rather than mass
        for col in 0..inner.w {
            let x = col as f32;
            let far = sh - 1.0 - ground(self.seed + 17, x + self.travelled * 0.5) * (sh * 0.5);
            c.put(
                inner.x + col,
                inner.y + far.floor() as i32,
                eighth(far),
                pal.muted,
            );
        }
        // near hills: a smooth ridge in eighths over a light shade body
        let shade = c.pal.theme.faint_shade();
        for col in 0..inner.w {
            let top = self.hill_top(col as f32);
            let t = top.floor() as i32;
            c.put(inner.x + col, inner.y + t, eighth(top), pal.green);
            for y in (t + 1).max(0)..inner.h {
                c.put(inner.x + col, inner.y + y, shade, pal.green);
            }
            // a reed now and then, from the world column so it scrolls with the ground
            let world = (col as f32 + self.travelled) as i64;
            if world.rem_euclid(11) == 0 && t > 1 {
                c.put(inner.x + col, inner.y + t - 1, '╽', pal.string);
            }
        }
        for m in &self.motes {
            if m.x >= 0.0 && m.x < sw {
                c.put(
                    inner.x + m.x as i32,
                    inner.y + m.y as i32,
                    g('✧', '*'),
                    pal.yellow,
                );
            }
        }
        // the lantern and a halo that grows with the light gathered
        let (lx, ly) = (inner.x + self.pos.0 as i32, inner.y + self.pos.1 as i32);
        let halo = (1 + self.gathered / 8).min(3) as i32;
        for dy in -1..=1 {
            for dx in -halo..=halo {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let dist = (dx * dx) as f32 / (halo * halo) as f32 + (dy * dy) as f32;
                if dist <= 1.0 {
                    c.put(lx + dx, ly + dy, '·', pal.yellow);
                }
            }
        }
        c.put(lx, ly - 1, '╻', pal.string);
        c.put(lx, ly, g('◈', '@'), pal.string);
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
