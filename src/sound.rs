//! Soft tones for the games, synthesised at run time into small WAV files in a cache folder and
//! played by the system's own player in the background, the way Zacess Scenes plays its tones.
//! Nothing is bundled and no audio library is linked: a missing player only means no sound, and
//! every tone has its meaning written on screen as well.

use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use zacess_scenes::sound::{
    Player, SAMPLE_RATE, SOUND_ENV, VOLUME_ENV, env_turns_off, find_player, parse_volume, wav_bytes,
};

/// The volume when nothing sets one, out of 100.
pub const DEFAULT_VOLUME: u8 = 35;
/// How far `+` and `-` move the volume.
pub const STEP: u8 = 10;
/// The loudest sample at volume 100, as a share of full scale, the same headroom Zacess Scenes
/// keeps so no player's own resampling clips.
const PEAK: f32 = 0.7;
/// The loudness a full level tone is set to at volume 100: the root mean square of its loudest
/// 50 milliseconds, as a share of full scale. The same target Zacess Scenes levels its tones to,
/// so a game at volume 35 sounds as loud as a scene at 35.
const TARGET_RMS: f32 = 0.2;
/// The shortest note. Anything shorter reads as a click rather than a tone on laptop speakers.
pub const SHORTEST: f32 = 0.12;
/// The least time between two quiet tones, so a run of fast keys never stacks into noise.
const TICK_GAP: f32 = 0.09;
/// Bumped whenever the synthesis changes, so an older cached file is not reused.
const SYNTH_VERSION: u32 = 2;

/// The tones a game can ask for. Pitches are in semitones from A4, so a melody stays in tune.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfx {
    /// The quiet tone of a frequent action: a step, a turn, a typed letter or a cursor move.
    Tick,
    /// A placed piece, a pickup, a pass or a guess.
    Tap,
    /// A rising pair for a clear, a merge, a lit node or a door opening.
    Good,
    /// A rising triad for a level or a game won.
    Win,
    /// A low falling pair for a refused move or a lost game. Never harsh.
    Low,
    /// Two even pulses for something to act on soon: fuel running low, a chip running hot or a
    /// dish on its last chance. Never an alarm.
    Warn,
    /// One note at a pitch, for caught notes and gathered light.
    Note(i8),
}

impl Sfx {
    /// Every tone with a pitch of its own, for `zacess-games sounds`.
    pub const SAMPLER: [Sfx; 7] = [
        Sfx::Tick,
        Sfx::Tap,
        Sfx::Good,
        Sfx::Win,
        Sfx::Low,
        Sfx::Warn,
        Sfx::Note(0),
    ];

    /// The notes: (semitones from A4, start in seconds, length in seconds).
    fn notes(self) -> Vec<(i8, f32, f32)> {
        match self {
            Sfx::Tick => vec![(10, 0.0, SHORTEST)],
            Sfx::Tap => vec![(5, 0.0, 0.16)],
            Sfx::Good => vec![(3, 0.0, 0.16), (10, 0.11, 0.24)],
            Sfx::Win => vec![(3, 0.0, 0.2), (7, 0.15, 0.2), (12, 0.3, 0.45)],
            Sfx::Low => vec![(-2, 0.0, 0.18), (-7, 0.14, 0.28)],
            // the same E5 twice, neither rising nor falling, so it reads as "look" rather than
            // a win or a loss
            Sfx::Warn => vec![(7, 0.0, 0.14), (7, 0.22, 0.2)],
            Sfx::Note(p) => vec![(p, 0.0, 0.4)],
        }
    }

    /// How loud the tone is beside a full one: frequent actions sit well back.
    fn level(self) -> f32 {
        match self {
            Sfx::Tick => 0.55,
            Sfx::Tap => 0.75,
            _ => 1.0,
        }
    }

    /// Whether the tone is quiet and frequent, so it waits out the gap after another.
    fn frequent(self) -> bool {
        matches!(self, Sfx::Tick | Sfx::Tap)
    }

    pub fn name(self) -> String {
        match self {
            Sfx::Tick => "tick".into(),
            Sfx::Tap => "tap".into(),
            Sfx::Good => "good".into(),
            Sfx::Win => "win".into(),
            Sfx::Low => "low".into(),
            Sfx::Warn => "warn".into(),
            Sfx::Note(p) => format!("note{p}"),
        }
    }

    /// What the tone means, for `zacess-games sounds`.
    pub fn meaning(self) -> &'static str {
        match self {
            Sfx::Tick => "a step, a turn, a typed letter or a cursor move",
            Sfx::Tap => "a placed piece, a pickup, a pass or a guess",
            Sfx::Good => "a clear, a merge, a lit node or an open door",
            Sfx::Win => "a level solved, a goal or a game won",
            Sfx::Low => "a refused move, a bump or a lost game",
            Sfx::Warn => "fuel running low, a chip running hot or a dish on its last chance",
            Sfx::Note(_) => "a caught note or gathered light, at its own pitch",
        }
    }
}

/// The root mean square of the loudest 50 milliseconds, the loudness measure Scenes uses.
pub fn loudness(samples: &[f32]) -> f32 {
    zacess_scenes::sound::loudness(samples)
}

/// The tone before levelling: each note a sine with soft second and third partials, so it has a
/// pitched body on small speakers, an 8 millisecond attack and a gentle exponential decay.
fn raw(sfx: Sfx) -> Vec<f32> {
    let notes = sfx.notes();
    let end = notes
        .iter()
        .map(|&(_, start, len)| start + len)
        .fold(0.0_f32, f32::max);
    let rate = SAMPLE_RATE as f32;
    let total = (end * rate) as usize + 1;
    let mut mix = vec![0.0_f32; total];
    for &(pitch, start, len) in &notes {
        let freq = 440.0 * 2f32.powf(f32::from(pitch) / 12.0);
        let first = (start * rate) as usize;
        let count = (len.max(SHORTEST) * rate) as usize;
        for i in 0..count {
            let t = i as f32 / rate;
            let attack = (t / 0.008).min(1.0);
            let fade = (-t * 3.0 / len).exp();
            let tail = ((len - t) / 0.03).clamp(0.0, 1.0);
            let w = std::f32::consts::TAU * freq * t;
            let wave = w.sin() + 0.4 * (2.0 * w).sin() + 0.15 * (3.0 * w).sin();
            if let Some(s) = mix.get_mut(first + i) {
                *s += wave * attack * fade * tail;
            }
        }
    }
    mix
}

/// The samples of a tone at a volume from 0 to 100, levelled to the target loudness unless that
/// would push its peak past the ceiling, as Scenes levels its own tones.
pub fn synthesise(sfx: Sfx, volume: u8) -> Vec<i16> {
    let mix = raw(sfx);
    let loud = loudness(&mix);
    let peak = mix.iter().fold(0.0_f32, |m, s| m.max(s.abs()));
    let volume = f32::from(volume.min(100)) / 100.0;
    let gain = if loud > 0.0 && peak > 0.0 {
        (TARGET_RMS * sfx.level() / loud).min(PEAK / peak) * volume
    } else {
        0.0
    };
    mix.iter()
        .map(|s| {
            (s * gain * f32::from(i16::MAX))
                .round()
                .clamp(-f32::from(i16::MAX), f32::from(i16::MAX)) as i16
        })
        .collect()
}

/// The cache folder: `$XDG_CACHE_HOME/zacess-games`, `~/Library/Caches` on macOS,
/// `%LOCALAPPDATA%` on Windows, otherwise `~/.cache`.
pub fn cache_dir() -> PathBuf {
    let var = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty());
    let base = if let Some(dir) = var("XDG_CACHE_HOME") {
        PathBuf::from(dir)
    } else if cfg!(windows) {
        var("LOCALAPPDATA").map_or_else(std::env::temp_dir, PathBuf::from)
    } else if let Some(home) = var("HOME") {
        let home = PathBuf::from(home);
        if cfg!(target_os = "macos") {
            home.join("Library").join("Caches")
        } else {
            home.join(".cache")
        }
    } else {
        std::env::temp_dir()
    };
    base.join("zacess-games")
}

fn ensure_wav(dir: &Path, sfx: Sfx, volume: u8) -> std::io::Result<PathBuf> {
    let path = dir.join(format!("{}-v{volume}-s{SYNTH_VERSION}.wav", sfx.name()));
    if !path.is_file() {
        std::fs::create_dir_all(dir)?;
        // written beside the target and renamed, so a player never reads half a file
        let partial = path.with_extension(format!("wav.{}", std::process::id()));
        std::fs::write(&partial, wav_bytes(&synthesise(sfx, volume)))?;
        std::fs::rename(&partial, &path)?;
    }
    Ok(path)
}

pub fn play_command(player: &Player, file: &Path) -> Command {
    match player {
        Player::Command(program) => {
            let mut command = Command::new(program);
            if program.file_stem().is_some_and(|s| s == "aplay") {
                command.arg("-q");
            }
            command.arg(file);
            command
        }
        Player::PowerShell(program) => {
            let quoted = file.display().to_string().replace('\'', "''");
            let mut command = Command::new(program);
            command.args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                &format!("(New-Object System.Media.SoundPlayer '{quoted}').PlaySync()"),
            ]);
            command
        }
    }
}

/// Sound for a session: on or off, the volume and the player found once.
pub struct Sound {
    /// Effects on or off and their volume.
    pub enabled: bool,
    pub volume: u8,
    /// Music on or off and its volume.
    pub music_enabled: bool,
    pub music_volume: u8,
    /// The background sound under each game on or off and its volume.
    pub background_enabled: bool,
    pub background_volume: u8,
    player: Option<Player>,
    dir: PathBuf,
    /// A headless run: the settings show as they would live, but nothing is ever played.
    headless: bool,
    /// When the last quiet tone started, for the gap between frequent tones.
    last_tick: Cell<Option<Instant>>,
}

/// The music volume when nothing sets one: below the effects, so it stays in the background.
pub const DEFAULT_MUSIC_VOLUME: u8 = 20;
/// The variable that sets the music volume, 0 to 100.
pub const MUSIC_VOLUME_ENV: &str = "ZACESS_MUSIC_VOLUME";
/// The background sound's volume when nothing sets one: under the music, since it plays the
/// whole time.
pub const DEFAULT_BACKGROUND_VOLUME: u8 = 15;
/// The variable that turns the background sound on or off, the same one Zacess Scenes reads.
pub const BACKGROUND_ENV: &str = "ZACESS_AMBIENT";

/// The keys the settings are remembered under in the records file.
pub const FX_KEY: &str = "setting-effects-volume";
pub const FX_ON_KEY: &str = "setting-effects-on";
pub const MUSIC_KEY: &str = "setting-music-volume";
pub const MUSIC_ON_KEY: &str = "setting-music-on";
pub const BACKGROUND_KEY: &str = "setting-background-volume";
pub const BACKGROUND_ON_KEY: &str = "setting-background-on";

/// Where the sound settings come from, in order: the flags, the variables, the settings saved
/// last time and the defaults.
#[derive(Clone, Copy, Debug, Default)]
pub struct Choice {
    pub no_sound: bool,
    pub volume: Option<u8>,
    pub no_music: bool,
    pub music_volume: Option<u8>,
    /// `--background` or `--no-background`.
    pub background: Option<bool>,
    pub background_volume: Option<u8>,
}

impl Sound {
    pub fn from_env(choice: Choice, saved: &crate::scores::Scores) -> Self {
        let var = |name: &str| std::env::var(name).ok();
        let env_off = env_turns_off(var(SOUND_ENV).as_deref());
        let env_volume = var(VOLUME_ENV).and_then(|v| parse_volume(&v));
        let env_music = var(MUSIC_VOLUME_ENV).and_then(|v| parse_volume(&v));
        let saved_u8 = |key: &str| saved.get(key).and_then(|v| u8::try_from(v.min(100)).ok());
        let saved_on = |key: &str| saved.get(key).is_none_or(|v| v != 0);
        let env_background = match var(BACKGROUND_ENV).as_deref().map(str::trim) {
            Some("1" | "true" | "yes" | "on") => Some(true),
            Some(value) if env_turns_off(Some(value)) => Some(false),
            _ => None,
        };
        // the background sound is off until asked for, so a saved setting needs saying on
        let saved_background = saved.get(BACKGROUND_ON_KEY).map(|v| v != 0);
        Self {
            enabled: !choice.no_sound && !env_off && saved_on(FX_ON_KEY),
            volume: choice
                .volume
                .or(env_volume)
                .or_else(|| saved_u8(FX_KEY))
                .unwrap_or(DEFAULT_VOLUME),
            music_enabled: !choice.no_music && !env_off && saved_on(MUSIC_ON_KEY),
            music_volume: choice
                .music_volume
                .or(env_music)
                .or_else(|| saved_u8(MUSIC_KEY))
                .unwrap_or(DEFAULT_MUSIC_VOLUME),
            background_enabled: !env_off
                && choice
                    .background
                    .or(env_background)
                    .or(saved_background)
                    .unwrap_or(false),
            background_volume: choice
                .background_volume
                .or_else(|| saved_u8(BACKGROUND_KEY))
                .unwrap_or(DEFAULT_BACKGROUND_VOLUME),
            player: find_player(),
            dir: cache_dir(),
            headless: false,
            last_tick: Cell::new(None),
        }
    }

    /// The default settings with nothing played, for tests, headless runs and stills.
    pub fn silent() -> Self {
        Self {
            enabled: true,
            volume: DEFAULT_VOLUME,
            music_enabled: true,
            music_volume: DEFAULT_MUSIC_VOLUME,
            background_enabled: false,
            background_volume: DEFAULT_BACKGROUND_VOLUME,
            player: None,
            dir: cache_dir(),
            headless: true,
            last_tick: Cell::new(None),
        }
    }

    /// Writes the settings to the records, so the next run starts with them.
    pub fn save(&self, scores: &mut crate::scores::Scores) {
        scores.set(FX_KEY, u64::from(self.volume));
        scores.set(FX_ON_KEY, u64::from(self.enabled));
        scores.set(MUSIC_KEY, u64::from(self.music_volume));
        scores.set(MUSIC_ON_KEY, u64::from(self.music_enabled));
        scores.set(BACKGROUND_KEY, u64::from(self.background_volume));
        scores.set(BACKGROUND_ON_KEY, u64::from(self.background_enabled));
    }

    fn step(v: u8, up: bool) -> u8 {
        if up {
            v.saturating_add(STEP).min(100)
        } else {
            v.saturating_sub(STEP)
        }
    }

    /// Effects volume up or down by one step. Turning it up also unmutes.
    pub fn change(&mut self, up: bool) {
        self.volume = Self::step(self.volume, up);
        if up {
            self.enabled = true;
        }
    }

    /// Music volume up or down by one step. Turning it up also unmutes.
    pub fn change_music(&mut self, up: bool) {
        self.music_volume = Self::step(self.music_volume, up);
        if up {
            self.music_enabled = true;
        }
    }

    /// Background sound volume up or down by one step. Turning it up also turns it on and
    /// down to 0 turns it off.
    pub fn change_background(&mut self, up: bool) {
        self.background_volume = Self::step(self.background_volume, up);
        if up {
            self.background_enabled = true;
        } else if self.background_volume == 0 {
            self.background_enabled = false;
        }
    }

    /// Mutes everything. Brings effects and music back when everything is already muted; the
    /// background sound stays off until asked for again, as it starts.
    pub fn toggle_all(&mut self) {
        let any = self.enabled || self.music_enabled || self.background_enabled;
        self.enabled = !any;
        self.music_enabled = !any;
        if any {
            self.background_enabled = false;
        }
    }

    /// Whether the background sound should play, apart from what the game is doing.
    pub fn background_on(&self) -> bool {
        !self.headless
            && self.background_enabled
            && self.background_volume > 0
            && self.player.is_some()
    }

    /// The background sound setting, such as `15%` or `off`.
    pub fn background_label(&self) -> String {
        if !self.background_enabled || self.background_volume == 0 {
            "off".into()
        } else {
            format!("{}%", self.background_volume)
        }
    }

    pub fn music_on(&self) -> bool {
        !self.headless && self.music_enabled && self.music_volume > 0 && self.player.is_some()
    }

    pub fn player(&self) -> Option<&Player> {
        self.player.as_ref()
    }

    /// The effects setting, such as `35%` or `muted`.
    pub fn label(&self) -> String {
        if !self.enabled || self.volume == 0 {
            "muted".into()
        } else {
            format!("{}%", self.volume)
        }
    }

    /// The music setting, such as `20%` or `off`.
    pub fn music_label(&self) -> String {
        if !self.music_enabled || self.music_volume == 0 {
            "off".into()
        } else {
            format!("{}%", self.music_volume)
        }
    }

    /// The settings for the status bar, such as `FX 35% · Music 20%`, with the background sound
    /// added while it is on.
    pub fn status(&self) -> String {
        let mut s = format!("FX {} · Music {}", self.label(), self.music_label());
        // named only while on, since it starts off and the status line is short on a menu
        if self.background_enabled && self.background_volume > 0 {
            s.push_str(&format!(" · Background {}", self.background_label()));
        }
        if self.player.is_none() && !self.headless {
            s.push_str(" (no player)");
        }
        s
    }

    /// Plays a tone in the background. Every failure is silent: the screen shows the same thing.
    /// A quiet tone that comes too soon after the last one is skipped.
    pub fn play(&self, sfx: Sfx) {
        if self.headless || !self.enabled || self.volume == 0 {
            return;
        }
        if sfx.frequent() {
            let now = Instant::now();
            if self
                .last_tick
                .get()
                .is_some_and(|t| now.duration_since(t) < Duration::from_secs_f32(TICK_GAP))
            {
                return;
            }
            self.last_tick.set(Some(now));
        }
        let _ = self.start(sfx);
    }

    /// Starts a tone whatever the settings say and returns the player, for `zacess-games sounds`.
    pub fn start(&self, sfx: Sfx) -> Option<Child> {
        let player = self.player.as_ref()?;
        let file = ensure_wav(&self.dir, sfx, self.volume.max(1)).ok()?;
        play_command(player, &file)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tones_stay_under_the_ceiling_and_start_softly() {
        for sfx in Sfx::SAMPLER {
            let samples = synthesise(sfx, 100);
            let peak = samples.iter().map(|s| s.unsigned_abs()).max().unwrap();
            assert!(f32::from(peak) <= PEAK * f32::from(i16::MAX) + 1.0);
            // no click at the very start
            assert!(samples[0].unsigned_abs() < 200);
        }
    }

    fn as_float(samples: &[i16]) -> Vec<f32> {
        samples
            .iter()
            .map(|&s| f32::from(s) / f32::from(i16::MAX))
            .collect()
    }

    #[test]
    fn no_tone_is_shorter_than_the_shortest_note() {
        for sfx in Sfx::SAMPLER {
            let secs = synthesise(sfx, 35).len() as f32 / SAMPLE_RATE as f32;
            assert!(secs >= SHORTEST, "{} lasts {secs}", sfx.name());
        }
    }

    #[test]
    fn tones_at_35_are_as_loud_as_a_scenes_tone_at_35() {
        use zacess_scenes::sound::{SoundSet, Tone};
        let scenes = loudness(&as_float(&zacess_scenes::sound::synthesise(
            SoundSet::Piano,
            Tone::Pass,
            35,
        )));
        for sfx in [Sfx::Good, Sfx::Win, Sfx::Low, Sfx::Note(0)] {
            let ours = loudness(&as_float(&synthesise(sfx, 35)));
            assert!(
                ours >= scenes * 0.8,
                "{} is {ours} beside {scenes}",
                sfx.name()
            );
        }
        // the frequent tones sit back but stay clearly audible
        for sfx in [Sfx::Tick, Sfx::Tap] {
            let ours = loudness(&as_float(&synthesise(sfx, 35)));
            assert!(
                ours >= scenes * 0.45,
                "{} is {ours} beside {scenes}",
                sfx.name()
            );
        }
    }

    #[test]
    fn the_background_sound_starts_off_and_its_keys_turn_it_on_and_off() {
        let mut s = Sound::silent();
        assert!(!s.background_enabled && !s.background_on());
        s.change_background(true);
        assert!(s.background_enabled);
        assert_eq!(s.background_volume, DEFAULT_BACKGROUND_VOLUME + STEP);
        assert_eq!(s.status(), "FX 35% · Music 20% · Background 25%");
        // a headless run never plays it, whatever the setting
        assert!(!s.background_on());
        s.change_background(false);
        s.change_background(false);
        s.change_background(false);
        assert_eq!(s.background_volume, 0);
        assert!(!s.background_enabled);
        assert_eq!(s.status(), "FX 35% · Music 20%");
        // muting everything turns it off and bringing sound back leaves it off
        s.change_background(true);
        s.toggle_all();
        assert!(!s.background_enabled && !s.enabled);
        s.toggle_all();
        assert!(s.enabled && s.music_enabled && !s.background_enabled);
    }

    #[test]
    fn the_warn_tone_is_two_even_pulses_apart_from_low_and_good() {
        let notes = Sfx::Warn.notes();
        assert_eq!(notes.len(), 2);
        assert_eq!(notes[0].0, notes[1].0);
        assert!(notes[0].0 > Sfx::Low.notes()[0].0);
        assert!(!Sfx::Warn.frequent());
        assert!(Sfx::SAMPLER.contains(&Sfx::Warn));
    }

    #[test]
    fn volume_zero_is_silence() {
        assert!(synthesise(Sfx::Win, 0).iter().all(|&s| s == 0));
    }

    #[test]
    fn the_volume_steps_stay_in_range() {
        let mut s = Sound::silent();
        assert_eq!(s.status(), "FX 35% · Music 20%");
        s.toggle_all();
        assert_eq!(s.status(), "FX muted · Music off");
        s.toggle_all();
        assert_eq!(s.label(), "35%");
        s.change_music(false);
        assert_eq!(s.music_volume, 10);
        s.volume = 95;
        s.change(true);
        assert_eq!(s.volume, 100);
        s.volume = 5;
        s.change(false);
        assert_eq!(s.volume, 0);
    }
}
