//! What every game provides to the shared loop: its info, keys, a tick, drawing and a score.

use crate::scores::Scores;
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Rect, Status};

/// The fixed facts about a game, for the menu, the help and the docs.
pub struct Info {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: (char, char),
    pub hue: Hue,
    pub blurb: &'static str,
    /// The key hint line under the play area.
    pub hints: &'static str,
    /// The How to play page: a `# ` line starts a section, every other line is a paragraph that
    /// is wrapped to the window.
    pub help: &'static [&'static str],
    /// What the keyboard does in the game.
    pub input: Input,
    /// The word the score is counted in, such as "points".
    pub unit: &'static str,
}

/// How a game reads the keyboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input {
    /// Keys only: `Space` pauses, `?` helps and `Q` quits.
    Keys,
    /// Letters are typed, so `Q` is a letter and `Esc` quits; everything else is as with keys.
    Letters,
    /// Any character is typed, the space and symbols included. `Esc` pauses and opens the sound
    /// keys, F1 helps and `Ctrl+C` quits.
    Text,
}

/// How a game ended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Won(String),
    Lost(String),
}

/// Effects a game asks for during a key or a tick: tones and a short written note.
#[derive(Default, Debug)]
pub struct Fx {
    pub sounds: Vec<Sfx>,
    pub toast: Option<String>,
}

impl Fx {
    pub fn sound(&mut self, sfx: Sfx) {
        self.sounds.push(sfx);
    }

    /// A short line in the status bar, the written twin of a tone.
    pub fn say(&mut self, text: impl Into<String>) {
        self.toast = Some(text.into());
    }
}

/// Settings every game gets when it starts.
#[derive(Clone, Copy, Debug)]
pub struct Setup {
    pub seed: u64,
    /// Slows falling, drifting and the rhythm. Animations become still marks.
    pub reduced_motion: bool,
    /// Days since 1 January 1970, for the daily puzzle.
    pub day: u64,
    /// A round started from the end screen, so the daily puzzle gives way to practice.
    pub practice: bool,
}

pub trait Game {
    fn info(&self) -> &'static Info;
    fn key(&mut self, key: Key, fx: &mut Fx);
    /// Moves time on by `dt` seconds. Not called while paused.
    fn tick(&mut self, _dt: f32, _fx: &mut Fx) {}
    fn draw(&self, c: &mut Canvas, area: Rect);
    fn status(&self) -> Status;
    fn score(&self) -> u64;
    fn outcome(&self) -> Option<Outcome>;
    /// The key under which the best score is kept. Most games use their id.
    fn score_key(&self) -> String {
        self.info().id.to_string()
    }
    /// Reads any records the game keeps beyond its best score, such as the daily streak.
    fn restore(&mut self, _scores: &Scores) {}
    /// Writes those records when the game ends or is left.
    fn keep(&self, _scores: &mut Scores) {}
    /// Whether the game may be drawn at half size and stretched to fill a large window. Games
    /// full of text lay themselves out for a large window instead.
    fn zoomable(&self) -> bool {
        true
    }
    /// The size a game's drawing needs, when it cannot shrink. In a smaller play area it is
    /// drawn at this size and cropped, keeping the part at `crop_anchor`.
    fn natural_size(&self) -> Option<(i32, i32)> {
        None
    }
    /// Which part of a cropped drawing stays in view: 0 is the left or top, 1 the right or
    /// bottom.
    fn crop_anchor(&self) -> (f32, f32) {
        (0.5, 1.0)
    }
    /// Whether the game moves on its own, so the loop ticks it often.
    fn animated(&self) -> bool {
        true
    }
    /// Keys that bring a game to an interesting point for a still in the docs.
    fn demo(&self) -> Vec<(Key, f32)> {
        Vec::new()
    }
}
