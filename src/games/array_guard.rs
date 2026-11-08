//! Array Guard: a calm but lively shooter. Debris drifts down towards a row of five satellite
//! dishes. A turret on the rail above them fires up to clear it. Each dish has a shield of three;
//! a hit takes one and a dish with no shield left is lost to the next hit. Capsules give a
//! spread shot, a shield top-up or a slow field. Waves grow a little each time. Bursts are rings
//! that grow over a third of a second and never blink.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use zacess_scenes::colour::Rgb;

use crate::pixels_ui::{Field, Ink, fit, lump, ring};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status};

pub const INFO: Info = Info {
    id: "array-guard",
    name: "Array Guard",
    icon: ('◎', '@'),
    hue: Hue::Info,
    blurb: "clear debris from a dish array",
    hints: "← → move · ↑ F or Enter fire",
    help: &[
        "# Goal",
        "Protect a row of five satellite dishes from drifting debris for as many waves as you can.",
        "# Rules",
        "- Your cannon runs along the rail behind the dishes and fires straight up.",
        "- Small debris breaks with one shot. Large debris is a bigger rock that takes two and shrinks to a small one after the first.",
        "- Debris that reaches the bottom hits the nearest dish. Each dish has a shield of three, shown as lamps on its cabinet and as a bar in the panel. A dish with no shield left is lost to the next hit and its bowl falls from the mast.",
        "- Capsules drift down now and then, each with its letter on its face. Shoot one to collect it: S a spread shot for ten seconds, H one shield back on every dish and L a slow field for eight seconds.",
        "- A wave ends when its debris is cleared. The next one brings a little more.",
        "# Scoring",
        "10 points for small debris, 25 for large. Clearing a wave scores 100 times the wave plus 20 for every shield left.",
        "# Winning and losing",
        "There is no last wave. The game ends when every dish is lost.",
        "# Keys",
        "- ← → (or H L, A D) move the turret.",
        "- ↑, F or Enter fire, in upper or lower case. Up to four shots fly at once.",
        "Tip: large debris is slow; clear the small pieces first.",
    ],
    input: Input::Keys,
    unit: "points",
};

pub const W: i32 = 44;
pub const H: i32 = 14;
const DISHES: usize = 5;
const SHIELD: u8 = 3;
const RAIL: i32 = H - 3;
const COOLDOWN: f32 = 0.22;
const MAX_SHOTS: usize = 4;
const BURST: f32 = 0.36;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rock {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub speed: f32,
    pub hp: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Power {
    Spread,
    Shield,
    Slow,
}

impl Power {
    fn letter(self) -> char {
        match self {
            Power::Spread => 'S',
            Power::Shield => 'H',
            Power::Slow => 'L',
        }
    }
}

pub struct ArrayGuard {
    pub turret: i32,
    pub shots: Vec<(f32, f32, f32)>,
    pub rocks: Vec<Rock>,
    pub capsules: Vec<(f32, f32, Power)>,
    pub shields: [u8; DISHES],
    pub lost: [bool; DISHES],
    bursts: Vec<(f32, f32, f32)>,
    pub wave: u32,
    to_spawn: u32,
    spawn_in: f32,
    rest: f32,
    cooldown: f32,
    spread: f32,
    slow: f32,
    score: u64,
    rng: Rng,
    pace: f32,
}

fn dish_x(i: usize) -> i32 {
    4 + i as i32 * 9
}

impl ArrayGuard {
    pub fn new(setup: Setup) -> Self {
        let mut a = Self {
            turret: W / 2,
            shots: Vec::new(),
            rocks: Vec::new(),
            capsules: Vec::new(),
            shields: [SHIELD; DISHES],
            lost: [false; DISHES],
            bursts: Vec::new(),
            wave: 0,
            to_spawn: 0,
            spawn_in: 0.0,
            rest: 0.0,
            cooldown: 0.0,
            spread: 0.0,
            slow: 0.0,
            score: 0,
            rng: Rng::new(setup.seed),
            pace: if setup.reduced_motion { 0.5 } else { 1.0 },
        };
        a.next_wave();
        a.rest = 1.0;
        a
    }

    fn next_wave(&mut self) {
        self.wave += 1;
        self.to_spawn = 5 + 2 * self.wave;
        self.spawn_in = 0.5;
        self.rest = 2.0;
    }

    fn fire(&mut self, fx: &mut Fx) {
        if self.cooldown > 0.0 || self.shots.len() >= MAX_SHOTS {
            return;
        }
        let x = self.turret as f32 + 1.0;
        let y = RAIL as f32 - 1.0;
        self.shots.push((x, y, 0.0));
        if self.spread > 0.0 {
            self.shots.push((x, y, -6.0));
            self.shots.push((x, y, 6.0));
        }
        self.cooldown = COOLDOWN;
        fx.sound(Sfx::Tick);
    }

    fn alive(&self) -> usize {
        self.lost.iter().filter(|l| !**l).count()
    }

    fn hit_dish(&mut self, x: f32, fx: &mut Fx) {
        let near = (0..DISHES)
            .filter(|&i| !self.lost[i])
            .min_by_key(|&i| (dish_x(i) + 1 - x.round() as i32).abs());
        let Some(i) = near else { return };
        if (dish_x(i) + 1 - x.round() as i32).abs() > 3 {
            return;
        }
        if self.shields[i] > 0 {
            self.shields[i] -= 1;
            fx.say(format!("Dish {} hit: shield {}", i + 1, self.shields[i]));
        } else {
            self.lost[i] = true;
            fx.say(format!("Dish {} lost", i + 1));
        }
        fx.sound(Sfx::Low);
    }
}

/// Where a logical cell lands in the field's pixels at scale `k`: one logical cell is `k`
/// pixels wide and `2k` tall, so positions stay continuous at any scale.
#[derive(Clone, Copy)]
struct Scale(f32);

impl Scale {
    fn x(self, x: f32) -> f32 {
        (x + 0.5) * self.0
    }
    fn y(self, y: f32) -> f32 {
        (y * 2.0 + 1.0) * self.0
    }
    fn u(self) -> f32 {
        self.0
    }
}

impl ArrayGuard {
    /// The sky, the plain, the rail, the dishes on their masts, the cannon, debris, bolts,
    /// capsules and bursts. Everything is drawn at the field's scale, so the shapes grow with
    /// the window while the game keeps its own grid.
    fn paint_field(&self, f: &mut Field, ink: &Ink, k: Scale) {
        let pal = ink.pal;
        let (w, h) = (f.w(), f.h());
        let u = k.u();
        // a calm sky: night with still stars on a dark theme, a pale day on a light one
        let (top, low) = if ink.light() {
            (ink.tint(pal.blue, 0.4), ink.back())
        } else {
            (ink.back(), ink.tint(pal.blue, 0.5))
        };
        let horizon = k.y(H as f32 - 3.0);
        for y in 0..h {
            f.fill_row(y, ink.step(top, low, y as f32 / horizon, 10));
        }
        if !ink.light() {
            let star = ink.mix(ink.back(), pal.text, 0.35);
            let count = (w * h / 220).clamp(6, 40) as u32;
            for i in 0..count {
                let a = i.wrapping_mul(2_654_435_761) >> 9;
                let b = (i ^ 0x5bd1).wrapping_mul(2_246_822_519) >> 11;
                let y = (b % (horizon as u32 * 2 / 3).max(1)) as i32;
                f.set((a % w as u32) as i32, y, star);
            }
        }
        // far low hills on the horizon, then the ground the dishes stand on
        let ground_y = h - (u * 1.2).ceil() as i32;
        let hills = ink.tint(pal.green, if ink.light() { 0.35 } else { 0.55 });
        for x in 0..w {
            let t = x as f32 / u;
            let rise = ((t * 0.21).sin() * 0.6 + (t * 0.07 + 1.3).sin() * 0.4 + 1.0) * 1.2 * u;
            let top = ground_y - rise.round() as i32 - (u * 0.5) as i32;
            f.fill_rect(x, top, 1, ground_y - top, hills);
        }
        f.fill_rect(0, ground_y, w, h - ground_y, ink.soften(pal.green, 0.7));
        f.fill_rect(0, ground_y, w, 1, ink.soften(pal.green, 0.45));
        // the rail the cannon runs along, behind the dishes
        let rail_y = k.y(RAIL as f32) as i32 + u.round() as i32;
        let rail = ink.soften(pal.muted, 0.5);
        f.fill_rect(0, rail_y, w, (u * 0.5).ceil() as i32, rail);
        for i in 0..DISHES {
            self.paint_dish(f, ink, k, i, ground_y);
        }
        self.paint_cannon(f, ink, k, rail_y);
        for &(x, y, _) in &self.shots {
            let (sx, sy) = (k.x(x), k.y(y));
            let len = (1.8 * u).max(2.0);
            let glow = pal.info;
            let x0 = sx.floor() as i32;
            for py in (sy - len / 2.0) as i32..(sy + len / 2.0) as i32 {
                f.set(x0, py, pal.bright);
                if u >= 2.0 {
                    f.blend(x0 - 1, py, glow, 0.35);
                    f.blend(x0 + 1, py, glow, 0.35);
                }
            }
        }
        for rock in &self.rocks {
            let centre = (k.x(rock.x), k.y(rock.y));
            // the bumps turn as the rock falls, so it tumbles slowly
            let spin = rock.y * 0.5 + rock.x;
            let (colour, r) = if rock.hp >= 2 {
                (pal.red, 2.0 * u)
            } else {
                (pal.yellow, 1.3 * u)
            };
            let tones = [
                ink.highlight(colour, 0.35),
                colour,
                ink.shadow(colour, 0.45),
            ];
            lump(f, centre, r.max(1.4), tones, 0.14, spin);
        }
        self.paint_capsules(f, ink, k);
        // a burst is a ring that grows in three still steps over a third of a second
        for &(x, y, t) in &self.bursts {
            let centre = (k.x(x), k.y(y));
            let step = (t / BURST * 3.0) as i32;
            if step == 0 {
                f.disc(centre.0, centre.1, 0.8 * u, pal.bright);
            } else {
                ring(f, centre, (0.9 + step as f32) * u, pal.bright);
            }
        }
    }

    /// The letter cell and the pixel span of each capsule's face, shared by the shell and the
    /// letter so the letter always sits on the face.
    fn capsule_cells(&self, k: Scale) -> Vec<(i32, i32, i32, Power)> {
        self.capsules
            .iter()
            .map(|&(x, y, p)| {
                let col = k.x(x).floor() as i32;
                let row = ((k.y(y) - 1.0) / 2.0).round() as i32;
                let half = (1.5 * k.u()).ceil() as i32;
                (col, row, half, p)
            })
            .collect()
    }

    /// A capsule: a rounded magenta shell around a pale face that carries its letter.
    fn paint_capsules(&self, f: &mut Field, ink: &Ink, k: Scale) {
        let pal = ink.pal;
        let face = ink.surface(pal.magenta, 0.25);
        let shell = (k.u() * 0.6).ceil() as i32;
        for (col, row, half, _) in self.capsule_cells(k) {
            let (x0, x1) = (col - half - shell, col + half + shell);
            let (y0, y1) = (2 * row - shell, 2 * row + 1 + shell);
            for y in y0..=y1 {
                for x in x0..=x1 {
                    let corner = (x == x0 || x == x1) && (y == y0 || y == y1);
                    if !corner {
                        f.set(x, y, pal.magenta);
                    }
                }
            }
            f.fill_rect(col - half, 2 * row, 2 * half + 1, 2, face);
        }
    }

    /// The cannon: a wide base on the rail, a domed body and a barrel pointing up, lit from
    /// the left.
    fn paint_cannon(&self, f: &mut Field, ink: &Ink, k: Scale, rail_y: i32) {
        let pal = ink.pal;
        let u = k.u();
        let cx = k.x(self.turret as f32 + 1.0);
        let (lit, body, dark) = (
            ink.highlight(pal.info, 0.35),
            pal.info,
            ink.shadow(pal.info, 0.4),
        );
        let base_h = (0.8 * u).ceil().max(1.0) as i32;
        // the dome, then the barrel with a bright muzzle
        let dome_y = (rail_y - base_h) as f32;
        let r = 1.6 * u;
        lump(f, (cx, dome_y), r.max(1.2), [lit, body, dark], 0.0, 0.0);
        let bw = (0.9 * u).round().max(1.0) as i32;
        let bh = (2.6 * u).round().max(2.0) as i32;
        let bx = (cx - bw as f32 / 2.0).round() as i32;
        let by = (dome_y - r).round() as i32 - bh + 1;
        f.fill_rect(bx, by, bw, bh, body);
        if bw > 1 {
            f.fill_rect(bx, by, 1, bh, lit);
        }
        f.fill_rect(bx, by, bw, 1, pal.bright);
        // the base: a trapezoid on the rail, over the lower half of the dome
        for i in 0..base_h {
            let half = 2.6 * u - i as f32 * 0.5;
            let y = rail_y - 1 - i;
            let (x0, x1) = ((cx - half).round() as i32, (cx + half).round() as i32);
            f.fill_rect(x0, y, x1 - x0, 1, dark);
            f.fill_rect(x0, y, ((x1 - x0) / 2).max(1), 1, body);
        }
    }

    /// One radio dish: a tilted bowl with its feed on a mast, coloured by its shield, over a
    /// cabinet with three lamps, one for each point of shield left. A lost dish is a bowl
    /// fallen beside a broken mast.
    fn paint_dish(&self, f: &mut Field, ink: &Ink, k: Scale, i: usize, ground_y: i32) {
        let pal = ink.pal;
        let u = k.u();
        let cx = k.x(dish_x(i) as f32 + 1.0);
        let stand = ink.soften(pal.muted, 0.2);
        let stand_dark = ink.soften(pal.muted, 0.5);
        let mast_w = (0.6 * u).round().max(1.0) as i32;
        let mast_x = (cx - mast_w as f32 / 2.0).round() as i32;
        // the cabinet at the foot of the mast
        let box_w = (3.6 * u).round().max(3.0) as i32;
        let box_h = (1.0 * u).round().max(1.0) as i32 + 1;
        let box_x = (cx - box_w as f32 / 2.0).round() as i32;
        let box_y = ground_y - box_h;
        if self.lost[i] {
            f.fill_rect(
                mast_x,
                box_y - (u as i32).max(1),
                mast_w,
                (u as i32).max(1),
                stand_dark,
            );
            f.fill_rect(box_x, box_y, box_w, box_h, stand_dark);
            let bowl = ink.soften(pal.muted, 0.45);
            let tones = [bowl, bowl, stand_dark];
            bowl_shape(
                f,
                (cx + 2.2 * u, ground_y as f32 - 0.6 * u),
                2.4 * u,
                1.9,
                tones,
            );
            return;
        }
        let colour = match self.shields[i] {
            3 => pal.green,
            2 => pal.cyan,
            1 => pal.yellow,
            _ => pal.warning,
        };
        let mast_h = (2.6 * u).round() as i32;
        f.fill_rect(mast_x, box_y - mast_h, mast_w, mast_h, stand);
        if mast_w > 1 {
            f.fill_rect(mast_x + mast_w - 1, box_y - mast_h, 1, mast_h, stand_dark);
        }
        f.fill_rect(box_x, box_y, box_w, box_h, stand);
        f.fill_rect(box_x, box_y + box_h - 1, box_w, 1, stand_dark);
        // the lamps on the cabinet, bright for each point of shield left
        let lamp = (0.5 * u).round().max(1.0) as i32;
        let gap = (box_w - 3 * lamp) / 4;
        for s in 0..SHIELD as i32 {
            let lit = s < i32::from(self.shields[i]);
            let x = box_x + gap + s * (lamp + gap) + (box_w - 3 * lamp - 4 * gap) / 2;
            let colour = if lit { colour } else { stand_dark };
            f.fill_rect(x, box_y + (box_h - lamp) / 2, lamp, lamp, colour);
        }
        // the bowl, tilted to face the sky, with its feed arm and horn at the focus
        let top = (box_y - mast_h) as f32;
        let tilt = -0.5_f32;
        let r = 3.3 * u;
        let tones = [ink.highlight(colour, 0.35), colour, ink.shadow(colour, 0.4)];
        bowl_shape(f, (cx, top), r, tilt, tones);
        // the feed sits on the bowl's axis, above its open face
        let feed = ink.soften(pal.muted, 0.15);
        let (ax, ay) = (tilt.sin(), -tilt.cos());
        let (fx, fy) = (cx + ax * r * 0.95, top + ay * r * 0.95);
        f.line(
            (cx.round() as i32, top.round() as i32),
            (fx.round() as i32, fy.round() as i32),
            feed,
        );
        f.disc(fx, fy, (0.45 * u).max(0.6), feed);
    }
}

/// A dish bowl seen from the side: a parabola of half-width `r` around its lowest point
/// `vertex`, turned by `tilt` radians: `tones` are the lit face along the open side, the body
/// and the shaded back.
fn bowl_shape(f: &mut Field, vertex: (f32, f32), r: f32, tilt: f32, tones: [Rgb; 3]) {
    let [face, rim, back] = tones;
    let depth = r * 0.7;
    let thick = (r * 0.2).max(0.8);
    let reach = (r + depth).ceil() as i32 + 1;
    let (sin, cos) = tilt.sin_cos();
    let (x0, y0) = (vertex.0.floor() as i32, vertex.1.floor() as i32);
    for y in y0 - reach..=y0 + reach {
        for x in x0 - reach..=x0 + reach {
            let (dx, dy) = (x as f32 + 0.5 - vertex.0, y as f32 + 0.5 - vertex.1);
            // into the bowl's own frame: across the bowl and up out of it
            let across = dx * cos + dy * sin;
            let up = -(-dx * sin + dy * cos);
            if across.abs() > r {
                continue;
            }
            let curve = depth * (across / r).powi(2);
            if up < curve || up > depth {
                continue;
            }
            // the lit dish face along the open side, the back of the bowl shaded
            let colour = if up > depth - thick {
                face
            } else if up < curve + thick && across > 0.0 {
                back
            } else {
                rim
            };
            f.set(x, y, colour);
        }
    }
}

impl Game for ArrayGuard {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn natural_size(&self) -> Option<(i32, i32)> {
        Some((W + 2, H + 2))
    }

    /// The field grows with the window by itself, so a large window gets finer pixel art
    /// instead of a doubled small drawing.
    fn zoomable(&self) -> bool {
        false
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        match key {
            Key::Left | Key::Char('h' | 'H' | 'a' | 'A') => {
                self.turret = (self.turret - 1).max(0);
            }
            Key::Right | Key::Char('l' | 'L' | 'd' | 'D') => {
                self.turret = (self.turret + 1).min(W - 3);
            }
            Key::Up | Key::Enter | Key::Char('f' | 'F') => self.fire(fx),
            _ => {}
        }
    }

    fn tick(&mut self, dt: f32, fx: &mut Fx) {
        if self.alive() == 0 {
            return;
        }
        self.cooldown = (self.cooldown - dt).max(0.0);
        self.spread = (self.spread - dt).max(0.0);
        self.slow = (self.slow - dt).max(0.0);
        let pace = self.pace * if self.slow > 0.0 { 0.5 } else { 1.0 };
        for b in &mut self.bursts {
            b.2 += dt;
        }
        self.bursts.retain(|b| b.2 < BURST);
        // shots fly up; a spread shot drifts sideways as it goes
        for s in &mut self.shots {
            s.1 -= 18.0 * dt;
            s.0 += s.2 * dt;
        }
        self.shots
            .retain(|s| s.1 >= 0.0 && (0.0..W as f32).contains(&s.0));
        for r in &mut self.rocks {
            r.y += r.speed * dt * pace;
            r.x = (r.x + r.dx * dt * pace).clamp(0.0, W as f32 - 1.0);
        }
        for c in &mut self.capsules {
            c.1 += 1.2 * dt * pace;
        }
        // shots against debris and capsules
        let mut i = 0;
        while i < self.shots.len() {
            let (sx, sy, _) = self.shots[i];
            let hit_rock = self
                .rocks
                .iter()
                .position(|r| (r.x - sx).abs() < 1.0 && (r.y - sy).abs() < 1.0);
            if let Some(k) = hit_rock {
                self.shots.remove(i);
                let r = &mut self.rocks[k];
                r.hp -= 1;
                if r.hp == 0 {
                    let big = r.speed < 1.6;
                    let (x, y) = (r.x, r.y);
                    self.rocks.remove(k);
                    self.bursts.push((x, y, 0.0));
                    self.score += if big { 25 } else { 10 };
                    fx.sound(Sfx::Good);
                } else {
                    fx.sound(Sfx::Tap);
                }
                continue;
            }
            let hit_cap = self
                .capsules
                .iter()
                .position(|c| (c.0 - sx).abs() < 1.5 && (c.1 - sy).abs() < 1.0);
            if let Some(k) = hit_cap {
                self.shots.remove(i);
                let (x, y, power) = self.capsules.remove(k);
                self.bursts.push((x, y, 0.0));
                match power {
                    Power::Spread => {
                        self.spread = 10.0;
                        fx.say("Spread shot for ten seconds");
                    }
                    Power::Shield => {
                        for (s, lost) in self.shields.iter_mut().zip(self.lost) {
                            if !lost {
                                *s = (*s + 1).min(SHIELD);
                            }
                        }
                        fx.say("Shields topped up");
                    }
                    Power::Slow => {
                        self.slow = 8.0;
                        fx.say("Slow field for eight seconds");
                    }
                }
                fx.sound(Sfx::Win);
                continue;
            }
            i += 1;
        }
        // debris reaching the dishes
        let landed: Vec<f32> = self
            .rocks
            .iter()
            .filter(|r| r.y >= (H - 1) as f32)
            .map(|r| r.x)
            .collect();
        self.rocks.retain(|r| r.y < (H - 1) as f32);
        for x in landed {
            self.hit_dish(x, fx);
            self.bursts.push((x, (H - 2) as f32, 0.0));
        }
        self.capsules.retain(|c| c.1 < (H - 1) as f32);
        if self.alive() == 0 {
            return;
        }
        // the wave: a rest, then debris one piece at a time
        if self.rest > 0.0 {
            self.rest -= dt;
            if self.rest <= 0.0 && self.wave > 1 {
                fx.say(format!("Wave {}", self.wave));
            }
            return;
        }
        if self.to_spawn > 0 {
            self.spawn_in -= dt;
            if self.spawn_in <= 0.0 {
                self.to_spawn -= 1;
                let big = self.rng.chance(0.25 + 0.03 * self.wave.min(10) as f32);
                let base = 1.4 + 0.12 * self.wave.min(12) as f32;
                self.rocks.push(Rock {
                    x: self.rng.range(1.0, W as f32 - 2.0),
                    y: 0.0,
                    dx: self.rng.range(-0.8, 0.8),
                    speed: if big { base * 0.6 } else { base },
                    hp: if big { 2 } else { 1 },
                });
                if self.rng.chance(0.12) {
                    let power = *self.rng.pick(&[Power::Spread, Power::Shield, Power::Slow]);
                    self.capsules
                        .push((self.rng.range(2.0, W as f32 - 3.0), 0.0, power));
                }
                self.spawn_in = (1.6 - 0.08 * self.wave as f32).max(0.6);
            }
        } else if self.rocks.is_empty() {
            let shields: u64 = self.shields.iter().map(|s| u64::from(*s)).sum();
            let bonus = 100 * u64::from(self.wave) + 20 * shields;
            self.score += bonus;
            fx.say(format!("Wave {} cleared, +{bonus}", self.wave));
            fx.sound(Sfx::Win);
            self.next_wave();
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        // the side panel is left out when the window is too narrow for it; the status bar
        // carries what matters
        let panel_w = if area.w >= W + 2 + 2 + 22 { 22 } else { 0 };
        // the field grows with the window, keeping its grid; a small window keeps scale one
        let room = (
            area.w - 2 - if panel_w > 0 { panel_w + 2 } else { 0 },
            area.h - 2,
        );
        let k = fit(room, W, H, 3.0);
        let (fw, fh) = ((W as f32 * k).round() as i32, (H as f32 * k).round() as i32);
        let r = area.centre(fw + 2 + 2 + panel_w, fh + 2);
        let field = Rect::new(r.x, r.y, fw + 2, fh + 2);
        c.titled(
            field,
            Line::Heavy,
            pal.info,
            &format!("Wave {}", self.wave),
            pal.bright,
        );
        let inner = Rect::new(field.x + 1, field.y + 1, fw, fh);
        let ink = Ink::new(pal);
        let mut f = Field::new(pal, inner, ink.back());
        self.paint_field(&mut f, &ink, Scale(k));
        f.paint(c);
        // the capsule letters are text, set on the capsule's own pale face
        for (col, row, _, p) in self.capsule_cells(Scale(k)) {
            c.put(inner.x + col, inner.y + row, p.letter(), pal.magenta);
        }
        let p = Rect::new(field.x + field.w + 2, field.y, panel_w, field.h);
        if panel_w == 0 {
            return;
        }
        c.titled(p, Line::Light, pal.muted, "Array", pal.bright);
        for i in 0..DISHES {
            let y = p.y + 2 + i as i32;
            c.text(p.x + 2, y, &format!("Dish {}", i + 1), pal.muted);
            if self.lost[i] {
                c.text(p.x + 10, y, "lost", pal.warning);
            } else {
                let marks: String = (0..SHIELD)
                    .map(|k| if k < self.shields[i] { '█' } else { '·' })
                    .collect();
                c.text(p.x + 10, y, &marks, pal.green);
                c.text(p.x + 14, y, &self.shields[i].to_string(), pal.text);
            }
        }
        let mut y = p.y + 8;
        if self.spread > 0.0 {
            c.text(
                p.x + 2,
                y,
                &format!("[S] spread {:.0}s", self.spread),
                pal.magenta,
            );
            y += 1;
        }
        if self.slow > 0.0 {
            c.text(
                p.x + 2,
                y,
                &format!("[L] slow {:.0}s", self.slow),
                pal.magenta,
            );
            y += 1;
        }
        if self.rest > 0.0 {
            c.text(p.x + 2, y, &format!("Wave {} coming", self.wave), pal.info);
        }
        c.text(
            p.x + 2,
            p.y + p.h - 2,
            &format!(
                "{} ◆ {} ◇",
                self.rocks.iter().filter(|r| r.hp >= 2).count(),
                self.rocks.iter().filter(|r| r.hp == 1).count()
            ),
            pal.muted,
        );
    }

    fn status(&self) -> Status {
        vec![
            ("Score".into(), self.score.to_string()),
            ("Wave".into(), self.wave.to_string()),
            ("Dishes".into(), format!("{}/{DISHES}", self.alive())),
        ]
    }

    fn score(&self) -> u64 {
        self.score
    }

    fn outcome(&self) -> Option<Outcome> {
        (self.alive() == 0).then(|| Outcome::Lost(format!("The array fell in wave {}.", self.wave)))
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        let mut keys = vec![(Key::None, 3.5)];
        for _ in 0..4 {
            keys.push((Key::Left, 0.1));
            keys.push((Key::Up, 0.3));
        }
        keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> ArrayGuard {
        ArrayGuard::new(Setup {
            seed: 9,
            reduced_motion: false,
            day: 0,
            practice: false,
        })
    }

    #[test]
    fn a_shot_breaks_small_debris_and_large_takes_two() {
        let mut a = game();
        a.rest = 10.0;
        a.rocks = vec![Rock {
            x: a.turret as f32 + 1.0,
            y: 5.0,
            dx: 0.0,
            speed: 0.0,
            hp: 2,
        }];
        let mut fx = Fx::default();
        a.key(Key::Up, &mut fx);
        for _ in 0..30 {
            a.tick(0.05, &mut fx);
        }
        assert_eq!(a.rocks[0].hp, 1);
        a.key(Key::Char('F'), &mut fx);
        for _ in 0..30 {
            a.tick(0.05, &mut fx);
        }
        assert!(a.rocks.is_empty());
        assert_eq!(a.score, 25);
    }

    #[test]
    fn landing_debris_takes_a_shield_then_the_dish() {
        let mut a = game();
        a.rest = 100.0;
        let mut fx = Fx::default();
        for _ in 0..=SHIELD {
            a.rocks = vec![Rock {
                x: dish_x(0) as f32 + 1.0,
                y: (H - 1) as f32,
                dx: 0.0,
                speed: 0.0,
                hp: 1,
            }];
            a.tick(0.01, &mut fx);
        }
        assert!(a.lost[0]);
        assert_eq!(a.alive(), DISHES - 1);
    }

    #[test]
    fn a_shield_capsule_tops_up_every_dish() {
        let mut a = game();
        a.rest = 100.0;
        a.shields = [1; DISHES];
        a.capsules = vec![(a.turret as f32 + 1.0, 4.0, Power::Shield)];
        let mut fx = Fx::default();
        a.key(Key::Enter, &mut fx);
        for _ in 0..30 {
            a.tick(0.05, &mut fx);
        }
        assert_eq!(a.shields, [2; DISHES]);
    }

    #[test]
    fn every_dish_lost_ends_the_game() {
        let mut a = game();
        a.lost = [true; DISHES];
        assert!(matches!(a.outcome(), Some(Outcome::Lost(_))));
    }

    #[test]
    fn a_cleared_wave_starts_the_next() {
        let mut a = game();
        a.rest = 0.0;
        a.to_spawn = 0;
        a.rocks.clear();
        let mut fx = Fx::default();
        a.tick(0.01, &mut fx);
        assert_eq!(a.wave, 2);
    }
}
