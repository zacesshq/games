//! Slipstream: a calm top-down road race against the fuel gauge. The car runs up a four-lane road
//! with slower traffic and a pit lane on the right. Fuel burns faster the quicker the car goes.
//! Tucking in close behind another car (its slipstream) cuts the burn to well under half. The
//! pit lane refills the tank at walking pace. Three bumps or an empty tank end the run. Distance
//! is the score.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::pixels_ui::{Field, Ink, SOLID, text_on};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, g};
use zacess_scenes::colour::Rgb;
use zacess_scenes::pixels::Shades;

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
        "- Your car, the one with a light stripe down its middle, drives up a four-lane road past slower traffic.",
        "- Fuel burns faster the faster you go. Close behind another car, in its slipstream (streaks of air run past your car), it burns at well under half.",
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
/// Below this much fuel the gauge shows the warning colour and the warning tone sounds.
const LOW_FUEL: f32 = 25.0;
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
    /// Whether the fuel warning has sounded since the tank was last above a quarter.
    fuel_low: bool,
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
            fuel_low: false,
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

    fn zoomable(&self) -> bool {
        // the road grows its own pixels with the window, which keeps the cars sharp
        false
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
        // the gauge turns to the warning colour below a quarter; the tone sounds once as it does
        let low = self.fuel <= LOW_FUEL;
        if low && !self.fuel_low && self.lane != PIT {
            fx.sound(Sfx::Warn);
            fx.say("Fuel low: the pit lane is on the right");
        }
        self.fuel_low = low;
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
        let geo = Geometry::fit(area);
        let box_w = geo.road_w() + geo.shoulder * 2 + 2;
        let rows = (ROWS * geo.row + 1) / 2;
        let box_h = rows + 2;
        // the side panel is left out when the window is too narrow for it; the status bar
        // carries what matters
        let panel_w = if area.w >= box_w + 3 + 22 { 22 } else { 0 };
        let r = area.centre(box_w + 3 + panel_w, box_h.max(15));
        let frame = Rect::new(r.x, r.y, box_w, box_h);
        c.titled(frame, Line::Heavy, pal.number, "Coast road", pal.bright);
        self.paint_road(c, Rect::new(frame.x + 1, frame.y + 1, box_w - 2, rows), geo);

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
        let fuel_colour = if self.fuel > LOW_FUEL {
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

/// How large the road is drawn: pixels to a row of the road, the width of a lane, the verges
/// and the size of a car, all grown together with the window.
#[derive(Clone, Copy, Debug)]
struct Geometry {
    row: i32,
    lane: i32,
    shoulder: i32,
    car_w: i32,
    car_h: i32,
}

impl Geometry {
    fn fit(area: Rect) -> Self {
        let row = ((area.h - 2) * 2 / ROWS).clamp(2, 6);
        let car_h = 2 * row + 3;
        let car_w = row + 3;
        let lane = car_w + 2;
        let room = area.w - 2 - 3 - 22 - LANES as i32 * lane;
        let shoulder = (room / 2).clamp(SHOULDER, lane * 2);
        Self {
            row,
            lane,
            shoulder,
            car_w,
            car_h,
        }
    }

    fn road_w(&self) -> i32 {
        LANES as i32 * self.lane
    }
}

impl Slipstream {
    /// The road from above as pixel art: grass verges with trees, the asphalt with its lane
    /// markings, the pit lane and the cars, all moving with the distance driven so the road
    /// glides down the screen a pixel at a time.
    fn paint_road(&self, c: &mut Canvas, inner: Rect, g: Geometry) {
        let pal = c.pal;
        let ink = Ink::new(pal);
        let shades = Shades::new(pal.theme);
        let grass = ink.tint(pal.green, 0.22);
        let stripe = ink.tint(pal.green, 0.27);
        let asphalt = ink.under(pal.muted, 0.2, &[pal.warning]);
        let paint = ink.tint(pal.text, 0.75);
        let dash = ink.tint(pal.text, 0.5);
        let pit_ground = ink.under(pal.warning, 0.16, &[pal.warning]);
        let mut layer = Field::new(pal, inner, grass);
        let (lw, lh) = (layer.w(), layer.h());
        let s = g.row;
        // the screen pixel row of a world pixel, so every mark moves with the road
        let scroll = self.travelled * s as f32;
        let py_of = |world: f32| (scroll - world).round() as i32;
        let base = scroll.floor() as i32;
        let road = g.shoulder;
        let road_w = g.road_w();
        // mowing stripes across the verges
        for y in 0..lh {
            if (base - y).rem_euclid(4 * s) < 2 * s {
                layer.px.fill_rect(0, y, road, 1, stripe);
                layer.px.fill_rect(road + road_w, y, lw, 1, stripe);
            }
        }
        // trees on the verges, round crowns seen from above, lit from the top left
        let radius = s as f32 + 0.6;
        let spacing = 5 * s + 2;
        for side in 0..2_i64 {
            for k in (base - lh - spacing).div_euclid(spacing)..=base.div_euclid(spacing) + 1 {
                let h = (k as i64 * 2_654_435_761 + side * 40_503).rem_euclid(1 << 16) as i32;
                if h % 5 == 0 {
                    continue;
                }
                let span = (road - 2 * radius as i32 - 1).max(1);
                let cx = if side == 0 {
                    radius + ((h / 3) % span) as f32
                } else {
                    (road + road_w + 1) as f32 + radius + ((h / 3) % span) as f32
                };
                let cy = py_of((k * spacing + h % s.max(2)) as f32) as f32;
                crown(&mut layer, &ink, pal.green, cx, cy, radius);
            }
        }
        layer.px.fill_rect(road, 0, road_w, lh, asphalt);
        let pit_x = road + PIT as i32 * g.lane;
        layer.px.fill_rect(pit_x, 0, g.lane, lh, pit_ground);
        // edge lines and the dashed lane lines
        layer.px.fill_rect(road - 1, 0, 1, lh, paint);
        layer.px.fill_rect(road + road_w, 0, 1, lh, paint);
        for y in 0..lh {
            if (base - y).rem_euclid(4 * s) < 2 * s {
                for lane in 1..PIT as i32 {
                    layer.px.set(road + lane * g.lane - 1, y, dash);
                }
            }
        }
        // the pit lane wall is a solid line in the warning colour with fuel marks on its floor
        let wall = ink.solid(pal.warning);
        layer.px.fill_rect(pit_x - 1, 0, 1, lh, wall);
        let arm = if s >= 3 { 2 } else { 1 };
        let step = 5 * s;
        for k in (base - lh - step).div_euclid(step)..=base.div_euclid(step) + 1 {
            let y = py_of((k * step) as f32);
            let cx = pit_x + g.lane / 2;
            if y > 2 * s + arm {
                layer.px.fill_rect(cx - arm, y, 2 * arm + 1, 1, wall);
                layer.px.fill_rect(cx, y - arm, 1, 2 * arm + 1, wall);
            }
        }
        let lane_x = |lane: usize| road + lane as i32 * g.lane + (g.lane - 1 - g.car_w) / 2;
        // a car's two rows of the road, with the body centred on them
        let top_of = |row: f32| (row * s as f32).round() as i32 - (g.car_h - 2 * s) / 2;
        let car_colours = [pal.cyan, pal.magenta, pal.yellow, pal.blue];
        for car in &self.cars {
            let colour = car_colours[car.colour % car_colours.len()];
            car_sprite(
                &mut layer,
                &ink,
                &shades,
                lane_x(car.lane),
                top_of(car.y),
                g,
                colour,
                false,
            );
        }
        let px = lane_x(self.lane);
        let py = top_of(CAR_ROW as f32);
        if self.drafting {
            // air streaming past in the slipstream, drawn as soft streaks that run down
            let air = ink.tint(pal.text, 0.45);
            let phase = (scroll * 1.5).floor() as i32;
            let ahead = self
                .cars
                .iter()
                .filter(|c| c.lane == self.lane && top_of(c.y) < py)
                .map(|c| top_of(c.y) + g.car_h)
                .max()
                .unwrap_or(py - 4 * s);
            let len = s;
            for k in 0..4 {
                let y = py - 2 - (k * 2 * s + phase).rem_euclid(8 * s);
                if y > ahead {
                    let x = px + 1 + (k % 2) * (g.car_w - 3);
                    layer.px.fill_rect(x, y - len + 1, 1, len, air);
                }
            }
        }
        let body = if self.grace > 0.0 {
            pal.warning
        } else {
            pal.number
        };
        car_sprite(&mut layer, &ink, &shades, px, py, g, body, true);
        layer.paint(c);
        text_on(
            c,
            inner.x + pit_x + (g.lane - 3) / 2,
            inner.y,
            " P ",
            pal.warning,
        );
    }
}

/// A tree crown from above: a disc in tints of green, lighter to the top left and deeper to the
/// bottom right.
fn crown(layer: &mut Field, ink: &Ink, green: Rgb, cx: f32, cy: f32, r: f32) {
    let tones = [
        ink.tint(green, 0.95),
        ink.tint(green, 0.75),
        ink.tint(green, 0.5),
    ];
    let reach = r.ceil() as i32;
    for dy in -reach..=reach {
        for dx in -reach..=reach {
            let (fx, fy) = (dx as f32 + 0.5 - 0.5, dy as f32);
            if fx * fx + fy * fy > r * r {
                continue;
            }
            let light = -(fx + fy) / (2.0 * r);
            let tone = if light > 0.3 {
                tones[0]
            } else if light > -0.3 {
                tones[1]
            } else {
                tones[2]
            };
            layer
                .px
                .set(cx.round() as i32 + dx, cy.round() as i32 + dy, tone);
        }
    }
}

/// A car from above, `car_w` by `car_h` pixels with its top left at `x`, `y`: tyres standing
/// out at both axles, a rounded nose and tail, a windscreen and a rear window, the body lit
/// from the top left. The player's car carries a light stripe down its middle so it reads apart
/// from the traffic without colour.
#[allow(clippy::too_many_arguments)]
fn car_sprite(
    layer: &mut Field,
    ink: &Ink,
    shades: &Shades,
    x: i32,
    y: i32,
    g: Geometry,
    colour: Rgb,
    player: bool,
) {
    let (w, h) = (g.car_w, g.car_h);
    let body = ink.solid(colour);
    let (lit, dark) = ink.lit(colour);
    let glass = shades.held(body, shades.darkest(), 0.6, SOLID);
    let tyre = ink.shade(ink.solid(colour), -1.0);
    let tyre = shades.held(tyre, shades.darkest(), 0.8, SOLID);
    let stripe = ink.shade(lit, 0.45);
    let tyre_len = (h / 5).max(1);
    let front = (h / 6).max(1);
    let rear = h - front - tyre_len;
    let screen = (h / 4).max(1);
    let screen_len = (h / 5).max(1);
    let back = (h * 7 / 10).min(h - 2);
    for dy in 0..h {
        for dx in 0..w {
            let edge = dx == 0 || dx == w - 1;
            let colour = if edge {
                // the tyres sit outside the body at each axle
                let axle = (front..front + tyre_len).contains(&dy)
                    || (rear..rear + tyre_len).contains(&dy);
                if !axle {
                    continue;
                }
                tyre
            } else {
                let (inner_l, inner_r) = (dx == 1, dx == w - 2);
                let corner = (dy == 0 || dy == h - 1) && (inner_l || inner_r);
                if corner {
                    continue;
                }
                let glass_row = (screen..screen + screen_len).contains(&dy) || dy == back;
                let middle = !inner_l && !inner_r;
                if glass_row && (middle || w <= 5) && !(w <= 5 && (inner_l || inner_r)) {
                    glass
                } else if player
                    && middle
                    && (dx - (w - 1) / 2).abs() <= (w - 5).max(0) / 2
                    && w > 4
                {
                    stripe
                } else if dy == 0 || inner_l {
                    lit
                } else if dy == h - 1 || inner_r {
                    dark
                } else {
                    body
                }
            };
            layer.px.set(x + dx, y + dy, colour);
        }
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
    fn the_fuel_warning_sounds_once_as_the_gauge_turns() {
        let mut s = game();
        s.cars.clear();
        s.fuel = LOW_FUEL + 0.5;
        s.speed = 8.0;
        let mut fx = Fx::default();
        let mut warned = 0;
        for _ in 0..40 {
            s.tick(0.1, &mut fx);
            warned += fx.sounds.iter().filter(|&&x| x == Sfx::Warn).count();
            fx.sounds.clear();
        }
        assert!(s.fuel < LOW_FUEL);
        assert_eq!(warned, 1);
        assert_eq!(
            fx.toast.as_deref(),
            Some("Fuel low: the pit lane is on the right")
        );
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
