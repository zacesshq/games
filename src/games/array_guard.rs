//! Array Guard: a calm but lively shooter. Debris drifts down towards a row of five satellite
//! dishes. A turret on the rail above them fires up to clear it. Each dish has a shield of three;
//! a hit takes one and a dish with no shield left is lost to the next hit. Capsules give a
//! spread shot, a shield top-up or a slow field. Waves grow a little each time. Bursts are rings
//! that grow over a third of a second and never blink.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, g};

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
        "- Your turret (◢▲◣) runs along the rail above the dishes and fires straight up.",
        "- Small debris (◇) breaks with one shot. Large debris (◆) takes two and shows ◇ after the first.",
        "- Debris that reaches the bottom hits the nearest dish. Each dish has a shield of three, drawn as a bar over it. A dish with no shield left is lost to the next hit (╳).",
        "- Capsules drift down now and then. Shoot one to collect it: [S] spread shot for ten seconds, [H] one shield back on every dish and [L] a slow field for eight seconds.",
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

impl Game for ArrayGuard {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn natural_size(&self) -> Option<(i32, i32)> {
        Some((W + 2, H + 2))
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
        let r = area.centre(W + 2 + 2 + panel_w, H + 2);
        let field = Rect::new(r.x, r.y, W + 2, H + 2);
        c.titled(
            field,
            Line::Heavy,
            pal.info,
            &format!("Wave {}", self.wave),
            pal.bright,
        );
        let at = |x: f32, y: f32| {
            (
                field.x + 1 + x.round() as i32,
                field.y + 1 + y.round() as i32,
            )
        };
        // a few fixed stars, so the field reads as sky
        for i in 0..14 {
            let x = (i * 17 + 5) % W;
            let y = (i * 7 + 2) % (H - 4);
            c.put(field.x + 1 + x, field.y + 1 + y, '·', pal.muted);
        }
        for i in 0..DISHES {
            let x = field.x + 1 + dish_x(i);
            let y = field.y + H;
            if self.lost[i] {
                c.text(x, y, "╳╳╳", pal.muted);
                continue;
            }
            let colour = match self.shields[i] {
                3 => pal.green,
                2 => pal.cyan,
                1 => pal.yellow,
                _ => pal.warning,
            };
            c.text(x, y, &format!("▟{}▙", g('◎', 'O')), colour);
            // the shield as a bar over the dish, one segment for each point left
            let arcs: String = (0..3)
                .map(|k| if k < self.shields[i] { '─' } else { ' ' })
                .collect();
            c.text(x, y - 1, &arcs, colour);
        }
        for x in 0..W {
            c.put(field.x + 1 + x, field.y + 1 + RAIL + 1, '┄', pal.muted);
        }
        let tx = field.x + 1 + self.turret;
        c.text(
            tx,
            field.y + 1 + RAIL,
            &format!("◢{}◣", g('▲', '^')),
            pal.info,
        );
        for &(x, y, _) in &self.shots {
            let (sx, sy) = at(x, y);
            c.put(sx, sy, '│', pal.bright);
        }
        for rock in &self.rocks {
            let (x, y) = at(rock.x, rock.y);
            let (glyph, colour) = if rock.hp >= 2 {
                (g('◆', '#'), pal.red)
            } else {
                (g('◇', 'o'), pal.yellow)
            };
            c.put(x, y, glyph, colour);
        }
        for &(x, y, p) in &self.capsules {
            let (cx, cy) = at(x, y);
            c.text(cx - 1, cy, &format!("[{}]", p.letter()), pal.magenta);
        }
        // a burst is a ring that grows in three still steps over a third of a second
        for &(x, y, t) in &self.bursts {
            let (bx, by) = at(x, y);
            let step = (t / BURST * 3.0) as i32;
            match step {
                0 => c.put(bx, by, '∙', pal.bright),
                1 => c.put(bx, by, g('○', 'o'), pal.bright),
                _ => {
                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        c.put(bx + dx, by + dy, '·', pal.bright);
                    }
                }
            }
        }
        let p = Rect::new(field.x + field.w + 2, field.y, panel_w, H + 2);
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
            p.y + H,
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
