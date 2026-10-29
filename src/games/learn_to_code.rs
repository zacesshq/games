//! Learn to Code: short levels in fifteen tracks, from first steps to job-ready. Each level has a
//! lesson of a line or two and a few lines of code with blanks to fill in. An answer is checked
//! inside the game against the answers the level lists, after spaces outside quotes are taken
//! out, so nothing typed is ever run as a program. A wrong answer gets a message that teaches:
//! one written for that mistake when the level has one, otherwise a check for the usual slips
//! (missing quotes, brackets that do not pair, = for ==, the wrong case). After three tries the
//! answer can be shown. Tracks live in Lua files, progress is saved for each language and new
//! tracks or levels can be added in `~/.config/zacess/games/learn-to-code`.

use mlua::{Table, Value};

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::levels;
use crate::scores::Scores;
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, Tier, g, page_lines};

pub const INFO: Info = Info {
    id: "learn-to-code",
    name: "Learn to Code",
    icon: ('λ', 'L'),
    hue: Hue::Property,
    blurb: "fifteen tracks, first steps on",
    hints: "Type in the blank · Tab next blank · Enter check · F2 hint",
    help: &[
        "# Goal",
        "Learn to program from first steps to job-ready, one idea at a time. There are tracks for Python, Java, C, C++, JavaScript, TypeScript, SQL, Bash and Linux, Git, Rust, Go, C# and Kotlin, a shared Data Structures and Algorithms track and an AI track in Python.",
        "# Rules",
        "- Pick a track with ↑ and ↓ and press Enter. The list is your progress overview: each track shows how many levels are done. Each remembers where you got to.",
        "- Every level has a short lesson and a few lines of code with blanks, shown as [ ]. Type into the blank that is marked ▸; Tab moves to the next blank.",
        "- Enter checks your answers. Spaces outside quotes never matter. When everything is right the program's output is shown and Enter goes on to the next level.",
        "- A wrong answer gets a message that explains the mistake. F2 shows a hint. After three tries F3 shows the answer.",
        "- Code is checked inside the game against the answers each level lists. Nothing you type is ever run on the computer.",
        "- F4 goes back to choose another language.",
        "# Scoring",
        "100 points for each level solved without the answer shown, less 20 for each wrong try, never below 20.",
        "# Winning and losing",
        "There is no losing. A track is complete when its last level is solved. The language list shows how many levels each language has done.",
        "# Keys",
        "- Every character types, the space included. Backspace deletes.",
        "- Tab moves to the next blank. Enter checks, then moves on.",
        "- F2 hint. F3 show the answer after three tries. F4 choose a language.",
        "Tip: read the expected output under the code first; it tells you what each blank must do.",
    ],
    input: Input::Text,
    unit: "points",
};

pub const BUILT_IN: &[(&str, &str)] = &[
    (
        "code/python.lua",
        include_str!("../../levels/code/python.lua"),
    ),
    ("code/java.lua", include_str!("../../levels/code/java.lua")),
    ("code/c.lua", include_str!("../../levels/code/c.lua")),
    ("code/cpp.lua", include_str!("../../levels/code/cpp.lua")),
    (
        "code/javascript.lua",
        include_str!("../../levels/code/javascript.lua"),
    ),
    (
        "code/typescript.lua",
        include_str!("../../levels/code/typescript.lua"),
    ),
    ("code/sql.lua", include_str!("../../levels/code/sql.lua")),
    ("code/bash.lua", include_str!("../../levels/code/bash.lua")),
    ("code/git.lua", include_str!("../../levels/code/git.lua")),
    ("code/dsa.lua", include_str!("../../levels/code/dsa.lua")),
    ("code/rust.lua", include_str!("../../levels/code/rust.lua")),
    ("code/go.lua", include_str!("../../levels/code/go.lua")),
    (
        "code/csharp.lua",
        include_str!("../../levels/code/csharp.lua"),
    ),
    (
        "code/kotlin.lua",
        include_str!("../../levels/code/kotlin.lua"),
    ),
    ("code/ai.lua", include_str!("../../levels/code/ai.lua")),
];

/// Tries before the answer can be shown.
const TRIES_BEFORE_ANSWER: u32 = 3;

#[derive(Clone, Debug, PartialEq)]
pub struct Blank {
    pub answers: Vec<String>,
    pub hint: String,
    /// Answers people often try, each with what to say about it.
    pub wrong: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodeLevel {
    pub name: String,
    pub topic: String,
    pub lesson: Vec<String>,
    pub code: Vec<String>,
    pub blanks: Vec<Blank>,
    pub output: Vec<String>,
}

/// How a track compares answers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rules {
    /// Single and double quotes mean the same, as in Python and JavaScript.
    pub quotes_equal: bool,
    /// Spaces between words matter, as in a shell command: runs of spaces become one instead
    /// of being taken out.
    pub keep_spaces: bool,
    /// Upper and lower case are the same, as in SQL keywords.
    pub ignore_case: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Track {
    pub language: String,
    pub id: String,
    /// How answers are compared in this track.
    pub rules: Rules,
    pub levels: Vec<CodeLevel>,
}

fn strings(t: &Table, key: &str) -> mlua::Result<Vec<String>> {
    Ok(t.get::<Option<Vec<String>>>(key)?.unwrap_or_default())
}

fn parse_track(source: &str, chunk: &str) -> Result<Track, String> {
    let lua = levels::sandbox().map_err(|e| e.to_string())?;
    let value: Value = lua
        .load(source)
        .set_name(chunk)
        .eval()
        .map_err(|e| e.to_string().lines().next().unwrap_or("error").to_string())?;
    let Value::Table(t) = value else {
        return Err(format!("{chunk} must return a table"));
    };
    let read = || -> mlua::Result<Track> {
        let mut levels = Vec::new();
        for lv in t.get::<Table>("levels")?.sequence_values::<Table>() {
            let lv = lv?;
            let mut blanks = Vec::new();
            for b in lv.get::<Table>("blanks")?.sequence_values::<Table>() {
                let b = b?;
                let mut wrong = Vec::new();
                if let Some(list) = b.get::<Option<Table>>("wrong")? {
                    for pair in list.sequence_values::<Vec<String>>() {
                        let pair = pair?;
                        if pair.len() == 2 {
                            wrong.push((pair[0].clone(), pair[1].clone()));
                        }
                    }
                }
                blanks.push(Blank {
                    answers: strings(&b, "answers")?,
                    hint: b.get::<Option<String>>("hint")?.unwrap_or_default(),
                    wrong,
                });
            }
            levels.push(CodeLevel {
                name: lv.get("name")?,
                topic: lv.get::<Option<String>>("topic")?.unwrap_or_default(),
                lesson: strings(&lv, "lesson")?,
                code: strings(&lv, "code")?,
                blanks,
                output: strings(&lv, "output")?,
            });
        }
        Ok(Track {
            language: t.get("language")?,
            id: t.get("id")?,
            rules: Rules {
                quotes_equal: t.get::<Option<bool>>("quotes_equal")?.unwrap_or(false),
                keep_spaces: t.get::<Option<bool>>("keep_spaces")?.unwrap_or(false),
                ignore_case: t.get::<Option<bool>>("ignore_case")?.unwrap_or(false),
            },
            levels,
        })
    };
    let track = read().map_err(|e| format!("{chunk}: {e}"))?;
    for (i, level) in track.levels.iter().enumerate() {
        let wanted = (1..=level.blanks.len())
            .filter(|n| level.code.iter().any(|l| l.contains(&format!("{{{n}}}"))))
            .count();
        if wanted != level.blanks.len() || level.blanks.iter().any(|b| b.answers.is_empty()) {
            return Err(format!(
                "{chunk}: level {} has a blank with no place or no answer",
                i + 1
            ));
        }
    }
    Ok(track)
}

/// The next blank marker `{N}` in a line of code, as its byte offset, its length less one and
/// its number. Braces that belong to the code itself, such as `{ get; set; }`, are skipped.
pub fn next_blank(line: &str, blanks: usize) -> Option<(usize, usize, usize)> {
    let mut from = 0;
    while let Some(rel) = line[from..].find('{') {
        let open = from + rel;
        if let Some(len) = line[open..].find('}')
            && let Ok(k) = line[open + 1..open + len].parse::<usize>()
            && (1..=blanks).contains(&k)
        {
            return Some((open, len, k));
        }
        from = open + 1;
    }
    None
}

/// The built-in tracks and any of the player's own, with problems from files that would not load.
pub fn load() -> (Vec<Track>, Vec<String>) {
    let mut tracks: Vec<Track> = Vec::new();
    let mut problems = Vec::new();
    let add = |t: Track, tracks: &mut Vec<Track>| {
        if let Some(have) = tracks.iter_mut().find(|h| h.id == t.id) {
            have.levels.extend(t.levels);
        } else {
            tracks.push(t);
        }
    };
    for (name, source) in BUILT_IN {
        match parse_track(source, name) {
            Ok(t) => add(t, &mut tracks),
            Err(e) => problems.push(e),
        }
    }
    if std::env::var_os("ZACESS_GAMES_NO_USER_LEVELS").is_none()
        && let Some(dir) = levels::user_dir("learn-to-code")
        && let Ok(entries) = std::fs::read_dir(dir)
    {
        let mut paths: Vec<_> = entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "lua"))
            .collect();
        paths.sort();
        for path in paths {
            let chunk = path.display().to_string();
            match std::fs::read_to_string(&path)
                .map_err(|e| e.to_string())
                .and_then(|s| parse_track(&s, &chunk))
            {
                Ok(t) => add(t, &mut tracks),
                Err(e) => problems.push(e),
            }
        }
    }
    (tracks, problems)
}

/// An answer with the spaces outside quotes taken out. Single quotes are made double where
/// they mean the same.
pub fn normalise(text: &str, rules: Rules) -> String {
    let mut out = String::new();
    let mut quote: Option<char> = None;
    let mut space = false;
    for ch in text.trim().chars() {
        let ch = if rules.quotes_equal && ch == '\'' {
            '"'
        } else {
            ch
        };
        match quote {
            Some(q) => {
                out.push(ch);
                if ch == q {
                    quote = None;
                }
            }
            None => {
                if ch.is_whitespace() {
                    space = true;
                    continue;
                }
                if space && rules.keep_spaces && !out.is_empty() {
                    out.push(' ');
                }
                space = false;
                if ch == '"' || ch == '\'' {
                    quote = Some(ch);
                    out.push(ch);
                } else if rules.ignore_case {
                    out.extend(ch.to_lowercase());
                } else {
                    out.push(ch);
                }
            }
        }
    }
    out
}

/// What is wrong with an answer in words that teach. `None` when it is right.
pub fn judge(blank: &Blank, typed: &str, rules: Rules) -> Option<String> {
    let mine = normalise(typed, rules);
    let answers: Vec<String> = blank.answers.iter().map(|a| normalise(a, rules)).collect();
    if answers.contains(&mine) {
        return None;
    }
    if mine.is_empty() {
        return Some("This blank is still empty: type an answer into it first.".into());
    }
    for (pattern, message) in &blank.wrong {
        if normalise(pattern, rules) == mine {
            return Some(message.clone());
        }
    }
    let first = &answers[0];
    if answers.iter().any(|a| a.eq_ignore_ascii_case(&mine)) {
        return Some("Nearly: names in code are case sensitive. Check the capitals.".into());
    }
    let pairs = [('(', ')'), ('[', ']'), ('{', '}')];
    for (open, close) in pairs {
        if mine.matches(open).count() != mine.matches(close).count() {
            return Some(format!(
                "The brackets do not pair up: every {open} needs a {close}."
            ));
        }
    }
    if mine.matches('"').count() % 2 == 1 || mine.matches('\'').count() % 2 == 1 {
        return Some("A quote is open: text starts and ends with the same quote mark.".into());
    }
    if first.starts_with('"') && !mine.contains('"') {
        return Some("Text needs quotes around it.".into());
    }
    if first.contains("==") && mine.contains('=') && !mine.contains("==") {
        return Some("One = stores a value. Two, ==, compares.".into());
    }
    Some("Not quite: that would not give the output shown. F2 shows a hint.".into())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Screen {
    Choose,
    Level,
    Solved,
}

pub struct LearnToCode {
    pub tracks: Vec<Track>,
    problems: Vec<String>,
    pub track: usize,
    screen: Screen,
    /// Levels done in each track, by track id.
    pub done: Vec<usize>,
    pub level: usize,
    pub typed: Vec<String>,
    pub blank: usize,
    tries: u32,
    shown: bool,
    feedback: Option<(bool, String)>,
    score: u64,
}

impl LearnToCode {
    pub fn new(_setup: Setup) -> Self {
        let (tracks, problems) = load();
        let done = vec![0; tracks.len()];
        Self {
            tracks,
            problems,
            track: 0,
            screen: Screen::Choose,
            done,
            level: 0,
            typed: Vec::new(),
            blank: 0,
            tries: 0,
            shown: false,
            feedback: None,
            score: 0,
        }
    }

    pub fn current(&self) -> Option<&CodeLevel> {
        self.tracks.get(self.track)?.levels.get(self.level)
    }

    fn open_level(&mut self, level: usize) {
        self.level = level;
        let blanks = self.current().map_or(0, |l| l.blanks.len());
        self.typed = vec![String::new(); blanks];
        self.blank = 0;
        self.tries = 0;
        self.shown = false;
        self.feedback = None;
        self.screen = Screen::Level;
    }

    fn begin_track(&mut self) {
        let len = self.tracks[self.track].levels.len();
        let at = self.done[self.track].min(len.saturating_sub(1));
        self.open_level(at);
    }

    pub fn check(&mut self, fx: &mut Fx) {
        let Some(level) = self.current().cloned() else {
            return;
        };
        let quotes = self.tracks[self.track].rules;
        for (i, b) in level.blanks.iter().enumerate() {
            if let Some(why) = judge(b, &self.typed[i], quotes) {
                self.tries += 1;
                self.blank = i;
                let more = if self.tries >= TRIES_BEFORE_ANSWER {
                    " F3 shows the answer."
                } else {
                    ""
                };
                self.feedback = Some((false, format!("Blank {}: {why}{more}", i + 1)));
                fx.sound(Sfx::Low);
                return;
            }
        }
        if !self.shown {
            self.score += 100u64.saturating_sub(20 * u64::from(self.tries)).max(20);
        }
        let t = self.track;
        self.done[t] = self.done[t].max(self.level + 1);
        self.feedback = Some((true, "Correct. The program prints the output below.".into()));
        self.screen = Screen::Solved;
        fx.sound(if self.done[t] == self.tracks[t].levels.len() {
            Sfx::Win
        } else {
            Sfx::Good
        });
    }

    fn total_done(&self) -> usize {
        self.done.iter().sum()
    }
}

impl Game for LearnToCode {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn zoomable(&self) -> bool {
        false
    }

    fn animated(&self) -> bool {
        false
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        if self.tracks.is_empty() {
            return;
        }
        match self.screen {
            Screen::Choose => match key {
                Key::Up => {
                    self.track = (self.track + self.tracks.len() - 1) % self.tracks.len();
                    fx.sound(Sfx::Tick);
                }
                Key::Down | Key::Tab => {
                    self.track = (self.track + 1) % self.tracks.len();
                    fx.sound(Sfx::Tick);
                }
                Key::Enter => {
                    self.begin_track();
                    fx.sound(Sfx::Tap);
                }
                _ => {}
            },
            Screen::Solved => match key {
                Key::Enter => {
                    let len = self.tracks[self.track].levels.len();
                    if self.level + 1 < len {
                        self.open_level(self.level + 1);
                    } else {
                        self.screen = Screen::Choose;
                    }
                    fx.sound(Sfx::Tap);
                }
                Key::F(4) => self.screen = Screen::Choose,
                _ => {}
            },
            Screen::Level => match key {
                Key::Char(c) => {
                    if let Some(t) = self.typed.get_mut(self.blank)
                        && t.chars().count() < 40
                    {
                        t.push(c);
                        fx.sound(Sfx::Tick);
                    }
                }
                Key::Backspace => {
                    if let Some(t) = self.typed.get_mut(self.blank)
                        && t.pop().is_some()
                    {
                        fx.sound(Sfx::Tick);
                    }
                }
                Key::Tab | Key::Down => {
                    if !self.typed.is_empty() {
                        self.blank = (self.blank + 1) % self.typed.len();
                    }
                }
                Key::Up => {
                    if !self.typed.is_empty() {
                        self.blank = (self.blank + self.typed.len() - 1) % self.typed.len();
                    }
                }
                Key::Enter => self.check(fx),
                Key::F(2) => {
                    if let Some(b) = self.current().and_then(|l| l.blanks.get(self.blank)) {
                        self.feedback = Some((false, format!("Hint: {}", b.hint)));
                        fx.sound(Sfx::Tap);
                    }
                }
                Key::F(3) => {
                    if self.tries >= TRIES_BEFORE_ANSWER {
                        if let Some(l) = self.current().cloned() {
                            self.typed = l.blanks.iter().map(|b| b.answers[0].clone()).collect();
                        }
                        self.shown = true;
                        self.check(fx);
                        self.feedback = Some((
                            true,
                            "Here is an answer. Read it through, then Enter goes on.".into(),
                        ));
                    } else {
                        self.feedback = Some((
                            false,
                            format!(
                                "The answer can be shown after {TRIES_BEFORE_ANSWER} tries. Try F2 for a hint."
                            ),
                        ));
                    }
                }
                Key::F(4) => self.screen = Screen::Choose,
                _ => {}
            },
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        // a large window gives the board its width; otherwise it keeps a readable size
        let w = area.w.min(if c.tier == Tier::Large { 130 } else { 76 });
        let r = area.centre(w, area.h);
        if self.tracks.is_empty() {
            c.text(
                r.x + 2,
                r.y + 1,
                "No Learn to Code tracks could be loaded.",
                pal.warning,
            );
            for (i, p) in self.problems.iter().enumerate() {
                c.text(r.x + 2, r.y + 2 + i as i32, p, pal.warning);
            }
            return;
        }
        if self.screen == Screen::Choose {
            // the progress overview: every track with its levels done, scrolled to the choice
            let rows = (area.h - 5).max(3);
            let n = self.tracks.len() as i32;
            let shown = n.min(rows);
            let first = (self.track as i32 - shown / 2).clamp(0, (n - shown).max(0));
            let b = Rect::new(r.x, r.y, r.w, shown + 4);
            let done: usize = self.done.iter().sum();
            let total: usize = self.tracks.iter().map(|t| t.levels.len()).sum();
            c.titled(
                b,
                Line::Heavy,
                pal.property,
                &format!("Your progress · {done} of {total} levels"),
                pal.bright,
            );
            let name_w = 32.min(r.w / 2);
            let bar_w = (r.w - name_w - 22).clamp(6, 30);
            for (row, i) in (first..first + shown).enumerate() {
                let t = &self.tracks[i as usize];
                let y = b.y + 2 + row as i32;
                let chosen = i as usize == self.track;
                c.put(
                    b.x + 2,
                    y,
                    if chosen { g('▶', '>') } else { ' ' },
                    pal.bright,
                );
                let name: String = t.language.chars().take(name_w as usize - 2).collect();
                c.text(
                    b.x + 4,
                    y,
                    &name,
                    if chosen { pal.bright } else { pal.property },
                );
                let d = self.done[i as usize];
                let len = t.levels.len();
                c.meter(
                    b.x + 4 + name_w,
                    y,
                    bar_w,
                    d as f32 / len.max(1) as f32,
                    pal.property,
                    pal.muted,
                );
                let text = if d >= len {
                    format!("{} all {len}", g('✓', '+'))
                } else {
                    format!("{d}/{len}")
                };
                c.text(b.x + 5 + name_w + bar_w, y, &text, pal.text);
            }
            if b.y + b.h < r.y + r.h {
                c.text(
                    r.x + 2,
                    b.y + b.h,
                    "↑ ↓ choose · Enter start or carry on",
                    pal.muted,
                );
            }
            return;
        }
        let Some(level) = self.current() else { return };
        let track = &self.tracks[self.track];
        let title = format!(
            "{} · {} of {} · {}",
            track.language,
            self.level + 1,
            track.levels.len(),
            level.topic
        );
        let lesson_lines = page_lines(&level.lesson, (r.w - 4) as usize);
        let lesson_h = lesson_lines.len().min(3) as i32;
        let code_h = level.code.len() as i32 + 2;
        let top = Rect::new(r.x, r.y, r.w, lesson_h + 2);
        c.titled(top, Line::Heavy, pal.property, &title, pal.bright);
        for (i, (_, l)) in lesson_lines.iter().take(3).enumerate() {
            c.text(r.x + 2, r.y + 1 + i as i32, l, pal.text);
        }
        let code = Rect::new(r.x, r.y + lesson_h + 2, r.w, code_h);
        c.titled(code, Line::Light, pal.muted, &level.name, pal.property);
        for (n, line) in level.code.iter().enumerate() {
            let y = code.y + 1 + n as i32;
            c.text(code.x + 1, y, &format!("{:>2}", n + 1), pal.muted);
            let mut x = code.x + 4;
            let mut rest = line.as_str();
            while !rest.is_empty() {
                if let Some((open, len, k)) = next_blank(rest, self.typed.len()) {
                    let before = &rest[..open];
                    c.text(x, y, before, pal.keyword);
                    x += before.chars().count() as i32;
                    let i = k - 1;
                    let active = i == self.blank && self.screen == Screen::Level;
                    let typed = &self.typed[i];
                    let shown = if typed.is_empty() {
                        "   ".to_string()
                    } else {
                        typed.clone()
                    };
                    if active {
                        c.put(x, y, g('▸', '>'), pal.bright);
                    } else {
                        c.put(x, y, '[', pal.property);
                    }
                    c.text(
                        x + 1,
                        y,
                        &shown,
                        if active { pal.bright } else { pal.property },
                    );
                    x += 1 + shown.chars().count() as i32;
                    c.put(
                        x,
                        y,
                        if active { '▏' } else { ']' },
                        if active { pal.bright } else { pal.property },
                    );
                    x += 1;
                    rest = &rest[open + len + 1..];
                } else {
                    c.text(x, y, rest, pal.keyword);
                    break;
                }
            }
        }
        let mut y = code.y + code_h;
        if let Some((good, text)) = &self.feedback {
            let mark = if *good { g('✓', '+') } else { g('✗', 'x') };
            let colour = if *good { pal.green } else { pal.warning };
            for (i, line) in crate::ui::wrap(&format!("{mark} {text}"), (r.w - 4) as usize)
                .iter()
                .take(2)
                .enumerate()
            {
                c.text(r.x + 2, y + i as i32, line, colour);
            }
            y += 2;
        }
        let label = if self.screen == Screen::Solved {
            "Output"
        } else {
            "Expected output"
        };
        c.text(r.x + 2, y, label, pal.muted);
        for (i, line) in level.output.iter().enumerate() {
            if y + 1 + (i as i32) < r.y + r.h {
                c.text(
                    r.x + 4,
                    y + 1 + i as i32,
                    line,
                    if self.screen == Screen::Solved {
                        pal.green
                    } else {
                        pal.text
                    },
                );
            }
        }
    }

    fn status(&self) -> Status {
        let lang = self
            .tracks
            .get(self.track)
            .map_or("-".to_string(), |t| t.language.clone());
        vec![
            ("Score".into(), self.score.to_string()),
            ("Language".into(), lang),
            ("Solved".into(), self.total_done().to_string()),
        ]
    }

    fn score(&self) -> u64 {
        self.score
    }

    fn outcome(&self) -> Option<Outcome> {
        None
    }

    fn restore(&mut self, scores: &Scores) {
        for (i, t) in self.tracks.iter().enumerate() {
            let saved = scores.get(&format!("code-{}", t.id)).unwrap_or(0) as usize;
            self.done[i] = saved.min(t.levels.len());
        }
        if let Some(last) = scores.get("code-language") {
            self.track = (last as usize).min(self.tracks.len().saturating_sub(1));
        }
    }

    fn keep(&self, scores: &mut Scores) {
        for (i, t) in self.tracks.iter().enumerate() {
            let key = format!("code-{}", t.id);
            if self.done[i] as u64 > scores.get(&key).unwrap_or(0) {
                scores.set(&key, self.done[i] as u64);
            }
        }
        scores.set("code-language", self.track as u64);
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        vec![
            (Key::Enter, 0.0),
            (Key::Char('H'), 0.0),
            (Key::Char('e'), 0.0),
            (Key::Char('l'), 0.0),
            (Key::Char('l'), 0.0),
            (Key::Char('o'), 0.0),
            (Key::Enter, 0.0),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> LearnToCode {
        LearnToCode::new(Setup {
            seed: 1,
            reduced_motion: false,
            day: 0,
            practice: false,
        })
    }

    #[test]
    fn every_track_loads_with_enough_levels_and_its_own_essentials() {
        let (tracks, problems) = load();
        assert!(problems.is_empty(), "{problems:?}");
        let ids: Vec<&str> = tracks.iter().map(|t| t.id.as_str()).collect();
        assert_eq!(
            ids,
            [
                "python",
                "java",
                "c",
                "cpp",
                "javascript",
                "typescript",
                "sql",
                "bash",
                "git",
                "dsa",
                "rust",
                "go",
                "csharp",
                "kotlin",
                "ai"
            ]
        );
        for t in &tracks {
            assert!(t.levels.len() >= 15, "{} has {}", t.id, t.levels.len());
            for l in &t.levels {
                assert!(
                    !l.lesson.is_empty() && !l.output.is_empty(),
                    "{} {}",
                    t.id,
                    l.name
                );
                for b in &l.blanks {
                    assert!(!b.hint.is_empty(), "{} {}", t.id, l.name);
                }
            }
        }
        let has = |id: &str, topic: &str| {
            tracks
                .iter()
                .find(|t| t.id == id)
                .unwrap()
                .levels
                .iter()
                .any(|l| l.topic.contains(topic))
        };
        for (id, topic) in [
            ("python", "Dictionaries"),
            ("java", "Interfaces"),
            ("c", "Pointers"),
            ("cpp", "RAII"),
            ("javascript", "Async"),
            ("typescript", "Generics"),
            ("sql", "Joins"),
            ("sql", "Transactions"),
            ("bash", "Pipes"),
            ("git", "Rebase"),
            ("dsa", "Dynamic programming"),
            ("rust", "Ownership"),
            ("go", "Channels"),
            ("csharp", "LINQ"),
            ("kotlin", "Null safety"),
            ("java", "Hashing"),
            ("java", "Sets"),
            ("java", "Maps"),
            ("java", "Stacks"),
            ("java", "Queues"),
            ("dsa", "Hash tables"),
            ("dsa", "Abstract data types"),
            ("ai", "Search"),
            ("ai", "Knowledge representation"),
            ("ai", "k-NN"),
            ("ai", "Robotics"),
        ] {
            assert!(has(id, topic), "{id} has no {topic} level");
        }
    }

    #[test]
    fn every_listed_answer_passes_and_every_listed_mistake_teaches() {
        let (tracks, _) = load();
        let mut problems = Vec::new();
        for t in &tracks {
            for l in &t.levels {
                for b in &l.blanks {
                    for a in &b.answers {
                        if let Some(why) = judge(b, a, t.rules) {
                            problems
                                .push(format!("{} {}: answer {a} is refused: {why}", t.id, l.name));
                        }
                    }
                    for (w, msg) in &b.wrong {
                        let got = judge(b, w, t.rules);
                        if got.as_deref() != Some(msg.as_str()) {
                            problems
                                .push(format!("{} {}: mistake {w} gives {got:?}", t.id, l.name));
                        }
                    }
                }
            }
        }
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }

    const PY: Rules = Rules {
        quotes_equal: true,
        keep_spaces: false,
        ignore_case: false,
    };

    #[test]
    fn shell_and_sql_tracks_keep_word_spaces_and_case_rules() {
        let shell = Rules {
            keep_spaces: true,
            ..Rules::default()
        };
        assert_eq!(normalise("  ls    -la  ", shell), "ls -la");
        assert_ne!(normalise("ls-la", shell), normalise("ls -la", shell));
        let sql = Rules {
            keep_spaces: true,
            ignore_case: true,
            ..Rules::default()
        };
        assert_eq!(normalise("GROUP  BY", sql), "group by");
        assert_eq!(
            normalise("'LED'", sql),
            "'LED'",
            "text in quotes keeps its case"
        );
    }

    #[test]
    fn blank_markers_are_found_among_the_code_braces() {
        assert_eq!(
            next_blank("public int Pin { {1}; set; }", 1),
            Some((17, 2, 1))
        );
        assert_eq!(next_blank("awk '{print ${1}}' f", 1), Some((13, 2, 1)));
        assert_eq!(next_blank("int pins[4] = {2, 3};", 1), None);
    }

    #[test]
    fn spaces_outside_quotes_never_matter() {
        assert_eq!(
            normalise(" count = count - 1 ", Rules::default()),
            "count=count-1"
        );
        assert_eq!(normalise("print( 'a b' )", PY), "print(\"a b\")");
        assert_eq!(normalise("x  >=  3", Rules::default()), "x>=3");
    }

    #[test]
    fn general_mistakes_get_teaching_messages() {
        let b = Blank {
            answers: vec!["\"volts\"".into()],
            hint: String::new(),
            wrong: Vec::new(),
        };
        assert!(judge(&b, "", PY).unwrap().contains("empty"));
        assert!(judge(&b, "volts", PY).unwrap().contains("quotes"));
        assert!(judge(&b, "\"volts", PY).unwrap().contains("quote is open"));
        let b = Blank {
            answers: vec!["toUpperCase".into()],
            hint: String::new(),
            wrong: Vec::new(),
        };
        assert!(
            judge(&b, "touppercase", Rules::default())
                .unwrap()
                .contains("case sensitive")
        );
        assert!(judge(&b, "f(", Rules::default()).unwrap().contains("pair"));
    }

    #[test]
    fn solving_a_level_saves_progress_for_its_language() {
        let mut g = game();
        let mut fx = Fx::default();
        g.key(Key::Enter, &mut fx);
        for ch in "\"Hello, world\"".chars() {
            g.key(Key::Char(ch), &mut fx);
        }
        g.key(Key::Enter, &mut fx);
        assert_eq!(g.done[0], 1);
        assert_eq!(g.score, 100);
        let mut scores = Scores::in_memory();
        g.keep(&mut scores);
        assert_eq!(scores.get("code-python"), Some(1));
        let mut again = game();
        again.restore(&scores);
        again.key(Key::Enter, &mut fx);
        assert_eq!(again.level, 1);
    }

    #[test]
    fn the_answer_shows_only_after_three_tries() {
        let mut g = game();
        let mut fx = Fx::default();
        g.key(Key::Enter, &mut fx);
        g.key(Key::F(3), &mut fx);
        assert_eq!(g.done[0], 0);
        for _ in 0..3 {
            g.key(Key::Char('x'), &mut fx);
            g.key(Key::Enter, &mut fx);
        }
        g.key(Key::F(3), &mut fx);
        assert_eq!(g.done[0], 1);
        assert_eq!(g.score, 0, "a shown answer scores nothing");
    }
}
