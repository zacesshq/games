//! Slipstream: a calm top-down road race against the fuel gauge. The car runs up a four-lane road
//! with slower traffic and a pit lane on the right. Fuel burns faster the quicker the car goes.
//! Tucking in close behind another car (its slipstream) cuts the burn to well under half. The
//! pit lane refills the tank at walking pace. Three bumps or an empty tank end the run. Distance
//! is the score.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, g};

pub const INFO: Info = Info {
    id: "slipstream",
    name: "Slipstream",
    icon: ('▲', '^'),
    hue: Hue::Number,
    blurb: "a calm road race against fuel",
    hints: "← → lane · ↑ faster · ↓ slower · lane P refuels",
    help: &[
        "# Goal",
        "Drive as far as you can on one tank of fuel at a time.",
        "# Rules",
        "- Your car (▗▲▲▖) drives up a four-lane road past slower traffic.",
        "- Fuel burns faster the faster you go. Close behind another car, in its slipstream (≈), it burns at well under half.",
        "- The pit lane on the right (P) refills the tank, but holds the car to pit speed.",
        "- Running into a car is a bump: you slow down and the car moves on.",
        "# Scoring",
        "One point for every metre driven. A tone and a note mark every kilometre.",
        "# Winning and losing",
        "There is no finish line. The run ends after three bumps or when an empty tank coasts to a stop.",
        "# Keys",
        "- ← → (or H L) change lane.",
        "- ↑ (or K) goes faster, ↓ (or J) slower.",
        "Tip: tuck in behind a car before the fuel runs low, then pit when the road is clear.",
    ],
    input: Input::Keys,
    unit: "metres",
};

/// Driving lanes plus the pit lane at the right.
pub const LANES: usize = 5;
pub const PIT: usize = LANES - 1;
const LANE_W: i32 = 5;
const SHOULDER: i32 = 6;
pub const ROWS: i32 = 15;
/// The row the player's car sits on, counted from the top of the road.
pub const CAR_ROW: i32 = ROWS - 3;
const MAX_SPEED: f32 = 10.0;
const PIT_SPEED: f32 = 3.0;
const BUMPS: u32 = 3;
/// How far ahead a car still gives a slipstream, in rows.
const DRAFT_ROWS: f32 = 4.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Car {
    pub lane: usize,
    /// The car's top row on screen. It moves down as the player overtakes it.
    pub y: f32,
    pub speed: f32,
    pub colour: usize,
}

pub struct Slipstream {
    pub lane: usize,
    pub speed: f32,
    pub fuel: f32,
    pub bumps: u32,
    pub travelled: f32,
    pub cars: Vec<Car>,
    rng: Rng,
    grace: f32,
    drafting: bool,
    over: Option<String>,
    scale: f32,
    milestone: u32,
}

impl Slipstream {
    pub fn new(setup: Setup) -> Self {
        let mut s = Self {
            lane: 1,
            speed: 5.0,
            fuel: 100.0,
            bumps: 0,
            travelled: 0.0,
            cars: Vec::new(),
            rng: Rng::new(setup.seed),
            grace: 0.0,
            drafting: false,
            over: None,
            // reduced motion halves how fast the road scrolls; the rules stay the same
            scale: if setup.reduced_motion { 0.5 } else { 1.0 },
            milestone: 0,
        };
        for i in 0..3 {
            let lane = s.rng.below(PIT);
            let speed = s.rng.range(2.0, 4.5);
            s.cars.push(Car {
                lane,
                y: 1.0 + i as f32 * 4.0,
                speed,
                colour: i,
            });
        }
        s
    }

    /// The car directly ahead in the same lane, within slipstream reach.
    pub fn in_slipstream(&self) -> bool {
        self.lane != PIT
            && self.cars.iter().any(|c| {
                let gap = CAR_ROW as f32 - (c.y + 2.0);
                c.lane == self.lane && (0.0..=DRAFT_ROWS).contains(&gap)
            })
    }

    /// Fuel burned in a second at the current speed.
    pub fn burn(&self) -> f32 {
        let base = 0.6 + self.speed * 0.25;
        if self.in_slipstream() {
            base * 0.4
        } else {
            base
        }
    }

    fn spawn(&mut self) {
        // a new car comes in above the screen in a lane that leaves at least one lane free there
        let near: Vec<usize> = self
            .cars
            .iter()
            .filter(|c| c.y < 3.0)
            .map(|c| c.lane)
            .collect();
        let free: Vec<usize> = (0..PIT).filter(|l| !near.contains(l)).collect();
        if free.len() < 2 {
            return;
        }
        let lane = *self.rng.pick(&free);
        let speed = self.rng.range(2.0, 4.5);
        let colour = self.rng.below(4);
        self.cars.push(Car {
            lane,
            y: -2.0,
            speed,
            colour,
        });
    }

    fn end(&mut self, why: &str, fx: &mut Fx) {
        self.over = Some(why.into());
        fx.sound(Sfx::Low);
    }
}

impl Game for Slipstream {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn natural_size(&self) -> Option<(i32, i32)> {
        Some((LANES as i32 * LANE_W + SHOULDER * 2 + 2, ROWS + 2))
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        if self.over.is_some() {
            return;
        }
        match key {
            Key::Left | Key::Char('h' | 'H') if self.lane > 0 => {
                self.lane -= 1;
                fx.sound(Sfx::Tap);
            }
            Key::Right | Key::Char('l' | 'L') if self.lane < PIT => {
                self.lane += 1;
                fx.sound(Sfx::Tap);
                if self.lane == PIT {
                    fx.say("Into the pit lane");
                }
            }
            Key::Up | Key::Char('k' | 'K') => {
                if self.lane == PIT {
                    fx.say("Pit speed: leave the pit lane to go faster");
                } else if self.fuel > 0.0 {
                    self.speed = (self.speed + 1.0).min(MAX_SPEED);
                    fx.sound(Sfx::Tick);
                }
            }
            Key::Down | Key::Char('j' | 'J') => {
                self.speed = (self.speed - 1.0).max(1.0);
                fx.sound(Sfx::Tick);
            }
            _ => {}
        }
    }

    fn tick(&mut self, dt: f32, fx: &mut Fx) {
        if self.over.is_some() {
            return;
        }
        if self.lane == PIT {
            self.speed = self.speed.min(PIT_SPEED);
            let before = self.fuel;
            self.fuel = (self.fuel + 25.0 * dt).min(100.0);
            if before < 100.0 && self.fuel >= 100.0 {
                fx.sound(Sfx::Good);
                fx.say("Tank full");
            }
        } else {
            self.fuel = (self.fuel - self.burn() * dt).max(0.0);
        }
        if self.fuel <= 0.0 {
            // an empty tank coasts to a stop
            self.speed = (self.speed - 2.0 * dt).max(0.0);
            if self.speed <= 0.0 {
                self.end("Out of fuel", fx);
                return;
            }
        }
        let drafting = self.in_slipstream();
        if drafting && !self.drafting {
            fx.say("In the slipstream: fuel lasts longer");
        }
        self.drafting = drafting;
        let step = self.speed * dt * self.scale;
        self.travelled += step;
        let metres = (self.travelled * 10.0) as u32;
        if metres / 1000 > self.milestone {
            self.milestone = metres / 1000;
            fx.sound(Sfx::Win);
            fx.say(format!("{} km", self.milestone));
        }
        let scale = self.scale;
        let mine = self.speed;
        for c in &mut self.cars {
            c.y += (mine - c.speed) * dt * scale;
        }
        self.cars.retain(|c| c.y < ROWS as f32 + 1.0 && c.y > -4.0);
        if self.cars.len() < 4 && self.rng.chance(dt * 0.8) {
            self.spawn();
        }
        self.grace = (self.grace - dt).max(0.0);
        if self.grace > 0.0 {
            return;
        }
        let top = CAR_ROW as f32;
        let lane = self.lane;
        if let Some(hit) = self
            .cars
            .iter_mut()
            .find(|c| c.lane == lane && c.y + 2.0 > top && c.y < top + 2.0)
        {
            hit.y = top - 3.0;
            let slower = (hit.speed - 1.0).max(1.0);
            self.speed = self.speed.min(slower);
            self.bumps += 1;
            self.grace = 1.5;
            if self.bumps >= BUMPS {
                self.end("Three bumps: the car is out", fx);
            } else {
                fx.sound(Sfx::Low);
                fx.say(format!("Bump: {} of {BUMPS}", self.bumps));
            }
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        let road_w = LANES as i32 * LANE_W;
        let box_w = road_w + SHOULDER * 2 + 2;
        let box_h = ROWS + 2;
        // the side panel is left out when the window is too narrow for it; the status bar
        // carries what matters
        let panel_w = if area.w >= box_w + 3 + 22 { 22 } else { 0 };
        let r = area.centre(box_w + 3 + panel_w, box_h);
        let frame = Rect::new(r.x, r.y, box_w, box_h);
        c.titled(frame, Line::Heavy, pal.number, "Coast road", pal.bright);
        let road_x = frame.x + 1 + SHOULDER;
        let scroll = self.travelled;
        for row in 0..ROWS {
            let y = frame.y + 1 + row;
            // the world row under this screen row, so scenery and markings move with the road
            let world = (scroll.floor() as i64) - i64::from(row);
            // the verges: a tree, a post or grass, picked from the world row
            for side in 0..2 {
                let x0 = if side == 0 {
                    frame.x + 1
                } else {
                    road_x + road_w
                };
                let h = (world.wrapping_mul(2_654_435_761) >> 7).rem_euclid(9) + side;
                let (glyph, colour) = match h {
                    0 | 5 => (g('♣', '*'), pal.green),
                    3 => ('╿', pal.string),
                    _ => (' ', pal.muted),
                };
                let at = x0 + 1 + (h % 4) as i32;
                c.put(at, y, glyph, colour);
            }
            c.put(road_x - 1, y, '▏', pal.muted);
            c.put(road_x + road_w, y, '▕', pal.muted);
            for lane in 1..LANES as i32 {
                let x = road_x + lane * LANE_W - 1;
                if lane as usize == PIT {
                    c.put(x, y, '┃', pal.warning);
                } else if world.rem_euclid(3) != 0 {
                    c.put(x, y, '┆', pal.muted);
                }
            }
        }
        // the pit lane's sign and fuel marks
        let pit_x = road_x + PIT as i32 * LANE_W + 1;
        c.text(pit_x, frame.y + 1, " P ", pal.warning);
        for row in (3..ROWS).step_by(4) {
            c.put(pit_x + 1, frame.y + 1 + row, '+', pal.warning);
        }
        let lane_x = |lane: usize| road_x + lane as i32 * LANE_W;
        let car_colours = [pal.cyan, pal.magenta, pal.yellow, pal.blue];
        for car in &self.cars {
            let y = frame.y + 1 + car.y.round() as i32;
            let x = lane_x(car.lane);
            let colour = car_colours[car.colour % car_colours.len()];
            if (0..ROWS).contains(&(car.y.round() as i32)) {
                c.text(x, y, "▐▀▀▌", colour);
            }
            if (0..ROWS).contains(&(car.y.round() as i32 + 1)) {
                c.text(x, y + 1, "▐▄▄▌", colour);
            }
        }
        let px = lane_x(self.lane);
        let py = frame.y + 1 + CAR_ROW;
        if self.drafting {
            for gap in 1..=2 {
                c.put(px + 1, py - gap, '≈', pal.muted);
                c.put(px + 2, py - gap, '≈', pal.muted);
            }
        }
        let body = if self.grace > 0.0 {
            pal.warning
        } else {
            pal.number
        };
        c.text(px, py, &format!("▗{}{}▖", g('▲', '^'), g('▲', '^')), body);
        c.text(px, py + 1, "▐██▌", body);

        let p = Rect::new(frame.x + box_w + 3, frame.y, panel_w, box_h);
        if panel_w == 0 {
            return;
        }
        c.titled(p, Line::Light, pal.muted, "Dash", pal.bright);
        c.text(p.x + 2, p.y + 2, "Speed", pal.muted);
        c.meter(
            p.x + 2,
            p.y + 3,
            12,
            self.speed / MAX_SPEED,
            pal.number,
            pal.muted,
        );
        c.text(
            p.x + 15,
            p.y + 3,
            &format!("{:.0}", self.speed * 20.0),
            pal.text,
        );
        c.text(p.x + 2, p.y + 5, "Fuel", pal.muted);
        let fuel_colour = if self.fuel > 25.0 {
            pal.green
        } else {
            pal.warning
        };
        c.meter(
            p.x + 2,
            p.y + 6,
            12,
            self.fuel / 100.0,
            fuel_colour,
            pal.muted,
        );
        c.text(p.x + 15, p.y + 6, &format!("{:.0}%", self.fuel), pal.text);
        if self.fuel <= 25.0 {
            c.text(p.x + 2, p.y + 7, "Low: pit lane P", pal.warning);
        }
        c.text(p.x + 2, p.y + 9, "Bumps", pal.muted);
        for i in 0..BUMPS {
            let (glyph, colour) = if i < self.bumps {
                (g('✕', 'x'), pal.warning)
            } else {
                (g('○', 'o'), pal.muted)
            };
            c.put(p.x + 9 + i as i32 * 2, p.y + 9, glyph, colour);
        }
        let draft = if self.drafting {
            "Slipstream ≈ on"
        } else {
            "Slipstream off"
        };
        c.text(
            p.x + 2,
            p.y + 11,
            draft,
            if self.drafting { pal.number } else { pal.muted },
        );
        c.text(
            p.x + 2,
            p.y + 13,
            &format!("{:.0} m", self.travelled * 10.0),
            pal.bright,
        );
    }

    fn status(&self) -> Status {
        vec![
            ("Distance".into(), format!("{:.0} m", self.travelled * 10.0)),
            ("Fuel".into(), format!("{:.0}%", self.fuel)),
            ("Bumps".into(), format!("{}/{BUMPS}", self.bumps)),
        ]
    }

    fn score(&self) -> u64 {
        (self.travelled * 10.0) as u64
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
            .as_ref()
            .map(|why| Outcome::Lost(format!("{why} after {:.0} metres.", self.travelled * 10.0)))
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        vec![(Key::Up, 0.5), (Key::Right, 1.5), (Key::Up, 1.0)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> Slipstream {
        Slipstream::new(Setup {
            seed: 6,
            reduced_motion: false,
            day: 0,
            practice: false,
        })
    }

    #[test]
    fn the_slipstream_cuts_the_burn() {
        let mut s = game();
        s.cars = vec![Car {
            lane: s.lane,
            y: CAR_ROW as f32 - 4.0,
            speed: 4.0,
            colour: 0,
        }];
        assert!(s.in_slipstream());
        let drafting = s.burn();
        s.cars[0].lane = (s.lane + 1) % PIT;
        assert!(!s.in_slipstream());
        assert!(drafting < s.burn() * 0.5);
    }

    #[test]
    fn the_pit_lane_refills_at_pit_speed() {
        let mut s = game();
        s.cars.clear();
        s.fuel = 10.0;
        s.speed = 8.0;
        let mut fx = Fx::default();
        for _ in 0..PIT {
            s.key(Key::Right, &mut fx);
        }
        assert_eq!(s.lane, PIT);
        s.tick(1.0, &mut fx);
        assert!(s.fuel > 30.0);
        assert!(s.speed <= PIT_SPEED);
        s.key(Key::Up, &mut fx);
        assert!(s.speed <= PIT_SPEED);
    }

    #[test]
    fn three_bumps_end_the_run() {
        let mut s = game();
        let mut fx = Fx::default();
        for _ in 0..BUMPS {
            s.grace = 0.0;
            s.cars = vec![Car {
                lane: s.lane,
                y: CAR_ROW as f32,
                speed: 2.0,
                colour: 0,
            }];
            s.tick(0.01, &mut fx);
        }
        assert_eq!(s.bumps, BUMPS);
        assert!(matches!(s.outcome(), Some(Outcome::Lost(_))));
    }

    #[test]
    fn an_empty_tank_coasts_to_a_stop() {
        let mut s = game();
        s.cars.clear();
        s.fuel = 0.0;
        let mut fx = Fx::default();
        for _ in 0..100 {
            s.tick(0.1, &mut fx);
        }
        assert!(matches!(s.outcome(), Some(Outcome::Lost(_))));
    }

    #[test]
    fn reduced_motion_halves_the_scroll() {
        let mut a = game();
        let mut b = Slipstream::new(Setup {
            seed: 6,
            reduced_motion: true,
            day: 0,
            practice: false,
        });
        a.cars.clear();
        b.cars.clear();
        let mut fx = Fx::default();
        a.tick(1.0, &mut fx);
        b.tick(1.0, &mut fx);
        assert!((a.travelled - 2.0 * b.travelled).abs() < 1e-3);
    }
}
