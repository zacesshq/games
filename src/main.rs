//! Zacess Games: fourteen small original games for the terminal in the Zacess themes.

mod app;
mod game;
mod games;
mod keyboard;
mod levels;
mod music;
mod record;
mod scores;
mod sound;
mod term;
mod ui;

use std::path::PathBuf;
use std::process::ExitCode;

use zacess_scenes::colour::ColourDepth;
use zacess_scenes::motion;
use zacess_scenes::rng::Rng;
use zacess_scenes::snapshot;
use zacess_scenes::terminal::limited_font;
use zacess_scenes::theme::{self, Choice, ThemeData};

use crate::game::Setup;
use crate::scores::Scores;
use crate::sound::Sound;
use crate::term::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");
/// The day stills are drawn on (29 October 2026), so the daily Termle term and its date stay the
/// same from one day to the next and the committed stills only change when a game does.
const SNAP_DAY: u64 = 20_755;

const USAGE: &str = "\
Zacess Games: fourteen small original games for the terminal.

Usage:
  zacess-games                 the menu
  zacess-games <game>          one game: stackline, merge-gates, termle, signal,
                               trace, drift, rooms, relay, through-ball,
                               slipstream, array-guard, heatsink, touch-typing
                               or learn-to-code (a short start is enough)
  zacess-games list            every game with its best score
  zacess-games sounds [game]   plays every effect, then each game's music
  zacess-games snap [game|all] [--out DIR] [--theme NAME|all]
                               SVG stills of the games
  zacess-games snap [game|all] --frames N [--fps F] [--out DIR]
                               JSON frames of each game's recording
                               (scripts/recordings.txt) for a web page
  zacess-games --headless <game> [--keys KEYS] [--size WxH]
                               plays keys without a terminal and prints the screen

Options:
  --theme NAME        a Zacess theme (otherwise ZACESS_THEME, then the theme state file)
  --reduced-motion    slower falling and drifting, still marks instead of motion
                      (also ZACESS_REDUCED_MOTION=1 or the system setting)
  --no-sound          start with effects and music muted (also ZACESS_SOUND=0)
  --volume N          effects volume from 0 to 100 (also ZACESS_VOLUME)
  --no-music          start with the music off
  --music-volume N    music volume from 0 to 100 (also ZACESS_MUSIC_VOLUME)
  --seed N            a fixed seed, for the same game twice
  --colour DEPTH      truecolour or 256
  --basic             plain characters for fonts without geometric shapes
  --layout NAME       the keyboard for Touch Typing: uk-mac, uk-pc or us
                      (also ZACESS_KEYBOARD; otherwise the system's layout)
  --frames N          with snap, N frames of JSON instead of a still
  --fps F             frames a second for --frames, 1 to 30 (default 10)
  --script FILE       with --frames, a recording script instead of the
                      built-in one
  --version, --help

Effects start at volume 35 and music at 20; changes are remembered. In most
games Space pauses, ? shows how to play, + and - set the effects, [ and ] the
music and 0 mutes. The typing games pause with Esc and show help with F1.
Ctrl+C always quits.";

struct Args {
    command: Option<String>,
    target: Option<String>,
    theme: Option<String>,
    reduced: bool,
    no_sound: bool,
    volume: Option<u8>,
    no_music: bool,
    music_volume: Option<u8>,
    seed: Option<u64>,
    colour: Option<ColourDepth>,
    basic: bool,
    headless: Option<String>,
    keys: Option<String>,
    size: (usize, usize),
    out: Option<PathBuf>,
    frames: Option<usize>,
    fps: u32,
    script: Option<PathBuf>,
}

fn parse_args(raw: Vec<String>) -> Result<Args, String> {
    let mut a = Args {
        command: None,
        target: None,
        theme: None,
        reduced: false,
        no_sound: false,
        volume: None,
        no_music: false,
        music_volume: None,
        seed: None,
        colour: None,
        basic: false,
        headless: None,
        keys: None,
        size: (100, 30),
        out: None,
        frames: None,
        fps: 10,
        script: None,
    };
    let mut it = raw.into_iter();
    while let Some(arg) = it.next() {
        let mut value = |name: &str| it.next().ok_or_else(|| format!("{name} needs a value"));
        match arg.as_str() {
            "--theme" => a.theme = Some(value("--theme")?),
            "--reduced-motion" => a.reduced = true,
            "--no-sound" => a.no_sound = true,
            "--no-music" => a.no_music = true,
            "--music-volume" => {
                let v = value("--music-volume")?;
                a.music_volume = Some(
                    zacess_scenes::sound::parse_volume(&v)
                        .ok_or_else(|| format!("--music-volume takes 0 to 100, not \"{v}\""))?,
                );
            }
            "--basic" => a.basic = true,
            "--volume" => {
                let v = value("--volume")?;
                a.volume = Some(
                    zacess_scenes::sound::parse_volume(&v)
                        .ok_or_else(|| format!("--volume takes 0 to 100, not \"{v}\""))?,
                );
            }
            "--seed" => {
                let v = value("--seed")?;
                a.seed = Some(
                    v.parse()
                        .map_err(|_| format!("--seed takes a number, not \"{v}\""))?,
                );
            }
            "--colour" | "--color" => {
                let v = value("--colour")?;
                a.colour = Some(
                    ColourDepth::parse(&v)
                        .ok_or_else(|| format!("--colour takes truecolour or 256, not \"{v}\""))?,
                );
            }
            "--layout" => {
                let v = value("--layout")?;
                keyboard::set_flag(
                    keyboard::Layout::parse(&v).ok_or_else(|| {
                        format!("--layout takes uk-mac, uk-pc or us, not \"{v}\"")
                    })?,
                );
            }
            "--headless" => a.headless = Some(value("--headless")?),
            "--keys" => a.keys = Some(value("--keys")?),
            "--size" => {
                let v = value("--size")?;
                let (w, h) = v
                    .split_once('x')
                    .and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?)))
                    .ok_or_else(|| format!("--size takes WIDTHxHEIGHT, not \"{v}\""))?;
                a.size = (w, h);
            }
            "--out" => a.out = Some(PathBuf::from(value("--out")?)),
            "--frames" => {
                let v = value("--frames")?;
                a.frames = Some(
                    v.parse()
                        .ok()
                        .filter(|n| (1..=3600).contains(n))
                        .ok_or_else(|| format!("--frames takes 1 to 3600, not \"{v}\""))?,
                );
            }
            "--fps" => {
                let v = value("--fps")?;
                a.fps = v
                    .parse()
                    .ok()
                    .filter(|n| (1..=30).contains(n))
                    .ok_or_else(|| format!("--fps takes 1 to 30, not \"{v}\""))?;
            }
            "--script" => a.script = Some(PathBuf::from(value("--script")?)),
            s if s.starts_with('-') && s != "-" => return Err(format!("unknown option {s}")),
            _ => {
                if a.command.is_none() {
                    a.command = Some(arg);
                } else if a.target.is_none() {
                    a.target = Some(arg);
                } else {
                    return Err(format!("unexpected \"{arg}\""));
                }
            }
        }
    }
    Ok(a)
}

fn choose_theme(flag: Option<&str>) -> Result<&'static ThemeData, String> {
    let env = std::env::var(theme::ENV_VAR).ok();
    let state = theme::read_state_file();
    let prefers_dark =
        !matches!(state, Some(theme::ThemeState::Auto { .. })) || motion::system_prefers_dark();
    theme::choose(&Choice {
        flag,
        env: env.as_deref(),
        state_file: state,
        prefers_dark,
    })
}

fn today() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() / 86_400)
}

/// Keys written as words and letters split by commas, such as `right,right,enter,wait:1.5`.
fn parse_keys(text: &str) -> Result<Vec<(Key, f32)>, String> {
    let mut keys: Vec<(Key, f32)> = Vec::new();
    for token in text.split(',').map(str::trim).filter(|t| !t.is_empty()) {
        if let Some(secs) = token.strip_prefix("wait:") {
            let s: f32 = secs.parse().map_err(|_| format!("bad wait \"{token}\""))?;
            match keys.last_mut() {
                Some(last) => last.1 += s,
                None => keys.push((Key::None, s)),
            }
            continue;
        }
        let key = match token.to_ascii_lowercase().as_str() {
            "up" => Key::Up,
            "down" => Key::Down,
            "left" => Key::Left,
            "right" => Key::Right,
            "enter" => Key::Enter,
            "tab" => Key::Tab,
            "space" => Key::Pause,
            "bs" | "backspace" => Key::Backspace,
            "help" => Key::Help,
            "esc" => Key::Esc,
            "ctrlc" => Key::Quit,
            "f1" => Key::Help,
            _ if token.chars().count() == 1 => Key::Char(token.chars().next().unwrap_or(' ')),
            _ => return Err(format!("unknown key \"{token}\"")),
        };
        keys.push((key, 0.0));
    }
    Ok(keys)
}

fn snap(args: &Args, setup: Setup) -> Result<(), String> {
    if let Some(count) = args.frames {
        return snap_frames(args, setup, count);
    }
    let out = args.out.clone().unwrap_or_else(|| PathBuf::from("."));
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let themes: Vec<&'static ThemeData> = match args.theme.as_deref() {
        Some("all") => theme::all().iter().collect(),
        other => vec![choose_theme(other)?],
    };
    let all = args.target.as_deref().is_none_or(|n| n == "all");
    let wanted: Vec<&'static game::Info> = match args.target.as_deref() {
        None | Some("all") => games::ALL.to_vec(),
        Some(name) => vec![games::find(name).ok_or_else(|| format!("no game called \"{name}\""))?],
    };
    for t in themes {
        if all {
            let frame = app::headless_menu(args.size, t, 0, false);
            let label = format!("The Zacess Games menu in {}", t.name);
            let path = out.join(format!("menu-{}.svg", t.short()));
            std::fs::write(&path, snapshot::svg(&frame, t, "zacess-games", &label))
                .map_err(|e| e.to_string())?;
            println!("{}", path.display());
        }
        for info in &wanted {
            let demo = games::make(info.id, setup)
                .map(|g| g.demo())
                .unwrap_or_default();
            let (frame, _) = app::headless(info.id, setup, &demo, args.size, t, None)
                .ok_or_else(|| format!("{} would not start", info.id))?;
            let label = format!("{} in {}, part way through a game", info.name, t.name);
            let path = out.join(format!("{}-{}.svg", info.id, t.short()));
            std::fs::write(
                &path,
                snapshot::svg(&frame, t, &format!("zacess-games {}", info.id), &label),
            )
            .map_err(|e| e.to_string())?;
            println!("{}", path.display());
        }
    }
    Ok(())
}

/// Each game's recording as JSON frames in the format Zacess Scenes uses, so the same player
/// shows both on a web page.
fn snap_frames(args: &Args, setup: Setup, count: usize) -> Result<(), String> {
    let out = args.out.clone().unwrap_or_else(|| PathBuf::from("."));
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    if args.theme.as_deref() == Some("all") {
        return Err("--frames records one theme at a time".into());
    }
    let t = choose_theme(args.theme.as_deref())?;
    let text = match &args.script {
        Some(path) => std::fs::read_to_string(path)
            .map_err(|e| format!("{} could not be read: {e}", path.display()))?,
        None => record::BUILT_IN.to_string(),
    };
    let script = record::parse(&text)?;
    let wanted: Vec<&'static game::Info> = match args.target.as_deref() {
        None | Some("all") => games::ALL.to_vec(),
        Some(name) => vec![games::find(name).ok_or_else(|| format!("no game called \"{name}\""))?],
    };
    if args.keys.is_some() && wanted.len() > 1 {
        return Err("--keys records one game at a time".into());
    }
    for info in wanted {
        // keys given on the command line replace the recording, its seed and start included
        let recording = record::find(&script, info.id).filter(|_| args.keys.is_none());
        let setup = Setup {
            seed: recording.and_then(|r| r.seed).unwrap_or(setup.seed),
            ..setup
        };
        let keys = match (&args.keys, recording) {
            (Some(keys), _) => parse_keys(keys)?,
            (None, Some(r)) => r.keys.clone(),
            (None, None) => games::make(info.id, setup)
                .map(|g| g.demo())
                .unwrap_or_default(),
        };
        let take = app::Take {
            start: recording.map_or(0.0, |r| r.start),
            fps: args.fps,
            count,
        };
        let frames = app::record(info.id, setup, &keys, args.size, t, take)
            .ok_or_else(|| format!("{} would not start", info.id))?;
        let path = out.join(format!("{}.json", info.id));
        std::fs::write(&path, snapshot::frames_json(&frames, args.fps, t))
            .map_err(|e| e.to_string())?;
        println!("{}", path.display());
    }
    Ok(())
}

fn real_main() -> Result<(), String> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    if raw.iter().any(|a| a == "--help" || a == "-h") {
        println!("{USAGE}");
        return Ok(());
    }
    if raw.iter().any(|a| a == "--version" || a == "-V") {
        println!("zacess-games {VERSION}");
        return Ok(());
    }
    let args = parse_args(raw)?;
    let basic = args.basic || limited_font(|name| std::env::var(name).ok());
    ui::set_basic(basic);
    let reduced = motion::reduced(
        args.reduced,
        std::env::var(motion::ENV_VAR).ok().as_deref(),
        motion::system_reduced_motion,
    );
    let setup = Setup {
        seed: args.seed.unwrap_or_else(|| Rng::from_time().next_u64()),
        reduced_motion: reduced,
        day: today(),
        practice: false,
    };

    if let Some(name) = &args.headless {
        let info = games::find(name).ok_or_else(|| format!("no game called \"{name}\""))?;
        let t = choose_theme(args.theme.as_deref())?;
        let keys = parse_keys(args.keys.as_deref().unwrap_or(""))?;
        let setup = Setup {
            seed: args.seed.unwrap_or(1),
            ..setup
        };
        let (frame, session) = app::headless(info.id, setup, &keys, args.size, t, None)
            .ok_or_else(|| format!("{} would not start", info.id))?;
        print!("{}", frame.to_text());
        println!("score {}", session.game.score());
        return Ok(());
    }

    match args.command.as_deref() {
        Some("list" | "scores") => {
            let scores = Scores::load();
            for info in games::ALL {
                let best = scores.get(info.id).map_or("no score yet".to_string(), |b| {
                    format!("best {b} {}", info.unit)
                });
                println!("{:<12} {:<46} {best}", info.id, info.blurb);
            }
            Ok(())
        }
        Some("sounds") => sounds(&args),
        Some("snap") => snap(
            &args,
            Setup {
                seed: args.seed.unwrap_or(1),
                reduced_motion: false,
                day: SNAP_DAY,
                ..setup
            },
        ),
        Some("menu") | None => play(None, &args, setup),
        Some(name) => {
            let info = games::find(name).ok_or_else(|| {
                format!("no game called \"{name}\". zacess-games list shows them all.")
            })?;
            play(Some(info), &args, setup)
        }
    }
}

fn sound_from(args: &Args, scores: &Scores) -> Sound {
    Sound::from_env(
        sound::Choice {
            no_sound: args.no_sound,
            volume: args.volume,
            no_music: args.no_music,
            music_volume: args.music_volume,
        },
        scores,
    )
}

/// Plays every effect and then a little of each game's music, naming each one, so sound can be
/// checked outside a game.
fn sounds(args: &Args) -> Result<(), String> {
    let scores = Scores::load();
    let sound = sound_from(args, &scores);
    if sound.player().is_none() {
        return Err(
            "no sound player was found (afplay, pw-play, paplay, aplay or PowerShell)".into(),
        );
    }
    println!("{}", sound.status());
    let only = args.target.as_deref();
    if only.is_none() {
        println!("Effects at {}%:", sound.volume);
        for sfx in sound::Sfx::SAMPLER {
            println!("  {:<6} {}", sfx.name(), sfx.meaning());
            if let Some(mut child) = sound.start(sfx) {
                let _ = child.wait();
            }
            std::thread::sleep(std::time::Duration::from_millis(350));
        }
    }
    let wanted: Vec<&'static game::Info> = match only {
        None | Some("all") => games::ALL.to_vec(),
        Some(name) => vec![games::find(name).ok_or_else(|| format!("no game called \"{name}\""))?],
    };
    println!(
        "Music at {}%, eight seconds of each:",
        sound.music_volume.max(1)
    );
    for info in wanted {
        let m = music::mood(info.id);
        println!("  {:<14} {:.0} beats a minute", info.name, m.tempo);
        let file = music::ensure_file(&sound::cache_dir(), info.id, sound.music_volume.max(1))
            .map_err(|e| e.to_string())?;
        if let Some(player) = sound.player()
            && let Ok(mut child) = sound::play_command(player, &file)
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
        {
            std::thread::sleep(std::time::Duration::from_secs(8));
            let _ = child.kill();
            let _ = child.wait();
        }
    }
    Ok(())
}

fn play(start: Option<&'static game::Info>, args: &Args, setup: Setup) -> Result<(), String> {
    let t = choose_theme(args.theme.as_deref())?;
    let scores = Scores::load();
    let depth = args
        .colour
        .unwrap_or_else(|| ColourDepth::detect(|name| std::env::var(name).ok()));
    app::run(
        start,
        app::Options {
            theme: t,
            depth,
            setup,
            sound: sound_from(args, &scores),
            scores,
        },
    )
    .map_err(|e| format!("the terminal could not be used: {e}"))
}

fn main() -> ExitCode {
    match real_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("zacess-games: {message}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_parse_with_waits() {
        let keys = parse_keys("right, x, wait:1.5, enter").unwrap();
        assert_eq!(
            keys,
            vec![(Key::Right, 0.0), (Key::Char('x'), 1.5), (Key::Enter, 0.0)]
        );
        assert!(parse_keys("sideways").is_err());
    }

    #[test]
    fn options_parse() {
        let a = parse_args(
            [
                "trace",
                "--theme",
                "deep-light",
                "--seed",
                "4",
                "--size",
                "100x30",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        )
        .ok()
        .unwrap();
        assert_eq!(a.command.as_deref(), Some("trace"));
        assert_eq!(a.seed, Some(4));
        assert_eq!(a.size, (100, 30));
        assert!(parse_args(vec!["--volume".into(), "300".into()]).is_err());
    }

    #[test]
    fn frame_options_parse_and_check_their_range() {
        let args = |list: &[&str]| parse_args(list.iter().map(|s| s.to_string()).collect());
        let a = args(&["snap", "relay", "--frames", "70", "--fps", "12"])
            .ok()
            .unwrap();
        assert_eq!(a.frames, Some(70));
        assert_eq!(a.fps, 12);
        assert_eq!(args(&["snap"]).ok().unwrap().fps, 10);
        assert!(args(&["--frames", "0"]).is_err());
        assert!(args(&["--fps", "31"]).is_err());
        assert!(args(&["--fps", "fast"]).is_err());
    }

    #[test]
    fn every_game_draws_headless_in_light_dark_and_high_contrast() {
        for id in ["dark", "light", "deep-dark", "hc-light", "yellow"] {
            let t = theme::find(id).unwrap_or_else(|| panic!("{id} is not a theme"));
            for info in games::ALL {
                let setup = Setup {
                    seed: 3,
                    reduced_motion: false,
                    day: 20_000,
                    practice: false,
                };
                let demo = games::make(info.id, setup).unwrap().demo();
                let (frame, _) = app::headless(info.id, setup, &demo, (100, 30), t, None).unwrap();
                let text = frame.to_text();
                assert!(text.contains(info.name), "{} in {}", info.id, t.name);
                // the window, the status bar and the key hints are all there
                assert!(text.contains('╭') && text.contains('╯'));
                assert!(text.contains("Best"));
                assert!(frame.drawn() > 300, "{} draws too little", info.id);
                let svg = snapshot::svg(&frame, t, info.name, info.name);
                assert!(svg.starts_with("<svg"));
            }
        }
    }
}
