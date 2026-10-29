//! The recording script: the keys each game's showcase recording plays, kept in
//! `scripts/recordings.txt` so a recording can be changed without touching the games.
//!
//! A `[game]` line starts a game's recording. Under it, `seed N` sets its seed, a line starting
//! `type SECONDS ` types the rest of the line one character at a time with that wait after each,
//! and any other line is keys written as for `--keys`, such as `left, left, enter, wait:0.4`.
//! A `#` line is a comment.

use crate::parse_keys;
use crate::term::Key;

/// The script built into the binary, so `snap --frames` works from an installed copy too.
pub const BUILT_IN: &str = include_str!("../scripts/recordings.txt");

/// One game's recording.
#[derive(Clone, Debug, PartialEq)]
pub struct Recording {
    pub game: String,
    pub seed: Option<u64>,
    /// Seconds played before the first frame is kept.
    pub start: f32,
    pub keys: Vec<(Key, f32)>,
}

/// Reads a whole script, naming the line of the first problem.
pub fn parse(text: &str) -> Result<Vec<Recording>, String> {
    let mut out: Vec<Recording> = Vec::new();
    for (n, raw) in text.lines().enumerate() {
        let line = raw.trim();
        let at = |e: String| format!("recordings line {}: {e}", n + 1);
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            let id = crate::games::find(name.trim())
                .ok_or_else(|| at(format!("no game called \"{name}\"")))?
                .id;
            if out.iter().any(|r| r.game == id) {
                return Err(at(format!("{id} is recorded twice")));
            }
            out.push(Recording {
                game: id.to_string(),
                seed: None,
                start: 0.0,
                keys: Vec::new(),
            });
            continue;
        }
        let current = out
            .last_mut()
            .ok_or_else(|| at("keys come after a [game] line".into()))?;
        if let Some(seed) = line.strip_prefix("seed ") {
            let seed = seed.trim();
            current.seed = Some(
                seed.parse()
                    .map_err(|_| at(format!("bad seed \"{seed}\"")))?,
            );
        } else if let Some(start) = line.strip_prefix("start ") {
            let start = start.trim();
            current.start = start
                .parse::<f32>()
                .ok()
                .filter(|s| (0.0..=600.0).contains(s))
                .ok_or_else(|| at(format!("bad start \"{start}\"")))?;
        } else if let Some(rest) = raw.trim_start().strip_prefix("type ") {
            // the text keeps its own spaces, so only the wait is split off
            let (wait, typed) = rest.split_once(' ').unwrap_or((rest, ""));
            let wait: f32 = wait
                .parse()
                .map_err(|_| at(format!("bad wait \"{wait}\"")))?;
            if typed.is_empty() {
                return Err(at("type wants some text".into()));
            }
            current
                .keys
                .extend(typed.chars().map(|ch| (Key::Char(ch), wait)));
        } else if let Some(rest) = line.strip_prefix("step ") {
            let (wait, keys) = rest.trim().split_once(' ').unwrap_or((rest, ""));
            let wait: f32 = wait
                .parse()
                .map_err(|_| at(format!("bad wait \"{wait}\"")))?;
            for token in keys.split_whitespace() {
                let (name, times) = match token.split_once('*') {
                    Some((name, n)) => (
                        name,
                        n.parse::<usize>()
                            .ok()
                            .filter(|n| (1..=100).contains(n))
                            .ok_or_else(|| at(format!("bad count in \"{token}\"")))?,
                    ),
                    None => (token, 1),
                };
                let key = parse_keys(name).map_err(at)?;
                for _ in 0..times {
                    current.keys.extend(key.iter().map(|&(k, _)| (k, wait)));
                }
            }
        } else {
            current.keys.extend(parse_keys(line).map_err(at)?);
        }
    }
    Ok(out)
}

/// The recording for one game in a script.
pub fn find<'a>(script: &'a [Recording], game: &str) -> Option<&'a Recording> {
    script.iter().find(|r| r.game == game)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app;
    use crate::game::Setup;
    use zacess_scenes::flash;
    use zacess_scenes::frame::Frame;
    use zacess_scenes::snapshot;
    use zacess_scenes::theme::{self, ThemeData};

    // the window, rate and length scripts/snapshots.sh publishes the recordings at
    const SHOWCASE_SIZE: (usize, usize) = (80, 24);
    const SHOWCASE_FPS: u32 = 10;
    const SHOWCASE_FRAMES: usize = 70;

    fn showcase(recording: &Recording, t: &'static ThemeData) -> Vec<Frame> {
        let setup = Setup {
            seed: recording.seed.unwrap_or(1),
            reduced_motion: false,
            day: crate::SNAP_DAY,
            practice: false,
        };
        app::record(
            &recording.game,
            setup,
            &recording.keys,
            SHOWCASE_SIZE,
            t,
            app::Take {
                start: recording.start,
                fps: SHOWCASE_FPS,
                count: SHOWCASE_FRAMES,
            },
        )
        .unwrap()
    }

    #[test]
    fn a_recording_is_the_same_every_time() {
        let script = parse(BUILT_IN).unwrap();
        let t = theme::find("deep-dark").unwrap();
        for recording in &script {
            let a = snapshot::frames_json(&showcase(recording, t), SHOWCASE_FPS, t);
            let b = snapshot::frames_json(&showcase(recording, t), SHOWCASE_FPS, t);
            assert_eq!(a, b, "{} differs between runs", recording.game);
        }
    }

    #[test]
    fn every_recording_keeps_the_flash_limit_in_every_theme() {
        let script = parse(BUILT_IN).unwrap();
        for t in theme::all() {
            for recording in &script {
                let frames = showcase(recording, t);
                assert_eq!(frames.len(), SHOWCASE_FRAMES);
                let measured: Vec<Vec<f32>> = frames.iter().map(|f| flash::regions(f, t)).collect();
                let most = flash::most_changes_per_second(&measured, SHOWCASE_FPS as usize);
                assert!(
                    most <= flash::MAX_CHANGES_PER_SECOND,
                    "{} in {} changes {most} times a second",
                    recording.game,
                    t.name
                );
            }
        }
    }

    #[test]
    fn every_recording_shows_play_rather_than_a_still() {
        let script = parse(BUILT_IN).unwrap();
        let t = theme::find("deep-dark").unwrap();
        for recording in &script {
            let frames = showcase(recording, t);
            let mut texts: Vec<String> = frames.iter().map(Frame::to_text).collect();
            texts.dedup();
            // a recording that sits still for most of its length shows nobody playing
            assert!(
                texts.len() >= 5,
                "{} changes only {} times",
                recording.game,
                texts.len()
            );
        }
    }

    #[test]
    fn the_recorder_keeps_time_and_skips_the_start() {
        let t = theme::find("deep-dark").unwrap();
        let setup = Setup {
            seed: 1,
            reduced_motion: false,
            day: crate::SNAP_DAY,
            practice: false,
        };
        // a key that waits two seconds at 10 frames a second spans 20 frames, so the frame after
        // it is the first to show the next key
        let keys = [(Key::None, 2.0), (Key::Right, 1.0)];
        let take = |start, count| app::Take {
            start,
            fps: 10,
            count,
        };
        let frames = app::record("trace", setup, &keys, (80, 24), t, take(0.0, 25)).unwrap();
        assert_eq!(frames.len(), 25);
        assert_eq!(frames[0].to_text(), frames[19].to_text());
        assert_ne!(frames[19].to_text(), frames[20].to_text());
        // skipping the first two seconds starts on that move
        let skipped = app::record("trace", setup, &keys, (80, 24), t, take(2.0, 5)).unwrap();
        assert_eq!(skipped[0].to_text(), frames[20].to_text());
    }

    #[test]
    fn a_script_reads_keys_typing_and_seeds() {
        let script = parse(
            "# a comment\n[stack]\nseed 4\nstart 1.5\nleft, enter, wait:0.5\n\n[touch-typing]\ntype 0.1 a b\n[rooms]\nstep 0.2 up*2 left\n",
        )
        .unwrap();
        assert_eq!(script.len(), 3);
        assert_eq!(
            script[2].keys,
            vec![(Key::Up, 0.2), (Key::Up, 0.2), (Key::Left, 0.2)]
        );
        assert_eq!(script[0].game, "stackline");
        assert_eq!(script[0].seed, Some(4));
        assert_eq!(script[0].start, 1.5);
        assert_eq!(script[0].keys, vec![(Key::Left, 0.0), (Key::Enter, 0.5)]);
        assert_eq!(
            script[1].keys,
            vec![
                (Key::Char('a'), 0.1),
                (Key::Char(' '), 0.1),
                (Key::Char('b'), 0.1)
            ]
        );
    }

    #[test]
    fn a_bad_script_names_the_line() {
        assert!(parse("left").unwrap_err().contains("line 1"));
        assert!(parse("[nothing]").unwrap_err().contains("nothing"));
        assert!(parse("[relay]\n[relay]").unwrap_err().contains("twice"));
        assert!(parse("[relay]\nsideways").unwrap_err().contains("line 2"));
        assert!(parse("[relay]\ntype x hi").unwrap_err().contains("wait"));
        assert!(parse("[relay]\nstart -1").unwrap_err().contains("start"));
        assert!(
            parse("[relay]\nstep 0.1 up*0")
                .unwrap_err()
                .contains("count")
        );
    }

    #[test]
    fn the_built_in_script_records_every_game() {
        let script = parse(BUILT_IN).unwrap();
        for info in crate::games::ALL {
            assert!(
                find(&script, info.id).is_some(),
                "{} has no recording",
                info.id
            );
        }
    }
}
