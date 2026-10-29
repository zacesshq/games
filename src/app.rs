//! The shared loop: the menu and a running game with pause, the How to play page, the sound and
//! music keys and the end screen. Also the headless runs the tests and the stills use.

use std::time::{Duration, Instant};

use zacess_scenes::colour::ColourDepth;
use zacess_scenes::flash::{Meter, Verdict};
use zacess_scenes::frame::Frame;
use zacess_scenes::theme::ThemeData;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::games;
use crate::music;
use crate::scores::Scores;
use crate::sound::Sound;
use crate::term::{Key, Term};
use crate::ui::{self, Canvas, Chrome, Line, Palette, Rect, Status, g, overlay};

/// Frames a second while a game moves on its own.
const FPS: u64 = 30;
/// How long a written note stays above the key lines.
const TOAST_SECONDS: f32 = 2.5;
/// Where the full controls sheet lives, named on every How to play page.
pub const CONTROLS_DOC: &str = "docs/controls.md";

/// The line of keys every game shares, by how the game reads the keyboard.
pub fn common_keys(input: Input) -> &'static str {
    match input {
        Input::Keys => "Space pause · ? help · + - fx · [ ] music · 0 mute · Q quit",
        Input::Letters => "Space pause · ? help · + - fx · [ ] music · 0 mute · Esc out",
        Input::Text => "Esc pause and sound keys · F1 help · Ctrl+C quit",
    }
}

/// The shared keys in a few words, for the one key line of a compact window.
pub fn short_keys(input: Input) -> &'static str {
    match input {
        Input::Keys => "Space pause · ? help",
        Input::Letters => "Space pause · ? help",
        Input::Text => "Esc pause · F1 help",
    }
}

/// Draws a game into the play area. On a large window a game that allows it is drawn at half
/// the size and then stretched to twice, so its board fills the room. Returns how many cells
/// the game tried to draw outside its area, which the tests hold at none.
pub fn draw_game(c: &mut Canvas, game: &dyn Game, area: Rect) -> usize {
    if c.tier == ui::Tier::Large && game.zoomable() {
        let (zw, zh) = ((area.w / 2).max(1) as usize, (area.h / 2).max(1) as usize);
        let mut small = Frame::new(zw, zh);
        let outside = {
            let mut sc = Canvas::new(&mut small, c.pal);
            // the half-size drawing is laid out as a standard window would be
            sc.tier = ui::Tier::Standard;
            game.draw(&mut sc, Rect::new(0, 0, zw as i32, zh as i32));
            sc.outside
        };
        let x = area.x + (area.w - 2 * zw as i32) / 2;
        let y = area.y + (area.h - 2 * zh as i32) / 2;
        ui::zoom(&small, c.frame, (x, y));
        return outside;
    }
    if let Some((nw, nh)) = game.natural_size()
        && (area.w < nw || area.h < nh)
    {
        let (fw, fh) = (nw.max(area.w), nh.max(area.h));
        let mut full = Frame::new(fw as usize, fh as usize);
        let outside = {
            let mut fc = Canvas::new(&mut full, c.pal);
            fc.tier = c.tier;
            game.draw(&mut fc, Rect::new(0, 0, fw, fh));
            fc.outside
        };
        let (ax, ay) = game.crop_anchor();
        let sx = ((fw - area.w) as f32 * ax).round() as i32;
        let sy = ((fh - area.h) as f32 * ay).round() as i32;
        for y in 0..area.h {
            for x in 0..area.w {
                if let Some(cell) = full.get(sx + x, sy + y)
                    && cell.ch != ' '
                {
                    c.frame.put(area.x + x, area.y + y, cell.ch, cell.colour);
                }
            }
        }
        return outside;
    }
    let before = c.outside;
    let clip = c.clip;
    c.clip = area;
    game.draw(c, area);
    c.clip = clip;
    c.outside - before
}

/// The How to play page of a game: its own sections, then the keys every game shares and where
/// the full sheet is.
pub fn how_to_play(info: &Info) -> Vec<String> {
    let mut lines: Vec<String> = info.help.iter().map(|s| s.to_string()).collect();
    lines.push("# Keys every game shares".into());
    match info.input {
        Input::Keys | Input::Letters => {
            lines.push("- Space pauses and carries on.".into());
            lines.push("- ? opens this page. ↑ and ↓ scroll it.".into());
        }
        Input::Text => {
            lines.push(
                "- Space and every symbol are typed, so Esc pauses instead. While paused the sound keys below work and Q leaves."
                    .into(),
            );
            lines.push("- F1 opens this page. ↑ and ↓ scroll it.".into());
        }
    }
    lines.push("- + and - set the effects volume. ] and [ set the music volume.".into());
    lines.push("- 0 mutes everything and brings it back.".into());
    lines.push(match info.input {
        Input::Keys => "- Q or Esc leaves. Ctrl+C leaves at once.".into(),
        Input::Letters => "- Esc leaves (Q is a letter here). Ctrl+C leaves at once.".into(),
        Input::Text => "- Ctrl+C leaves at once.".into(),
    });
    lines.push("- Enter plays again once a game is over.".into());
    lines.push(format!(
        "The full controls sheet is {CONTROLS_DOC} in the Zacess Games repository."
    ));
    lines
}

/// One game being played, with everything around it.
pub struct Session {
    pub game: Box<dyn Game>,
    pub setup: Setup,
    pub paused: bool,
    /// The How to play page and how far it is scrolled, when open.
    pub help: Option<usize>,
    toast: Option<(String, f32)>,
    pub best: Option<u64>,
    /// Cells the game tried to draw outside its area on the last frame.
    pub outside: usize,
    new_best: bool,
    recorded: bool,
}

/// What a key did to the session.
#[derive(Debug, PartialEq, Eq)]
pub enum Flow {
    Stay,
    /// Back to the menu. Out when the game was started by name.
    Leave,
}

/// The sound and music keys, shared by the menu and every game. Returns a note about the
/// change. `None` when the key is not one of them.
pub fn sound_key(key: Key, sound: &mut Sound, scores: &mut Scores) -> Option<String> {
    match key {
        Key::Char('+' | '=') => sound.change(true),
        Key::Char('-' | '_') => sound.change(false),
        Key::Char(']') => sound.change_music(true),
        Key::Char('[') => sound.change_music(false),
        Key::Char('0') => sound.toggle_all(),
        _ => return None,
    }
    sound.save(scores);
    Some(sound.status())
}

impl Session {
    pub fn new(mut game: Box<dyn Game>, setup: Setup, scores: &Scores) -> Self {
        game.restore(scores);
        let best = scores.get(&game.score_key());
        Self {
            game,
            setup,
            paused: false,
            help: None,
            toast: None,
            best,
            outside: 0,
            new_best: false,
            recorded: false,
        }
    }

    fn apply(&mut self, fx: Fx, sound: &Sound) {
        for s in fx.sounds {
            sound.play(s);
        }
        if let Some(t) = fx.toast {
            self.toast = Some((t, TOAST_SECONDS));
        }
    }

    /// Keeps the score once, when the game ends or is left.
    pub fn record(&mut self, scores: &mut Scores) {
        if self.recorded {
            return;
        }
        self.recorded = true;
        self.game.keep(scores);
        let score = self.game.score();
        if scores.offer(&self.game.score_key(), score) {
            self.new_best = true;
        }
    }

    /// Whether music should play now: not while paused, reading the page or after the end.
    pub fn music_wanted(&self) -> bool {
        !self.paused && self.help.is_none() && self.game.outcome().is_none()
    }

    fn leave(&mut self, scores: &mut Scores) -> Flow {
        self.record(scores);
        Flow::Leave
    }

    pub fn key(&mut self, key: Key, sound: &mut Sound, scores: &mut Scores) -> Flow {
        let info = self.game.info();
        let input = info.input;
        if key == Key::Quit {
            return self.leave(scores);
        }
        // the How to play page scrolls with the arrows; any other key closes it
        if let Some(scroll) = self.help {
            match key {
                Key::Up | Key::Char('k') => self.help = Some(scroll.saturating_sub(1)),
                Key::Down | Key::Char('j') => self.help = Some(scroll + 1),
                Key::None | Key::Resize(..) => {}
                _ => self.help = None,
            }
            return Flow::Stay;
        }
        let help_key = match input {
            Input::Text => key == Key::Help,
            _ => matches!(key, Key::Help | Key::Char('?')),
        };
        if help_key {
            self.help = Some(0);
            return Flow::Stay;
        }
        // in a game that takes any character the sound keys work only while paused
        let sound_now = input != Input::Text || self.paused || self.game.outcome().is_some();
        if sound_now && let Some(note) = sound_key(key, sound, scores) {
            self.toast = Some((note, TOAST_SECONDS));
            return Flow::Stay;
        }
        let pause_key = match input {
            Input::Text => key == Key::Esc,
            _ => key == Key::Pause,
        };
        let quit_key = match input {
            Input::Keys => matches!(key, Key::Esc | Key::Char('q' | 'Q')),
            Input::Letters => key == Key::Esc,
            Input::Text => self.paused && matches!(key, Key::Char('q' | 'Q')),
        };
        if self.paused {
            if quit_key {
                return self.leave(scores);
            }
            // any other key carries on, so nobody is stuck behind the pause box
            if !matches!(key, Key::None | Key::Resize(..)) {
                self.paused = false;
            }
            return Flow::Stay;
        }
        if self.game.outcome().is_some() {
            if quit_key || key == Key::Esc {
                return self.leave(scores);
            }
            if key == Key::Enter {
                self.record(scores);
                let next = Setup {
                    seed: self
                        .setup
                        .seed
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1),
                    practice: true,
                    ..self.setup
                };
                if let Some(game) = games::make(info.id, next) {
                    *self = Session::new(game, next, scores);
                }
            }
            return Flow::Stay;
        }
        if pause_key {
            self.paused = true;
            return Flow::Stay;
        }
        if quit_key {
            return self.leave(scores);
        }
        // a typing game gets the space bar as a character
        let key = match (input, key) {
            (Input::Text, Key::Pause) => Key::Char(' '),
            (_, k) => k,
        };
        let mut fx = Fx::default();
        self.game.key(key, &mut fx);
        self.apply(fx, sound);
        if self.game.outcome().is_some() {
            self.record(scores);
        }
        Flow::Stay
    }

    pub fn tick(&mut self, dt: f32, sound: &Sound, scores: &mut Scores) {
        if let Some((_, left)) = &mut self.toast {
            *left -= dt;
            if *left <= 0.0 {
                self.toast = None;
            }
        }
        if self.paused || self.help.is_some() || self.game.outcome().is_some() {
            return;
        }
        let mut fx = Fx::default();
        self.game.tick(dt, &mut fx);
        self.apply(fx, sound);
        if self.game.outcome().is_some() {
            self.record(scores);
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, pal: Palette, sound: &Sound) {
        frame.clear();
        let mut c = Canvas::new(frame, pal);
        let (w, h) = (c.frame.width(), c.frame.height());
        if w < ui::MIN_WIDTH || h < ui::MIN_HEIGHT {
            ui::too_small(&mut c);
            return;
        }
        let info = self.game.info();
        let identity = pal.identity(info.hue);
        let mut status: Status = self.game.status();
        status.push((
            "Best".into(),
            self.best.map_or("-".into(), |b| b.to_string()),
        ));
        let sound_label = sound.status();
        let chrome = Chrome {
            title: info.name,
            icon: g(info.icon.0, info.icon.1),
            identity,
            status: &status,
            hints: info.hints,
            common: common_keys(info.input),
            common_short: short_keys(info.input),
            sound: &sound_label,
            toast: self.toast.as_ref().map(|(t, _)| t.as_str()),
        };
        let area = ui::chrome(&mut c, &chrome);
        self.outside = draw_game(&mut c, self.game.as_ref(), area);
        if let Some(scroll) = self.help {
            let most = ui::page(
                &mut c,
                area,
                &format!("How to play {}", info.name),
                &how_to_play(info),
                scroll,
                identity,
            );
            self.help = Some(scroll.min(most));
        } else if self.paused {
            let lines = vec![
                format!("{} is paused.", info.name),
                format!("Sound now: {}", sound.status()),
                "+ and - effects · [ and ] music · 0 mute all".into(),
                match info.input {
                    Input::Letters => "Esc leaves · any other key carries on".into(),
                    _ => "Q leaves · any other key carries on".into(),
                },
            ];
            overlay(&mut c, area, "Paused", &lines, identity);
        } else if let Some(outcome) = self.game.outcome() {
            let (heading, text, colour) = match &outcome {
                Outcome::Won(t) => ("Well done", t.clone(), pal.green),
                Outcome::Lost(t) => ("Game over", t.clone(), pal.warning),
            };
            let mut lines = vec![text, format!("Score: {} {}", self.game.score(), info.unit)];
            if self.new_best {
                lines.push(format!("{} A new best.", g('★', '*')));
            }
            lines.push(String::new());
            lines.push("Enter plays again · Esc goes back".into());
            overlay(&mut c, area, heading, &lines, colour);
        }
    }
}

/// The menu of games: two columns of names with the chosen game's description underneath.
pub struct Menu {
    pub selected: usize,
    /// The How to play page of the chosen game and its scroll, while it is open.
    pub help: Option<usize>,
}

impl Menu {
    pub fn rows() -> usize {
        games::ALL.len().div_ceil(2)
    }

    pub fn draw(&mut self, frame: &mut Frame, pal: Palette, scores: &Scores, sound: &str) {
        frame.clear();
        let mut c = Canvas::new(frame, pal);
        let (w, h) = (c.frame.width(), c.frame.height());
        if w < ui::MIN_WIDTH || h < ui::MIN_HEIGHT {
            ui::too_small(&mut c);
            return;
        }
        let status: Status = vec![
            ("Games".into(), games::ALL.len().to_string()),
            ("Theme".into(), pal.theme.short().into()),
        ];
        let chrome = Chrome {
            title: "Menu",
            icon: g('◆', '*'),
            identity: pal.accent,
            status: &status,
            hints: "Arrows choose · Enter play · 1 to 9 jump",
            common: "? how to play · + - fx · [ ] music · 0 mute · Q quit",
            common_short: "? help · Q quit",
            sound,
            toast: None,
        };
        let area = ui::chrome(&mut c, &chrome);
        let rows = Self::rows() as i32;
        let box_w = area.w.min(78);
        let r = area.centre(box_w, area.h);
        c.titled(
            Rect::new(r.x, r.y, r.w, rows + 2),
            Line::Heavy,
            pal.accent,
            "Pick a game",
            pal.bright,
        );
        let col_w = (r.w - 2) / 2;
        for (i, info) in games::ALL.iter().enumerate() {
            let (col, row) = (i as i32 / rows, i as i32 % rows);
            let x = r.x + 1 + col * col_w;
            let y = r.y + 1 + row;
            let colour = pal.identity(info.hue);
            let chosen = i == self.selected;
            c.put(x + 1, y, if chosen { g('▶', '>') } else { ' ' }, pal.bright);
            c.text(x + 2, y, &format!("{:>2}", i + 1), pal.muted);
            c.put(x + 5, y, g(info.icon.0, info.icon.1), colour);
            let name = if chosen {
                format!("[{}]", info.name)
            } else {
                format!(" {} ", info.name)
            };
            c.text(x + 6, y, &name, if chosen { pal.bright } else { colour });
            if let Some(best) = scores.get(info.id) {
                let text = best.to_string();
                let bx = x + col_w - 1 - text.chars().count() as i32;
                if bx > x + 7 + name.chars().count() as i32 {
                    c.text(bx, y, &text, if chosen { colour } else { pal.muted });
                }
            }
        }
        // the chosen game's description and goal, so the menu explains each game
        let info = games::ALL[self.selected];
        let colour = pal.identity(info.hue);
        let top = r.y + rows + 2;
        let room = r.y + r.h - top;
        if room >= 3 {
            let preview = Rect::new(r.x, top, r.w, room);
            c.titled(preview, Line::Light, colour, info.name, pal.bright);
            let mut text = vec![format!("{}.", capitalise(info.blurb))];
            let goal: Vec<String> = info
                .help
                .iter()
                .skip_while(|l| !l.starts_with("# Goal"))
                .skip(1)
                .take_while(|l| !l.starts_with("# "))
                .map(|s| s.to_string())
                .collect();
            text.extend(goal);
            let lines = ui::page_lines(&text, (r.w - 4) as usize);
            for (i, (_, line)) in lines.iter().take((room - 2) as usize).enumerate() {
                c.text(r.x + 2, top + 1 + i as i32, line, pal.text);
            }
        }
        if let Some(scroll) = self.help {
            let mut page = vec![
                "# The menu".to_string(),
                "- Arrows (or H J K L) choose, Enter plays, 1 to 9 jump to a game.".into(),
                "- + - [ ] and 0 set the sound here as in every game.".into(),
                "- Q or Esc leaves. Ctrl+C leaves at once.".into(),
            ];
            page.extend(how_to_play(info));
            let most = ui::page(
                &mut c,
                area,
                &format!("How to play {}", info.name),
                &page,
                scroll,
                colour,
            );
            self.help = Some(scroll.min(most));
        }
    }

    /// Moves the choice or returns the chosen game when Enter or a number is pressed.
    pub fn key(&mut self, key: Key) -> Option<&'static Info> {
        let n = games::ALL.len();
        let rows = Self::rows();
        if let Some(scroll) = self.help {
            match key {
                Key::Up | Key::Char('k') => self.help = Some(scroll.saturating_sub(1)),
                Key::Down | Key::Char('j') => self.help = Some(scroll + 1),
                Key::None | Key::Resize(..) => {}
                _ => self.help = None,
            }
            return None;
        }
        match key {
            Key::Up | Key::Char('k') => self.selected = (self.selected + n - 1) % n,
            Key::Down | Key::Char('j') | Key::Tab => self.selected = (self.selected + 1) % n,
            Key::Left | Key::Char('h') if self.selected >= rows => self.selected -= rows,
            Key::Right | Key::Char('l') if self.selected + rows < n => self.selected += rows,
            Key::Enter => return Some(games::ALL[self.selected]),
            Key::Help | Key::Char('?') => self.help = Some(0),
            Key::Char(d @ '1'..='9') => {
                let i = d as usize - '1' as usize;
                if i < n {
                    self.selected = i;
                    return Some(games::ALL[i]);
                }
            }
            _ => {}
        }
        None
    }
}

fn capitalise(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

/// Everything a live run needs.
pub struct Options {
    pub theme: &'static ThemeData,
    pub depth: ColourDepth,
    pub setup: Setup,
    pub sound: Sound,
    pub scores: Scores,
}

/// Plays one game (the menu when `start` is `None`) until the player leaves. The music stops
/// on every way out.
pub fn run(start: Option<&'static Info>, opts: Options) -> std::io::Result<()> {
    let result = run_inner(start, opts);
    music::stop();
    result
}

fn run_inner(start: Option<&'static Info>, mut opts: Options) -> std::io::Result<()> {
    let pal = Palette::new(opts.theme);
    let intro = match start {
        Some(info) if info.input == Input::Text => format!(
            "Zacess Games: {}. Esc pauses, F1 shows how to play, Ctrl+C quits.",
            info.name
        ),
        Some(info) => format!(
            "Zacess Games: {}. Space pauses, ? shows how to play, Esc quits.",
            info.name
        ),
        None => {
            "Zacess Games menu. Arrow keys choose, Enter plays, ? shows how to play, Esc quits."
                .to_string()
        }
    };
    let mut term = Term::open(
        &intro,
        opts.depth,
        opts.theme.background,
        opts.theme.floor(),
    )?;
    let (w, h) = term.size();
    let mut frame = Frame::new(w, h);
    let mut shown = frame.clone();
    let mut meter = Meter::new();
    let mut menu = Menu {
        selected: 0,
        help: None,
    };
    let mut session = start.and_then(|info| {
        games::make(info.id, opts.setup).map(|g| Session::new(g, opts.setup, &opts.scores))
    });
    let mut last = Instant::now();
    let mut seed = opts.setup.seed;
    loop {
        let now = Instant::now();
        let dt = (now - last).as_secs_f32().min(0.25);
        last = now;
        meter.advance(dt);
        if let Some(s) = &mut session {
            s.tick(dt, &opts.sound, &mut opts.scores);
            s.draw(&mut frame, pal, &opts.sound);
        } else {
            menu.draw(&mut frame, pal, &opts.scores, &opts.sound.status());
        }
        let playing = session
            .as_ref()
            .filter(|s| s.music_wanted())
            .map(|s| s.game.info().id);
        music::keep(&opts.sound, playing);
        // a frame that would change a large area too often is held back, as in Scenes
        if meter.judge(&frame, opts.theme) == Verdict::Show {
            shown.clone_from(&frame);
        }
        term.draw(&shown)?;

        let animated = session
            .as_ref()
            .is_some_and(|s| s.game.animated() && !s.paused && s.help.is_none());
        let wait = if animated {
            Duration::from_millis(1000 / FPS)
        } else {
            Duration::from_millis(250)
        };
        let deadline = Instant::now() + wait;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            let key = term.poll(left)?;
            match key {
                Key::None => {}
                Key::Resize(w, h) => {
                    frame = Frame::new(w, h);
                    shown = frame.clone();
                    meter.reset();
                    term.reset()?;
                }
                Key::Quit => return Ok(()),
                key => {
                    if let Some(s) = &mut session {
                        if s.key(key, &mut opts.sound, &mut opts.scores) == Flow::Leave {
                            if start.is_some() {
                                return Ok(());
                            }
                            session = None;
                            meter.reset();
                        }
                    } else if menu.help.is_none() && matches!(key, Key::Esc | Key::Char('q' | 'Q'))
                    {
                        return Ok(());
                    } else if (menu.help.is_some()
                        || sound_key(key, &mut opts.sound, &mut opts.scores).is_none())
                        && let Some(info) = menu.key(key)
                    {
                        seed = seed
                            .wrapping_mul(6364136223846793005)
                            .wrapping_add(1442695040888963407);
                        let setup = Setup { seed, ..opts.setup };
                        session = games::make(info.id, setup)
                            .map(|g| Session::new(g, setup, &opts.scores));
                        meter.reset();
                    }
                }
            }
            if Instant::now() >= deadline || !matches!(key, Key::None) {
                break;
            }
        }
    }
}

/// Plays keys into a game without a terminal and returns the last frame, for the tests, the
/// `--headless` flag and the stills. Each key is followed by its pause in seconds, ticked at 30
/// frames a second. `frames` collects every frame drawn when given, for the flash check.
pub fn headless(
    id: &str,
    setup: Setup,
    keys: &[(Key, f32)],
    size: (usize, usize),
    theme: &'static ThemeData,
    mut frames: Option<&mut Vec<Frame>>,
) -> Option<(Frame, Session)> {
    let pal = Palette::new(theme);
    let mut scores = Scores::in_memory();
    let mut sound = Sound::silent();
    let game = games::make(id, setup)?;
    let mut session = Session::new(game, setup, &scores);
    let mut frame = Frame::new(size.0, size.1);
    let dt = 1.0 / FPS as f32;
    for &(key, wait) in keys {
        session.key(key, &mut sound, &mut scores);
        let mut t = 0.0;
        loop {
            session.draw(&mut frame, pal, &sound);
            if let Some(f) = frames.as_deref_mut() {
                f.push(frame.clone());
            }
            if t >= wait {
                break;
            }
            session.tick(dt, &sound, &mut scores);
            t += dt;
        }
    }
    session.draw(&mut frame, pal, &sound);
    Some((frame, session))
}

/// When a recording's frames are taken: seconds played before the first is kept, frames a second
/// and how many.
#[derive(Clone, Copy, Debug)]
pub struct Take {
    pub start: f32,
    pub fps: u32,
    pub count: usize,
}

/// Plays `keys` without a terminal and keeps the frames `take` asks for, sampled on the game's
/// own clock, for a web page to play. Time only moves on through each key's wait, so a key with no
/// wait lands in the same frame as the next. When the keys run out the game carries on alone.
/// The first seconds of `take.start` are played but not kept, so a slow opening can be skipped.
/// Every frame goes through the flash meter at the recording's rate, as a Scenes recording
/// does: a frame that would change a large area too often is replaced by the one before it.
pub fn record(
    id: &str,
    setup: Setup,
    keys: &[(Key, f32)],
    size: (usize, usize),
    theme: &'static ThemeData,
    take: Take,
) -> Option<Vec<Frame>> {
    let Take { start, fps, count } = take;
    let pal = Palette::new(theme);
    let mut scores = Scores::in_memory();
    let mut sound = Sound::silent();
    let game = games::make(id, setup)?;
    let mut session = Session::new(game, setup, &scores);
    let mut frame = Frame::new(size.0, size.1);
    let fps = u64::from(fps).clamp(1, FPS);
    let dt = 1.0 / FPS as f32;
    let mut meter = Meter::new();
    let mut frames: Vec<Frame> = Vec::with_capacity(count);
    // whole ticks rather than summed seconds, so the sample times never drift with rounding
    let skip = (start.max(0.0) * FPS as f32).round() as u64;
    let mut tick: u64 = 0;
    let rest = std::iter::repeat((Key::None, f32::INFINITY));
    for (key, wait) in keys.iter().copied().chain(rest) {
        if frames.len() >= count {
            break;
        }
        session.key(key, &mut sound, &mut scores);
        let ticks = if wait.is_finite() {
            (wait.max(0.0) * FPS as f32).round() as u64
        } else {
            u64::MAX
        };
        let mut waited = 0;
        // a frame due on the tick a key lands is drawn after the key, so the key shows in it
        while waited < ticks && frames.len() < count {
            // sample n falls due on tick start + n * FPS / fps
            while tick >= skip
                && frames.len() < count
                && frames.len() as u64 * FPS <= (tick - skip) * fps
            {
                session.draw(&mut frame, pal, &sound);
                meter.advance(1.0 / fps as f32);
                match (meter.judge(&frame, theme), frames.last()) {
                    (Verdict::Hold, Some(last)) => frames.push(last.clone()),
                    _ => frames.push(frame.clone()),
                }
            }
            session.tick(dt, &sound, &mut scores);
            tick += 1;
            waited += 1;
        }
    }
    Some(frames)
}

/// The menu drawn without a terminal, with the chosen game's page open when `help` is set.
pub fn headless_menu(
    size: (usize, usize),
    theme: &'static ThemeData,
    selected: usize,
    help: bool,
) -> Frame {
    let mut frame = Frame::new(size.0, size.1);
    Menu {
        selected,
        help: help.then_some(0),
    }
    .draw(
        &mut frame,
        Palette::new(theme),
        &Scores::in_memory(),
        &Sound::silent().status(),
    );
    frame
}

#[cfg(test)]
mod tests {
    use super::*;
    use zacess_scenes::flash;
    use zacess_scenes::theme;

    fn setup() -> Setup {
        Setup {
            seed: 7,
            reduced_motion: false,
            day: 20_400,
            practice: false,
        }
    }

    fn pause_key(info: &Info) -> Key {
        if info.input == Input::Text {
            Key::Esc
        } else {
            Key::Pause
        }
    }

    #[test]
    fn the_menu_lists_every_game_and_explains_the_chosen_one() {
        let frame = headless_menu((80, 24), theme::default_theme(), 0, false);
        let text = frame.to_text();
        for info in games::ALL {
            assert!(text.contains(info.name), "{} missing", info.name);
        }
        assert!(text.contains("Pick a game"));
        assert!(text.contains("FX 35% · Music 20%"));
        let frame = headless_menu((64, 24), theme::default_theme(), 3, true);
        assert!(frame.to_text().contains("How to play"));
    }

    #[test]
    fn every_page_and_hint_fits_the_smallest_window() {
        for info in games::ALL {
            // a standard window shows both key lines whole; a compact one cuts the game's line at
            // a separator and adds the short shared keys, all within the width
            assert!(
                info.hints.chars().count() <= ui::STANDARD.0 - 4,
                "{}",
                info.id
            );
            assert!(common_keys(info.input).chars().count() <= ui::STANDARD.0 - 4);
            let compact = ui::fit_hints(info.hints, ui::MIN_WIDTH - 4);
            assert!(!compact.is_empty() && compact.chars().count() <= ui::MIN_WIDTH - 4);
            assert!(short_keys(info.input).chars().count() <= 24);
            assert!(info.blurb.chars().count() <= 30, "{} blurb", info.id);
            // every page has the five sections, in order
            let heads: Vec<&str> = info
                .help
                .iter()
                .filter_map(|l| l.strip_prefix("# "))
                .collect();
            assert_eq!(
                heads,
                ["Goal", "Rules", "Scoring", "Winning and losing", "Keys"],
                "{}",
                info.id
            );
        }
    }

    #[test]
    fn the_how_to_play_page_opens_scrolls_and_closes_in_every_game() {
        for info in games::ALL {
            let help = if info.input == Input::Text {
                Key::Help
            } else {
                Key::Char('?')
            };
            let (frame, s) = headless(
                info.id,
                setup(),
                &[(help, 0.0), (Key::Down, 0.0), (Key::Down, 0.0)],
                (64, 24),
                theme::default_theme(),
                None,
            )
            .unwrap();
            assert!(frame.to_text().contains("How to play"), "{}", info.id);
            assert!(s.help.is_some(), "{}", info.id);
            let (_, s) = headless(
                info.id,
                setup(),
                &[(help, 0.0), (Key::Enter, 0.0)],
                (64, 24),
                theme::default_theme(),
                None,
            )
            .unwrap();
            assert!(s.help.is_none(), "{}", info.id);
        }
    }

    #[test]
    fn every_game_pauses_with_its_pause_key() {
        for info in games::ALL {
            let (frame, s) = headless(
                info.id,
                setup(),
                &[(pause_key(info), 0.5)],
                (80, 24),
                theme::default_theme(),
                None,
            )
            .unwrap();
            assert!(s.paused, "{}", info.id);
            assert!(frame.to_text().contains("Paused"), "{}", info.id);
        }
    }

    #[test]
    fn the_sound_keys_change_effects_and_music_apart() {
        let mut sound = Sound::silent();
        let mut scores = Scores::in_memory();
        sound_key(Key::Char('+'), &mut sound, &mut scores);
        sound_key(Key::Char('['), &mut sound, &mut scores);
        assert_eq!(sound.status(), "FX 45% · Music 10%");
        assert_eq!(scores.get(crate::sound::FX_KEY), Some(45));
        assert_eq!(scores.get(crate::sound::MUSIC_KEY), Some(10));
        sound_key(Key::Char('0'), &mut sound, &mut scores);
        assert_eq!(sound.status(), "FX muted · Music off");
        assert!(sound_key(Key::Char('a'), &mut sound, &mut scores).is_none());
    }

    #[test]
    fn a_typing_game_types_the_space_and_the_sound_keys() {
        let mut scores = Scores::in_memory();
        let mut sound = Sound::silent();
        let game = games::make("touch-typing", setup()).unwrap();
        let mut s = Session::new(game, setup(), &scores);
        s.key(Key::Pause, &mut sound, &mut scores);
        assert!(!s.paused, "space is a character there");
        s.key(Key::Char('+'), &mut sound, &mut scores);
        assert_eq!(sound.volume, 35, "+ is a character there");
        s.key(Key::Esc, &mut sound, &mut scores);
        assert!(s.paused);
        s.key(Key::Char('+'), &mut sound, &mut scores);
        assert_eq!(sound.volume, 45, "the sound keys work while paused");
        assert_eq!(s.key(Key::Char('q'), &mut sound, &mut scores), Flow::Leave);
    }

    #[test]
    fn every_game_fits_every_size_without_spilling() {
        let sizes = [
            ((60, 20), ui::Tier::Compact),
            ((80, 24), ui::Tier::Compact),
            ((100, 30), ui::Tier::Standard),
            ((160, 45), ui::Tier::Large),
            ((207, 48), ui::Tier::Large),
        ];
        let mut problems = Vec::new();
        for ((w, h), want) in sizes {
            assert_eq!(ui::tier(w, h), want);
            for info in games::ALL {
                let game = games::make(info.id, setup()).unwrap();
                let mut keys = game.demo();
                keys.push((Key::None, 0.5));
                let (frame, s) = headless(
                    info.id,
                    setup(),
                    &keys,
                    (w, h),
                    theme::default_theme(),
                    None,
                )
                .unwrap();
                let text = frame.to_text();
                if (w, h) == (207, 48) {
                    let t = theme::default_theme();
                    let mut frames = Vec::new();
                    let mut long = keys.clone();
                    long.push((Key::None, 2.0));
                    headless(info.id, setup(), &long, (w, h), t, Some(&mut frames)).unwrap();
                    let measured: Vec<Vec<f32>> =
                        frames.iter().map(|f| flash::regions(f, t)).collect();
                    if flash::most_changes_per_second(&measured, FPS as usize)
                        > flash::MAX_CHANGES_PER_SECOND
                    {
                        problems.push(format!("{} flashes at {w}x{h}", info.id));
                    }
                    for colour in frame.colours() {
                        if t.contrast_of(colour) < t.floor() {
                            problems.push(format!("{} below the floor at {w}x{h}", info.id));
                        }
                    }
                }
                if s.outside > 0 {
                    problems.push(format!("{} at {w}x{h} spills {} cells", info.id, s.outside));
                }
                if !text.contains(info.name) {
                    problems.push(format!("{} at {w}x{h} has no title", info.id));
                }
                // the key line is always there, cut to fit
                if !text.contains(info.hints.split(" · ").next().unwrap()) {
                    problems.push(format!("{} at {w}x{h} has no key line", info.id));
                }
            }
        }
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }

    #[test]
    fn a_resize_mid_game_keeps_the_game() {
        let mut scores = Scores::in_memory();
        let mut sound = Sound::silent();
        let pal = Palette::new(theme::default_theme());
        let game = games::make("merge-gates", setup()).unwrap();
        let mut s = Session::new(game, setup(), &scores);
        s.key(Key::Char('d'), &mut sound, &mut scores);
        let before = s.game.score_key();
        let status = s.game.status();
        for (w, h) in [
            (207, 48),
            (60, 20),
            (100, 30),
            (160, 45),
            (59, 19),
            (80, 24),
        ] {
            let mut frame = Frame::new(w, h);
            s.draw(&mut frame, pal, &sound);
            let text = frame.to_text();
            if w < ui::MIN_WIDTH {
                assert!(text.contains("too small") && text.contains("It is 59 by 19"));
            } else {
                assert!(text.contains("Merge Gates"), "{w}x{h}");
                assert_eq!(s.outside, 0, "{w}x{h}");
            }
        }
        assert_eq!(s.game.score_key(), before);
        assert_eq!(s.game.status(), status, "nothing about the game changed");
    }

    #[test]
    fn a_small_terminal_gets_a_plain_message() {
        let (frame, _) = headless(
            "trace",
            setup(),
            &[],
            (40, 10),
            theme::default_theme(),
            None,
        )
        .unwrap();
        let text = frame.to_text();
        assert!(text.contains("too small") && text.contains("It needs 60 by 20"));
    }

    #[test]
    fn long_runs_of_random_keys_never_break_a_game() {
        use zacess_scenes::rng::Rng;
        let pool = [
            Key::Up,
            Key::Down,
            Key::Left,
            Key::Right,
            Key::Enter,
            Key::Tab,
            Key::Backspace,
            Key::Char('w'),
            Key::Char('a'),
            Key::Char('s'),
            Key::Char('d'),
            Key::Char('u'),
            Key::Char('e'),
            Key::Char('1'),
            Key::Char('3'),
            Key::Char(' '),
            Key::Char('('),
            Key::Char('='),
            Key::F(2),
            Key::F(5),
            Key::Ctrl('r'),
        ];
        for info in games::ALL {
            for seed in 0..3 {
                let mut rng = Rng::new(seed);
                let keys: Vec<(Key, f32)> = (0..1500)
                    .map(|_| (*rng.pick(&pool), if rng.chance(0.2) { 0.2 } else { 0.0 }))
                    .collect();
                let s = Setup { seed, ..setup() };
                headless(info.id, s, &keys, (80, 24), theme::default_theme(), None).unwrap();
            }
        }
    }

    #[test]
    fn every_game_and_its_demo_keep_the_flash_limit_in_every_theme() {
        for t in theme::all() {
            for info in games::ALL {
                let game = games::make(info.id, setup()).unwrap();
                let mut keys = game.demo();
                keys.push((Key::None, 3.0));
                let mut frames = Vec::new();
                headless(info.id, setup(), &keys, (80, 24), t, Some(&mut frames)).unwrap();
                let measured: Vec<Vec<f32>> = frames.iter().map(|f| flash::regions(f, t)).collect();
                let most = flash::most_changes_per_second(&measured, FPS as usize);
                assert!(
                    most <= flash::MAX_CHANGES_PER_SECOND,
                    "{} in {} changes {most} times a second",
                    info.id,
                    t.name
                );
            }
        }
    }

    #[test]
    fn every_colour_drawn_keeps_the_floor() {
        for t in theme::all() {
            for info in games::ALL {
                let game = games::make(info.id, setup()).unwrap();
                let keys = game.demo();
                let (frame, _) = headless(info.id, setup(), &keys, (80, 24), t, None).unwrap();
                for colour in frame.colours() {
                    assert!(
                        t.contrast_of(colour) >= t.floor(),
                        "{} in {}: {} at {:.2}:1",
                        info.id,
                        t.name,
                        colour.hex(),
                        t.contrast_of(colour)
                    );
                }
            }
        }
    }

    #[test]
    fn leaving_a_game_records_its_score() {
        let mut scores = Scores::in_memory();
        let mut sound = Sound::silent();
        let game = games::make("merge-gates", setup()).unwrap();
        let mut s = Session::new(game, setup(), &scores);
        assert_eq!(s.key(Key::Esc, &mut sound, &mut scores), Flow::Leave);
    }
}
