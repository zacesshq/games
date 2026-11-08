//! Signal: a calm rhythm game. Notes drift along three lanes towards a line on the left, timed to
//! a slow melody. Press a note's lane as it reaches the line and its tone sounds. Nothing flashes:
//! a caught note leaves a soft glow that shrinks over half a second. A missed note simply passes.
//! There is no penalty for a press with no note, so a careful player is never punished.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::pixels_ui::{Field, Ink, text_on};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, g};
use zacess_scenes::colour::Rgb;

pub const INFO: Info = Info {
    id: "signal",
    name: "Signal",
    icon: ('◎', 'o'),
    hue: Hue::Blue,
    blurb: "a calm rhythm of soft tones",
    hints: "↑ or 1 high · → or 2 middle · ↓ or 3 low",
    help: &[
        "# Goal",
        "Catch every note of a calm song as it reaches the line.",
        "# Rules",
        "- Notes drift left along three lanes: high notes point up, middle notes are round and low notes point down.",
        "- Press a note's lane key as it crosses the outline of its shape on the left. A caught note plays its tone, fills the outline and leaves a soft glow.",
        "- A press with no note near costs nothing. A note that passes the line is missed.",
        "# Scoring",
        "300 points for a note in time, 100 for a close one, plus 10 for each note of the current run, up to 200.",
        "# Winning and losing",
        "The song ends when every note has passed. There is no way to lose; the end screen shows how many were in time, close and missed.",
        "# Keys",
        "- ↑ or 1 plays the high lane.",
        "- → or 2 plays the middle lane.",
        "- ↓ or 3 plays the low lane.",
        "Tip: watch the notes rather than listen; the tones are a garnish and the song plays the same muted.",
    ],
    input: Input::Keys,
    unit: "points",
};

/// Seconds a note takes from the right edge to the line.
const TRAVEL: f32 = 3.0;
const IN_TIME: f32 = 0.12;
const CLOSE: f32 = 0.25;
/// The pentatonic steps a melody walks, in semitones from A4.
const SCALE: [i8; 10] = [-12, -10, -7, -5, -3, 0, 2, 5, 7, 9];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Note {
    /// When it reaches the line, in seconds from the start.
    pub at: f32,
    pub lane: usize,
    pub pitch: i8,
    pub result: Option<Hit>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hit {
    InTime,
    Close,
    Missed,
}

pub struct Signal {
    pub notes: Vec<Note>,
    pub time: f32,
    score: u64,
    combo: u32,
    best_combo: u32,
    last: Option<(Hit, f32)>,
    rings: Vec<(usize, f32)>,
    pub beat: f32,
}

/// The lane a scale step plays in: high, middle or low, so the lanes follow the melody's shape.
fn lane_of(step: usize) -> usize {
    match step {
        6.. => 0,
        3..=5 => 1,
        _ => 2,
    }
}

/// A song: a melody walking the scale, its lane following the pitch, with rests so it breathes.
pub fn compose(seed: u64, beat: f32) -> Vec<Note> {
    let mut rng = Rng::new(seed);
    let mut notes = Vec::new();
    let mut step = 5usize;
    let mut t = 2.0 + TRAVEL;
    for bar in 0..12 {
        for b in 0..4 {
            // the first beat of a bar always sounds; others rest about a third of the time
            if b != 0 && rng.chance(0.35) {
                t += beat;
                continue;
            }
            let walk = rng.below(5) as i32 - 2;
            step = (step as i32 + walk).clamp(0, SCALE.len() as i32 - 1) as usize;
            let pitch = SCALE[step];
            let lane = lane_of(step);
            notes.push(Note {
                at: t,
                lane,
                pitch,
                result: None,
            });
            // a half beat now and then in the later bars, never two lanes at once
            if bar >= 6 && b == 2 && rng.chance(0.3) {
                let up = (step + 1).min(SCALE.len() - 1);
                notes.push(Note {
                    at: t + beat / 2.0,
                    lane: if up >= 7 {
                        0
                    } else if up >= 4 {
                        1
                    } else {
                        2
                    },
                    pitch: SCALE[up],
                    result: None,
                });
            }
            t += beat;
        }
    }
    notes
}

impl Signal {
    pub fn new(setup: Setup) -> Self {
        // 72 beats a minute; 54 with reduced motion
        let beat = if setup.reduced_motion {
            60.0 / 54.0
        } else {
            60.0 / 72.0
        };
        Self {
            notes: compose(setup.seed, beat),
            time: 0.0,
            score: 0,
            combo: 0,
            best_combo: 0,
            last: None,
            rings: Vec::new(),
            beat,
        }
    }

    fn press(&mut self, lane: usize, fx: &mut Fx) {
        let time = self.time;
        let best = self
            .notes
            .iter_mut()
            .filter(|n| n.lane == lane && n.result.is_none() && (n.at - time).abs() <= CLOSE)
            .min_by(|a, b| (a.at - time).abs().total_cmp(&(b.at - time).abs()));
        let Some(note) = best else { return };
        let off = (note.at - time).abs();
        let hit = if off <= IN_TIME {
            Hit::InTime
        } else {
            Hit::Close
        };
        note.result = Some(hit);
        let pitch = note.pitch;
        self.combo += 1;
        self.best_combo = self.best_combo.max(self.combo);
        let points = if hit == Hit::InTime { 300 } else { 100 };
        self.score += points + 10 * u64::from(self.combo.min(20));
        self.last = Some((hit, 1.2));
        self.rings.push((lane, 0.5));
        fx.sound(Sfx::Note(pitch));
    }

    fn counts(&self) -> (usize, usize, usize) {
        let mut c = (0, 0, 0);
        for n in &self.notes {
            match n.result {
                Some(Hit::InTime) => c.0 += 1,
                Some(Hit::Close) => c.1 += 1,
                Some(Hit::Missed) => c.2 += 1,
                None => {}
            }
        }
        c
    }

    fn finished(&self) -> bool {
        self.notes.iter().all(|n| n.result.is_some())
    }
}

impl Game for Signal {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn zoomable(&self) -> bool {
        // the stage grows its own lanes with the window, which keeps the notes sharp
        false
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        match key {
            Key::Up | Key::Char('1' | 'k' | 'K') => self.press(0, fx),
            Key::Right | Key::Char('2' | 'l' | 'L') => self.press(1, fx),
            Key::Down | Key::Char('3' | 'j' | 'J') => self.press(2, fx),
            _ => {}
        }
    }

    fn tick(&mut self, dt: f32, fx: &mut Fx) {
        self.time += dt;
        let time = self.time;
        let mut missed = false;
        for n in &mut self.notes {
            if n.result.is_none() && time - n.at > CLOSE {
                n.result = Some(Hit::Missed);
                missed = true;
            }
        }
        if missed {
            self.combo = 0;
            self.last = Some((Hit::Missed, 1.2));
        }
        for r in &mut self.rings {
            r.1 -= dt;
        }
        self.rings.retain(|r| r.1 > 0.0);
        if let Some((_, t)) = &mut self.last {
            *t -= dt;
            if *t <= 0.0 {
                self.last = None;
            }
        }
        if self.finished() {
            fx.sound(Sfx::Win);
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        let ink = Ink::new(pal);
        // the stage takes the window's width and as many rows as it can spare, so the lanes
        // and their notes grow with it; four rows stay for the result and the song
        let w = if area.w >= 90 {
            area.w - 4
        } else {
            area.w.min(70)
        };
        let lanes_h = (area.h - 4).clamp(11, 30);
        let r = area.centre(w, lanes_h + 4);
        let stage = Rect::new(r.x, r.y, w, lanes_h);
        c.titled(stage, Line::Heavy, pal.blue, "Stage", pal.bright);
        let colours = [pal.cyan, pal.green, pal.magenta];
        let keys = ["↑ 1", "→ 2", "↓ 3"];
        let text = [pal.muted, pal.text, pal.bright];
        // the stage floor, faintly blue, with a band for each lane that the key labels sit on
        let floor = ink.under(pal.blue, 0.05, &text);
        let mut layer = Field::new(
            pal,
            Rect::new(stage.x + 1, stage.y + 1, stage.w - 2, stage.h - 2),
            floor,
        );
        let (lw, lh) = (layer.w(), layer.h());
        let pitch = lh / 3;
        let top = (lh - 3 * pitch) / 2;
        let centre = |lane: usize| top + pitch / 2 + lane as i32 * pitch;
        // the note's half height in pixels, a little under half a lane
        let size = ((pitch - 4) / 2).max(2);
        for (lane, &colour) in colours.iter().enumerate() {
            let band = ink.under(colour, 0.16, &text);
            let rim = ink.tint(colour, 0.32);
            let y0 = top + lane as i32 * pitch;
            layer.px.fill_rect(0, y0 + 1, lw, pitch - 2, band);
            layer.px.fill_rect(0, y0, lw, 1, rim);
            layer.px.fill_rect(0, y0 + pitch - 1, lw, 1, rim);
        }
        let line_x = 6 + size * 2;
        let right = lw - size - 3;
        let span = (right - line_x) as f32;
        // beats drift towards the line in the gaps between the lanes, so the tempo can be seen
        let tick = ink.tint(pal.blue, 0.45);
        let first = (self.time / self.beat).floor() as i32;
        for k in first..first + (TRAVEL / self.beat) as i32 + 2 {
            let ahead = k as f32 * self.beat - self.time;
            let x = line_x + (ahead / TRAVEL * span).round() as i32;
            if x > line_x + size && x < right {
                for gap in 1..3 {
                    let y = top + gap * pitch;
                    layer.px.fill_rect(x, y - 1, 2, 2, tick);
                }
            }
        }
        for (lane, &colour) in colours.iter().enumerate() {
            let band = ink.under(colour, 0.28, &text);
            layer.px.fill_rect(
                line_x - size * 2 + 1,
                top + lane as i32 * pitch + 1,
                size * 4 - 1,
                pitch - 2,
                band,
            );
        }
        for (lane, &colour) in colours.iter().enumerate() {
            let caught = self
                .rings
                .iter()
                .filter(|r| r.0 == lane)
                .map(|r| r.1)
                .fold(0.0, f32::max);
            let cy = centre(lane);
            if caught > 0.0 {
                // a caught note leaves a soft glow that shrinks in three still steps, well under
                // three changes a second. It also fills the outline it landed in
                let grow = if caught > 0.33 {
                    1.0
                } else if caught > 0.16 {
                    0.8
                } else {
                    0.6
                };
                let (rx, ry) = (size as f32 * 3.4 * grow, (pitch as f32 / 2.0 - 1.5) * grow);
                glow(
                    &mut layer,
                    line_x as f32,
                    cy as f32,
                    rx,
                    ry,
                    ink.under(colour, 0.5, &text),
                );
                note(&mut layer, &ink, lane, line_x, cy, size, colour);
            } else {
                outline(&mut layer, lane, line_x, cy, size, ink.solid(colour));
            }
        }
        for n in &self.notes {
            if n.result.is_some() {
                continue;
            }
            let ahead = n.at - self.time;
            if ahead > TRAVEL || ahead < -CLOSE {
                continue;
            }
            let x = line_x + (ahead / TRAVEL * span).round() as i32;
            let cy = centre(n.lane);
            // a soft trail behind each note, the way it came
            let steps = [0.5, 0.42, 0.34, 0.26, 0.2, 0.14];
            let thick = (size / 3).max(1);
            for (i, t) in steps.iter().enumerate() {
                let tx = x + size + 1 + i as i32 * (size / 2).max(1);
                if tx < lw {
                    layer.px.fill_rect(
                        tx,
                        cy - thick / 2,
                        (size / 2).max(1),
                        thick,
                        ink.tint(colours[n.lane], *t),
                    );
                }
            }
            note(&mut layer, &ink, n.lane, x, cy, size, colours[n.lane]);
        }
        layer.paint(c);
        for (lane, key) in keys.iter().enumerate() {
            text_on(
                c,
                stage.x + 2,
                stage.y + 1 + centre(lane) / 2,
                key,
                pal.text,
            );
        }
        // the result line and progress under the stage
        let below = stage.y + stage.h;
        if let Some((hit, _)) = self.last {
            let (text, colour) = match hit {
                Hit::InTime => (format!("{} In time", g('◆', '*')), pal.green),
                Hit::Close => (format!("{} Close", g('◇', '+')), pal.yellow),
                Hit::Missed => (format!("{} Missed", '·'), pal.muted),
            };
            c.text(stage.x + 2, below, &text, colour);
        }
        let caught = self.notes.iter().filter(|n| n.result.is_some()).count();
        c.text(stage.x + 2, below + 2, "Song", pal.muted);
        c.meter(
            stage.x + 8,
            below + 2,
            30,
            caught as f32 / self.notes.len().max(1) as f32,
            pal.blue,
            pal.muted,
        );
        c.text(
            stage.x + 40,
            below + 2,
            &format!("{caught}/{}", self.notes.len()),
            pal.text,
        );
        let tempo = (60.0 / self.beat).round();
        c.text(
            stage.x + w - 16,
            below + 2,
            &format!("{tempo} beats/min"),
            pal.muted,
        );
    }

    fn status(&self) -> Status {
        let (good, close, _) = self.counts();
        vec![
            ("Score".into(), self.score.to_string()),
            ("Combo".into(), self.combo.to_string()),
            ("In time".into(), good.to_string()),
            ("Close".into(), close.to_string()),
        ]
    }

    fn score(&self) -> u64 {
        self.score
    }

    fn outcome(&self) -> Option<Outcome> {
        self.finished().then(|| {
            let (good, close, missed) = self.counts();
            Outcome::Won(format!(
                "{good} in time, {close} close and {missed} missed. Longest run {}.",
                self.best_combo
            ))
        })
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        // catch the first five notes on the beat, so a still shows rings and a result
        let keys = [Key::Up, Key::Right, Key::Down];
        let mut demo = vec![(Key::None, self.notes[0].at)];
        for pair in self.notes.windows(2).take(5) {
            demo.push((keys[pair[0].lane], pair[1].at - pair[0].at));
        }
        if let Some(last) = demo.last_mut() {
            last.1 = 0.1;
        }
        demo
    }
}

/// The pixels of a lane's note around its middle, `size` pixels from the middle to the top:
/// high points up, low points down and the middle is round, so the lanes differ by shape as
/// well as colour.
fn shape(lane: usize, size: i32) -> Vec<(i32, i32)> {
    let r = size as f32 + 0.5;
    let half_w = r * 1.25;
    let mut out = Vec::new();
    for dy in -size..=size {
        for dx in -(half_w.ceil() as i32)..=half_w.ceil() as i32 {
            let (u, v) = (dx as f32 / half_w, dy as f32 / r);
            let inside = match lane {
                0 => v.abs() <= 1.0 && u.abs() <= (v + 1.0) / 2.0 + 0.08,
                1 => u * u + v * v <= 1.0,
                _ => v.abs() <= 1.0 && u.abs() <= (1.0 - v) / 2.0 + 0.08,
            };
            if inside {
                out.push((dx, dy));
            }
        }
    }
    out
}

/// A note centred on `x`, `cy`, lit along its upper left edge and shaded along its lower
/// right, so it reads as a solid bead with the light from the top left.
fn note(layer: &mut Field, ink: &Ink, lane: usize, x: i32, cy: i32, size: i32, colour: Rgb) {
    let (lit, dark) = ink.lit(colour);
    let body = ink.solid(colour);
    let cells = shape(lane, size);
    let has = |dx: i32, dy: i32| cells.contains(&(dx, dy));
    for &(dx, dy) in &cells {
        let tone = if !has(dx, dy - 1) || !has(dx - 1, dy) {
            lit
        } else if !has(dx, dy + 1) || !has(dx + 1, dy) {
            dark
        } else {
            body
        };
        layer.px.set(x + dx, cy + dy, tone);
    }
}

/// The outline of a lane's note on the line: the shape to aim each note at.
fn outline(layer: &mut Field, lane: usize, x: i32, cy: i32, size: i32, colour: Rgb) {
    let cells = shape(lane, size);
    for &(dx, dy) in &cells {
        let edge = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .any(|&(ex, ey)| !cells.contains(&(dx + ex, dy + ey)));
        if edge {
            layer.px.set(x + dx, cy + dy, colour);
        }
    }
}

/// A filled ellipse of ground colour, for the glow a caught note leaves.
fn glow(layer: &mut Field, cx: f32, cy: f32, rx: f32, ry: f32, colour: Rgb) {
    for y in (cy - ry).floor() as i32..=(cy + ry).ceil() as i32 {
        for x in (cx - rx).floor() as i32..=(cx + rx).ceil() as i32 {
            let (dx, dy) = ((x as f32 - cx) / rx, (y as f32 - cy) / ry);
            if dx * dx + dy * dy <= 1.0 {
                layer.px.set(x, y, colour);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(reduced: bool) -> Setup {
        Setup {
            seed: 5,
            reduced_motion: reduced,
            day: 0,
            practice: false,
        }
    }

    #[test]
    fn a_song_is_the_same_for_a_seed_and_never_doubles_up() {
        let a = compose(5, 0.8);
        assert_eq!(a, compose(5, 0.8));
        assert!(a.len() >= 30);
        for pair in a.windows(2) {
            assert!(pair[1].at - pair[0].at >= 0.39, "notes too close");
        }
    }

    #[test]
    fn reduced_motion_slows_the_tempo() {
        assert!(Signal::new(setup(true)).beat > Signal::new(setup(false)).beat);
    }

    #[test]
    fn a_press_on_time_scores_and_a_late_one_is_close() {
        let mut s = Signal::new(setup(false));
        let first = s.notes[0];
        let key = [Key::Up, Key::Right, Key::Down][first.lane];
        let mut fx = Fx::default();
        s.time = first.at;
        s.key(key, &mut fx);
        assert_eq!(s.notes[0].result, Some(Hit::InTime));
        assert!(s.score >= 300);
        assert_eq!(fx.sounds, vec![Sfx::Note(first.pitch)]);
        let second = s.notes[1];
        s.time = second.at + 0.2;
        s.key([Key::Up, Key::Right, Key::Down][second.lane], &mut fx);
        assert_eq!(s.notes[1].result, Some(Hit::Close));
    }

    #[test]
    fn a_stray_press_costs_nothing_and_a_passed_note_is_missed() {
        let mut s = Signal::new(setup(false));
        let mut fx = Fx::default();
        s.key(Key::Up, &mut fx);
        assert_eq!(s.score, 0);
        s.tick(s.notes[0].at + 1.0, &mut fx);
        assert_eq!(s.notes[0].result, Some(Hit::Missed));
    }

    #[test]
    fn the_song_ends_when_every_note_has_passed() {
        let mut s = Signal::new(setup(false));
        let end = s.notes.last().unwrap().at + 1.0;
        let mut fx = Fx::default();
        s.tick(end, &mut fx);
        assert!(matches!(s.outcome(), Some(Outcome::Won(_))));
    }
}
