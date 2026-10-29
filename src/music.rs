//! Calm background music for each game, synthesised by the program like the tones: a soft pad of
//! chords, a gentle bass and, for some games, a quiet arpeggio. Each game has its own key, scale
//! and tempo. A loop is rendered once into the cache and played by the system's own player in the
//! background, started again each time it ends. Notes that run past the end of the loop are
//! wrapped round to its start, so the loop joins without a seam. Music is levelled well below the
//! effects and never carries meaning of its own.
//!
//! Only one music player runs at a time. It is kept in a static, so the terminal guard can stop
//! it on every way out, a panic and a termination signal included.

use std::path::{Path, PathBuf};
use std::process::{Child, Stdio};
use std::sync::Mutex;

use crate::sound::{Sound, cache_dir, loudness, play_command};

/// Music is rendered at a lower rate than the effects: it has no high partials, so the files
/// stay small.
pub const RATE: u32 = 16_000;
/// The loudness music is levelled to at volume 100, under the effects' 0.2.
const TARGET_RMS: f32 = 0.09;
const PEAK: f32 = 0.5;
/// Bumped whenever the synthesis changes, so an older cached loop is not reused.
const MUSIC_VERSION: u32 = 1;

/// A game's musical mood.
#[derive(Clone, Copy, Debug)]
pub struct Mood {
    /// The key's root in semitones from A4.
    pub root: i8,
    /// The scale the chords and the arpeggio are drawn from, in semitones from the root.
    pub scale: &'static [i8],
    /// The chords as scale degrees, one for each `bars` bars.
    pub chords: &'static [usize],
    pub bars: u32,
    pub tempo: f32,
    /// A quiet arpeggio over the pad. None for the calmest games.
    pub arpeggio: bool,
}

const MAJOR: &[i8] = &[0, 2, 4, 5, 7, 9, 11];
const MINOR: &[i8] = &[0, 2, 3, 5, 7, 8, 10];
const DORIAN: &[i8] = &[0, 2, 3, 5, 7, 9, 10];
const LYDIAN: &[i8] = &[0, 2, 4, 6, 7, 9, 11];

/// The mood for a game id. Every game has one; an unknown id gets a plain calm one.
pub fn mood(id: &str) -> Mood {
    let m = |root, scale, chords, bars, tempo, arpeggio| Mood {
        root,
        scale,
        chords,
        bars,
        tempo,
        arpeggio,
    };
    match id {
        "stackline" => m(-9, DORIAN, &[0, 3, 4, 3], 2, 84.0, true),
        "merge-gates" => m(-7, LYDIAN, &[0, 1, 4, 0], 2, 76.0, true),
        "termle" => m(-4, MAJOR, &[0, 5, 3, 4], 2, 70.0, true),
        // signal's lanes carry the melody, so its music is only a soft pad
        "signal" => m(-9, MAJOR, &[0, 3, 5, 4], 2, 72.0, false),
        "trace" => m(-2, MINOR, &[0, 5, 2, 6], 2, 74.0, true),
        "drift" => m(-11, LYDIAN, &[0, 4, 1, 4], 2, 54.0, false),
        "rooms" => m(-12, MINOR, &[0, 3, 5, 4], 2, 66.0, true),
        "relay" => m(-5, DORIAN, &[0, 4, 3, 1], 2, 80.0, true),
        "through-ball" => m(-7, MAJOR, &[0, 4, 5, 3], 2, 96.0, true),
        "slipstream" => m(-10, MINOR, &[0, 6, 5, 6], 2, 90.0, true),
        "heatsink" => m(-3, DORIAN, &[0, 3, 0, 4], 2, 78.0, true),
        "learn-to-code" => m(-4, MAJOR, &[0, 3, 4, 0], 2, 64.0, false),
        "touch-typing" => m(-6, LYDIAN, &[0, 1, 0, 4], 2, 60.0, false),
        "array-guard" => m(-8, MINOR, &[0, 5, 6, 4], 2, 92.0, true),
        _ => m(-9, MAJOR, &[0, 3, 4, 0], 2, 70.0, false),
    }
}

fn freq(semitones: f32) -> f32 {
    440.0 * 2f32.powf(semitones / 12.0)
}

/// The pitch of a scale degree, any number of octaves up or down.
fn degree(mood: &Mood, step: i32) -> f32 {
    let n = mood.scale.len() as i32;
    let octave = step.div_euclid(n);
    let i = step.rem_euclid(n) as usize;
    f32::from(mood.root) + f32::from(mood.scale[i]) + 12.0 * octave as f32
}

/// Adds one note into the loop, wrapping past the end back to the start.
fn note(mix: &mut [f32], start: f32, len: f32, pitch: f32, level: f32, attack: f32, warm: f32) {
    let rate = RATE as f32;
    let total = mix.len();
    let f = freq(pitch);
    let count = (len * rate) as usize;
    let first = (start * rate) as usize;
    for i in 0..count {
        let t = i as f32 / rate;
        let env = (t / attack).min(1.0) * ((len - t) / (len * 0.5)).clamp(0.0, 1.0);
        let w = std::f32::consts::TAU * f * t;
        let wave = w.sin() + warm * (2.0 * w).sin();
        mix[(first + i) % total] += wave * env * level;
    }
}

/// The raw loop of a mood, before levelling.
pub fn render(mood: &Mood) -> Vec<f32> {
    let beat = 60.0 / mood.tempo;
    let chord_len = beat * 4.0 * mood.bars as f32;
    let length = chord_len * mood.chords.len() as f32;
    let mut mix = vec![0.0_f32; (length * RATE as f32) as usize];
    let mut seed: u32 = 0x9e37_79b9 ^ mood.root as u32;
    for (c, &chord) in mood.chords.iter().enumerate() {
        let at = c as f32 * chord_len;
        let base = chord as i32;
        // the pad: the chord's three notes an octave up, swelling in and out over the chord
        for (k, step) in [base, base + 2, base + 4].into_iter().enumerate() {
            let pitch = degree(mood, step) + 12.0;
            note(
                &mut mix,
                at,
                chord_len + beat,
                pitch,
                0.22 - k as f32 * 0.03,
                1.2,
                0.15,
            );
        }
        // the bass: the chord's root on every other beat, soft and round
        let beats = (4 * mood.bars) as usize;
        for b in (0..beats).step_by(2) {
            let pitch = degree(mood, base) - 12.0;
            note(
                &mut mix,
                at + b as f32 * beat,
                beat * 1.8,
                pitch,
                0.3,
                0.02,
                0.05,
            );
        }
        // the arpeggio: quiet eighth notes walking the chord, with a rest now and then
        if mood.arpeggio {
            for e in 0..beats * 2 {
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                if (seed >> 28).is_multiple_of(4) {
                    continue;
                }
                let step = base + [0, 2, 4, 7][((seed >> 24) % 4) as usize];
                let pitch = degree(mood, step) + 24.0;
                note(
                    &mut mix,
                    at + e as f32 * beat / 2.0,
                    beat * 0.9,
                    pitch,
                    0.1,
                    0.01,
                    0.1,
                );
            }
        }
    }
    mix
}

/// The 16 bit samples of a mood's loop at a volume from 0 to 100.
pub fn synthesise(mood: &Mood, volume: u8) -> Vec<i16> {
    let mix = render(mood);
    // loudness is measured as Scenes measures it, at the effects' rate
    let loud = loudness(&mix);
    let peak = mix.iter().fold(0.0_f32, |m, s| m.max(s.abs()));
    let volume = f32::from(volume.min(100)) / 100.0;
    let gain = if loud > 0.0 && peak > 0.0 {
        (TARGET_RMS / loud).min(PEAK / peak) * volume
    } else {
        0.0
    };
    mix.iter()
        .map(|s| (s * gain * f32::from(i16::MAX)).round() as i16)
        .collect()
}

/// A mono 16 bit WAV at the music rate.
fn wav(samples: &[i16]) -> Vec<u8> {
    let data = (samples.len() * 2) as u32;
    let mut out = Vec::with_capacity(44 + samples.len() * 2);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16_u32.to_le_bytes());
    out.extend_from_slice(&1_u16.to_le_bytes());
    out.extend_from_slice(&1_u16.to_le_bytes());
    out.extend_from_slice(&RATE.to_le_bytes());
    out.extend_from_slice(&(RATE * 2).to_le_bytes());
    out.extend_from_slice(&2_u16.to_le_bytes());
    out.extend_from_slice(&16_u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data.to_le_bytes());
    for s in samples {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

/// The cached loop for a game at a volume, rendered when missing.
pub fn ensure_file(dir: &Path, id: &str, volume: u8) -> std::io::Result<PathBuf> {
    let path = dir.join(format!("music-{id}-v{volume}-m{MUSIC_VERSION}.wav"));
    if !path.is_file() {
        std::fs::create_dir_all(dir)?;
        let partial = path.with_extension(format!("wav.{}", std::process::id()));
        std::fs::write(&partial, wav(&synthesise(&mood(id), volume)))?;
        std::fs::rename(&partial, &path)?;
    }
    Ok(path)
}

/// The music player now running with the game and volume it plays.
static PLAYING: Mutex<Option<(Child, String)>> = Mutex::new(None);

/// Stops the music, waiting for the player to end so none is left behind.
pub fn stop() {
    let taken = PLAYING.lock().ok().and_then(|mut p| p.take());
    if let Some((mut child, _)) = taken {
        let _ = child.kill();
        let _ = child.wait();
    }
}

/// Plays a game's music. `None` stops it. Called every frame: it does nothing while the
/// right loop plays, starts it again when it ends and switches when the game or volume changes.
pub fn keep(sound: &Sound, game: Option<&str>) {
    let want = match game {
        Some(id) if sound.music_on() => Some(format!("{id}-{}", sound.music_volume)),
        _ => None,
    };
    let Some(want) = want else {
        stop();
        return;
    };
    let Ok(mut playing) = PLAYING.lock() else {
        return;
    };
    if let Some((child, key)) = playing.as_mut() {
        let running = matches!(child.try_wait(), Ok(None));
        if *key == want && running {
            return;
        }
        let _ = child.kill();
        let _ = child.wait();
        *playing = None;
    }
    let Some(player) = sound.player() else { return };
    let id = game.unwrap_or_default();
    let Ok(file) = ensure_file(&cache_dir(), id, sound.music_volume) else {
        return;
    };
    let child = play_command(player, &file)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    if let Ok(child) = child {
        *playing = Some((child, want));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const IDS: [&str; 14] = [
        "stackline",
        "merge-gates",
        "termle",
        "signal",
        "trace",
        "drift",
        "rooms",
        "relay",
        "through-ball",
        "slipstream",
        "heatsink",
        "learn-to-code",
        "touch-typing",
        "array-guard",
    ];

    #[test]
    fn every_game_has_a_calm_loop_of_twenty_to_sixty_seconds() {
        for id in IDS {
            let m = mood(id);
            assert!((50.0..=100.0).contains(&m.tempo), "{id}");
            let secs = 60.0 / m.tempo * 4.0 * m.bars as f32 * m.chords.len() as f32;
            assert!((19.0..=60.0).contains(&secs), "{id} lasts {secs}");
        }
    }

    #[test]
    fn music_sits_below_the_effects_and_never_clips() {
        let m = mood("signal");
        let samples = synthesise(&m, 100);
        let peak = samples.iter().map(|s| s.unsigned_abs()).max().unwrap();
        assert!(f32::from(peak) <= PEAK * f32::from(i16::MAX) + 1.0);
        let floats: Vec<f32> = samples
            .iter()
            .map(|&s| f32::from(s) / f32::from(i16::MAX))
            .collect();
        let effects: Vec<f32> = crate::sound::synthesise(crate::sound::Sfx::Good, 100)
            .iter()
            .map(|&s| f32::from(s) / f32::from(i16::MAX))
            .collect();
        assert!(loudness(&floats) < loudness(&effects) * 0.6);
    }

    #[test]
    fn the_loop_joins_without_a_jump() {
        let m = mood("drift");
        let s = synthesise(&m, 60);
        let (first, last) = (i32::from(s[0]), i32::from(s[s.len() - 1]));
        // neighbouring samples of a wrapped loop are close, as anywhere else in it
        let biggest = s
            .windows(2)
            .map(|w| (i32::from(w[1]) - i32::from(w[0])).abs())
            .max()
            .unwrap();
        assert!((first - last).abs() <= biggest + 1);
    }

    #[test]
    fn stop_with_nothing_playing_is_harmless() {
        stop();
        stop();
    }
}
