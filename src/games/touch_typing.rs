//! Touch Typing: a trainer in five levels of short lessons, from the home row to timed tests.
//! An on-screen keyboard shows every key with the finger that presses it, by bracket shape as
//! well as colour. It marks the next key. Mistakes are marked in place and never blink. Each
//! lesson needs 95% accuracy to unlock the next; progress and the best speed for each lesson
//! are saved.

use std::collections::BTreeMap;

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::keyboard::{self, Layout};
use crate::scores::Scores;
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, Tier, g};

pub const INFO: Info = Info {
    id: "touch-typing",
    name: "Touch Typing",
    icon: ('◫', 'k'),
    hue: Hue::Link,
    blurb: "learn to type by touch",
    hints: "Type the line · Backspace fix · ← → lesson · F2 layout",
    help: &[
        "# Goal",
        "Learn to type without looking, one group of keys at a time, from the home row to timed tests.",
        "# Rules",
        "- There are five levels: Beginner (the home row, finger by finger), Elementary (the top row, then the bottom row), Intermediate (all letters, capitals with Shift and common words), Advanced (numbers, punctuation, symbols, code and electronics text) and Expert (a timed minute and full sentences).",
        "- Type each line as shown. The next key is marked on the keyboard with » «, with the finger to use written under it.",
        "- Each key shows its finger by shape and colour: ( ) little finger, { } ring finger, [ ] middle finger, < > index finger. The space bar is for either thumb.",
        "- A wrong key is marked ^ under the letter you missed and counted. Backspace lets you fix it, but the miss still counts.",
        "- The keyboard follows your layout: UK Mac (Apple British), UK PC or US. It is read from the system and named above the line. F2 switches it and the choice is remembered. The finger hints, the Shift and Option hints and the symbols lesson all follow it.",
        "# Scoring",
        "Words a minute count five characters as a word. Accuracy is the share of keys typed right. The weakest keys are the three you miss most. The score is 100 for each lesson unlocked plus your best speed.",
        "# Winning and losing",
        "There is no losing. Finish a lesson with 95% accuracy or better to unlock the next one. Below that, Enter tries it again with new text.",
        "# Keys",
        "- Every character types, the space included. Shift types capitals and the upper symbols.",
        "- Backspace takes back the last key. ← and → choose any unlocked lesson. Enter moves on when a lesson is done. F2 switches the keyboard layout.",
        "Tip: keep your fingers resting on A S D F and J K L ; and feel for the bumps on F and J.",
    ],
    input: Input::Text,
    unit: "points",
};

/// The accuracy a lesson needs to unlock the next.
pub const TARGET: f32 = 0.95;
const LINE_LEN: usize = 34;
const LINES: usize = 3;
const TIMED_SECONDS: f32 = 60.0;

#[derive(Clone, Copy, Debug)]
pub struct Lesson {
    pub level: &'static str,
    pub name: &'static str,
    /// The keys drilled; an empty set means fixed text.
    pub keys: &'static str,
    pub kind: Kind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Short groups of the keys, for the first lessons.
    Drill,
    /// Words that use only the keys so far.
    Words,
    /// Words with a capital first letter.
    Capitals,
    Numbers,
    /// Lines from a fixed list.
    Text(&'static [&'static str]),
    /// Common words, untimed.
    Common,
    /// The symbols of the layout in use, in short groups.
    Symbols,
    /// Common words for a timed minute.
    Timed,
}

const CODE: &[&str] = &[
    "fn main() { println!(\"hi\"); }",
    "for (int i = 0; i < n; i++) {",
    "if x > 10 and y < 3: return x",
    "let pins: [u8; 4] = [2, 3, 5, 7];",
    "while (adc_read() < 512) { wait(); }",
    "def area(r): return 3.14 * r * r",
    "x = {\"volts\": 5, \"amps\": 0.2}",
    "printf(\"%d\\n\", count);",
];

const ELECTRONICS: &[&str] = &[
    "A 220 ohm resistor limits the LED.",
    "Ohm's law: V = I x R, so I = V / R.",
    "The 555 timer runs at 1.44/(R1+2R2)C.",
    "Decouple each IC with a 100 nF cap.",
    "Pull-up resistors hold SDA and SCL high.",
    "A diode drops about 0.7 V when on.",
    "PWM at 50% duty gives half the power.",
    "Ground the scope before you probe!",
];

const SENTENCES: &[&str] = &[
    "The quick brown fox jumps over the lazy dog.",
    "Pack my box with five dozen liquor jugs.",
    "Calm hands make fewer mistakes than fast ones.",
    "Every circuit needs a path back to ground.",
    "Good code reads like a clear set of steps.",
    "Practice a little every day and speed follows.",
    "A tidy bench keeps the solder off the floor.",
    "Small tests catch big bugs before they ship.",
];

pub const LESSONS: &[Lesson] = &[
    Lesson {
        level: "Beginner",
        name: "F and J, index fingers",
        keys: "fj",
        kind: Kind::Drill,
    },
    Lesson {
        level: "Beginner",
        name: "D and K, middle fingers",
        keys: "fjdk",
        kind: Kind::Drill,
    },
    Lesson {
        level: "Beginner",
        name: "S and L, ring fingers",
        keys: "fjdksl",
        kind: Kind::Drill,
    },
    Lesson {
        level: "Beginner",
        name: "A and ;, little fingers",
        keys: "fjdksla;",
        kind: Kind::Drill,
    },
    Lesson {
        level: "Beginner",
        name: "G and H, index reach",
        keys: "fjdksla;gh",
        kind: Kind::Drill,
    },
    Lesson {
        level: "Beginner",
        name: "Home row words",
        keys: "asdfghjkl",
        kind: Kind::Words,
    },
    Lesson {
        level: "Elementary",
        name: "E and I",
        keys: "asdfghjklei",
        kind: Kind::Words,
    },
    Lesson {
        level: "Elementary",
        name: "R and U",
        keys: "asdfghjkleiru",
        kind: Kind::Words,
    },
    Lesson {
        level: "Elementary",
        name: "T and Y",
        keys: "asdfghjkleiruty",
        kind: Kind::Words,
    },
    Lesson {
        level: "Elementary",
        name: "W and O",
        keys: "asdfghjkleirutywo",
        kind: Kind::Words,
    },
    Lesson {
        level: "Elementary",
        name: "Q and P, top row",
        keys: "asdfghjkleirutywoqp",
        kind: Kind::Words,
    },
    Lesson {
        level: "Elementary",
        name: "C and comma",
        keys: "asdfghjkleirutywoqpc,",
        kind: Kind::Words,
    },
    Lesson {
        level: "Elementary",
        name: "V and M",
        keys: "asdfghjkleirutywoqpc,vm",
        kind: Kind::Words,
    },
    Lesson {
        level: "Elementary",
        name: "B and N",
        keys: "asdfghjkleirutywoqpc,vmbn",
        kind: Kind::Words,
    },
    Lesson {
        level: "Elementary",
        name: "X, Z and full stop, bottom row",
        keys: "asdfghjkleirutywoqpc,vmbnxz.",
        kind: Kind::Words,
    },
    Lesson {
        level: "Intermediate",
        name: "All letters",
        keys: "abcdefghijklmnopqrstuvwxyz",
        kind: Kind::Words,
    },
    Lesson {
        level: "Intermediate",
        name: "Capitals with Shift",
        keys: "abcdefghijklmnopqrstuvwxyz",
        kind: Kind::Capitals,
    },
    Lesson {
        level: "Intermediate",
        name: "Common words",
        keys: "",
        kind: Kind::Common,
    },
    Lesson {
        level: "Advanced",
        name: "Numbers",
        keys: "0123456789",
        kind: Kind::Numbers,
    },
    Lesson {
        level: "Advanced",
        name: "Punctuation",
        keys: "",
        kind: Kind::Text(SENTENCES),
    },
    Lesson {
        level: "Advanced",
        name: "Symbols",
        keys: "",
        kind: Kind::Symbols,
    },
    Lesson {
        level: "Advanced",
        name: "Code",
        keys: "",
        kind: Kind::Text(CODE),
    },
    Lesson {
        level: "Advanced",
        name: "Electronics text",
        keys: "",
        kind: Kind::Text(ELECTRONICS),
    },
    Lesson {
        level: "Expert",
        name: "Timed minute",
        keys: "",
        kind: Kind::Timed,
    },
    Lesson {
        level: "Expert",
        name: "Full sentences",
        keys: "",
        kind: Kind::Text(SENTENCES),
    },
];

/// Words for the word lessons: common English plus electronics and programming.
const WORDS: &[&str] = &[
    "a", "add", "all", "ask", "dad", "fall", "flask", "glad", "half", "hall", "jag", "lad", "lag",
    "sad", "salad", "shall", "slag", "flag", "gas", "has", "had", "ash", "dash", "side", "life",
    "like", "hide", "idle", "field", "dish", "fish", "kid", "lid", "said", "rush", "sure", "rule",
    "rid", "fire", "hire", "ride", "rise", "user", "full", "dull", "kit", "tie", "tide", "tray",
    "yet", "tidy", "style", "key", "they", "true", "trait", "wire", "word", "work", "power",
    "slow", "two", "ow", "flow", "were", "water", "follow", "quiet", "query", "pair", "port",
    "pool", "tape", "type", "pipe", "output", "input", "code", "clock", "cache", "circuit",
    "voice", "click", "copy", "track", "cursor", "vim", "move", "mode", "value", "volume",
    "memory", "metal", "mute", "visual", "bit", "byte", "bus", "base", "number", "binary",
    "button", "noise", "node", "signal", "box", "zero", "size", "zone", "index", "fixed", "lazy",
    "buzz", "excel", "maze", "the", "and", "for", "with", "from", "this", "that", "your", "about",
    "every", "just",
];

pub const FINGER_NAMES: [&str; 9] = [
    "left little finger",
    "left ring finger",
    "left middle finger",
    "left index finger",
    "right index finger",
    "right middle finger",
    "right ring finger",
    "right little finger",
    "either thumb",
];

/// The practice lines of a lesson.
pub fn lines_for(lesson: &Lesson, layout: Layout, rng: &mut Rng) -> Vec<String> {
    let mut lines = Vec::new();
    for _ in 0..LINES {
        let mut line = String::new();
        while line.chars().count() < LINE_LEN {
            let piece = match lesson.kind {
                Kind::Drill => {
                    let keys: Vec<char> = lesson.keys.chars().collect();
                    let n = 2 + rng.below(3);
                    (0..n).map(|_| *rng.pick(&keys)).collect::<String>()
                }
                Kind::Words | Kind::Capitals => {
                    let fits: Vec<&&str> = WORDS
                        .iter()
                        .filter(|w| w.chars().all(|c| lesson.keys.contains(c)))
                        .collect();
                    let w = rng.pick(&fits).to_string();
                    if lesson.kind == Kind::Capitals {
                        let mut cs = w.chars();
                        cs.next()
                            .map(|f| f.to_ascii_uppercase().to_string() + cs.as_str())
                            .unwrap_or(w)
                    } else {
                        w
                    }
                }
                Kind::Numbers => {
                    let n = 1 + rng.below(9999);
                    if rng.chance(0.3) {
                        format!("{n}.{}", rng.below(10))
                    } else {
                        n.to_string()
                    }
                }
                Kind::Symbols => {
                    let symbols = layout.symbols();
                    let n = 2 + rng.below(3);
                    (0..n).map(|_| *rng.pick(&symbols)).collect::<String>()
                }
                Kind::Common | Kind::Timed => rng.pick(&WORDS[WORDS.len() - 60..]).to_string(),
                Kind::Text(list) => {
                    if line.is_empty() {
                        rng.pick(list).to_string()
                    } else {
                        break;
                    }
                }
            };
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(&piece);
            if matches!(lesson.kind, Kind::Text(_)) {
                break;
            }
        }
        lines.push(line);
    }
    lines
}

pub struct TouchTyping {
    /// The keyboard layout the lessons and the keyboard follow.
    pub layout: Layout,
    pub lesson: usize,
    pub unlocked: usize,
    pub lines: Vec<String>,
    pub line: usize,
    pub typed: Vec<char>,
    /// Places in the current line typed wrong at least once.
    wrong: Vec<bool>,
    pub hits: u32,
    pub misses: u32,
    started: Option<f32>,
    clock: f32,
    chars: u32,
    weak: BTreeMap<char, u32>,
    /// The result once a lesson is done: accuracy, speed and whether it passed.
    pub result: Option<(f32, f32, bool)>,
    best: BTreeMap<usize, u32>,
    rng: Rng,
}

impl TouchTyping {
    pub fn new(setup: Setup) -> Self {
        let mut t = Self {
            lesson: 0,
            unlocked: 0,
            lines: Vec::new(),
            line: 0,
            typed: Vec::new(),
            wrong: Vec::new(),
            hits: 0,
            misses: 0,
            started: None,
            clock: 0.0,
            chars: 0,
            weak: BTreeMap::new(),
            result: None,
            best: BTreeMap::new(),
            rng: Rng::new(setup.seed),
            layout: keyboard::chosen().unwrap_or_else(keyboard::fallback),
        };
        t.start(0);
        t
    }

    fn start(&mut self, lesson: usize) {
        self.lesson = lesson.min(LESSONS.len() - 1);
        self.lines = lines_for(&LESSONS[self.lesson], self.layout, &mut self.rng);
        self.line = 0;
        self.typed.clear();
        self.wrong = vec![false; self.current().chars().count()];
        self.hits = 0;
        self.misses = 0;
        self.started = None;
        self.chars = 0;
        self.weak.clear();
        self.result = None;
    }

    pub fn current(&self) -> &str {
        self.lines.get(self.line).map_or("", String::as_str)
    }

    fn timed(&self) -> bool {
        LESSONS[self.lesson].kind == Kind::Timed
    }

    pub fn accuracy(&self) -> f32 {
        let all = self.hits + self.misses;
        if all == 0 {
            1.0
        } else {
            self.hits as f32 / all as f32
        }
    }

    pub fn wpm(&self) -> f32 {
        let secs = self.started.map_or(0.0, |s| self.clock - s);
        if secs < 1.0 {
            0.0
        } else {
            self.chars as f32 / 5.0 / (secs / 60.0)
        }
    }

    /// The three keys missed most, with their counts.
    pub fn weakest(&self) -> Vec<(char, u32)> {
        let mut v: Vec<(char, u32)> = self.weak.iter().map(|(c, n)| (*c, *n)).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        v.truncate(3);
        v
    }

    fn finish(&mut self, fx: &mut Fx) {
        let acc = self.accuracy();
        let wpm = self.wpm();
        let passed = acc >= TARGET;
        self.result = Some((acc, wpm, passed));
        let best = self.best.entry(self.lesson).or_insert(0);
        *best = (*best).max(wpm.round() as u32);
        if passed {
            if self.unlocked == self.lesson && self.lesson + 1 < LESSONS.len() {
                self.unlocked += 1;
                fx.say("Next lesson unlocked");
            }
            fx.sound(Sfx::Win);
        } else {
            fx.say(format!("{:.0}% needs {:.0}%", acc * 100.0, TARGET * 100.0));
            fx.sound(Sfx::Low);
        }
    }

    fn type_char(&mut self, ch: char, fx: &mut Fx) {
        if self.result.is_some() {
            return;
        }
        if self.started.is_none() {
            self.started = Some(self.clock);
        }
        let line: Vec<char> = self.current().chars().collect();
        let pos = self.typed.len();
        let Some(&want) = line.get(pos) else { return };
        self.typed.push(ch);
        if ch == want {
            self.hits += 1;
            self.chars += 1;
            fx.sound(Sfx::Tick);
        } else {
            self.misses += 1;
            self.wrong[pos] = true;
            *self.weak.entry(want).or_insert(0) += 1;
            fx.sound(Sfx::Low);
        }
        if self.typed.len() == line.len() {
            self.chars += 1;
            self.line += 1;
            self.typed.clear();
            if self.line >= self.lines.len() {
                if self.timed() {
                    let more = lines_for(&LESSONS[self.lesson], self.layout, &mut self.rng);
                    self.lines.extend(more);
                } else {
                    self.finish(fx);
                    return;
                }
            }
            self.wrong = vec![false; self.current().chars().count()];
            fx.sound(Sfx::Tap);
        }
    }

    fn line_text(&self) -> Vec<char> {
        self.current().chars().collect()
    }
}

impl Game for TouchTyping {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn zoomable(&self) -> bool {
        false
    }

    fn animated(&self) -> bool {
        self.timed() && self.started.is_some() && self.result.is_none()
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        match key {
            Key::Char(c) => self.type_char(c, fx),
            Key::Backspace if self.result.is_none() => {
                if self.typed.pop().is_some() {
                    fx.sound(Sfx::Tick);
                }
            }
            Key::Left if self.lesson > 0 => {
                self.start(self.lesson - 1);
                fx.sound(Sfx::Tap);
            }
            Key::Right if self.lesson < self.unlocked => {
                self.start(self.lesson + 1);
                fx.sound(Sfx::Tap);
            }
            Key::Right => fx.say(format!(
                "Pass this lesson at {:.0}% to unlock the next",
                TARGET * 100.0
            )),
            Key::F(2) => {
                self.layout = self.layout.next();
                self.start(self.lesson);
                fx.say(format!("Keyboard: {}", self.layout.name()));
                fx.sound(Sfx::Tap);
            }
            Key::Enter => {
                if let Some((_, _, passed)) = self.result {
                    let next = if passed {
                        (self.lesson + 1).min(self.unlocked)
                    } else {
                        self.lesson
                    };
                    self.start(next);
                }
            }
            _ => {}
        }
    }

    fn tick(&mut self, dt: f32, fx: &mut Fx) {
        self.clock += dt;
        if self.timed()
            && self.result.is_none()
            && self
                .started
                .is_some_and(|s| self.clock - s >= TIMED_SECONDS)
        {
            self.finish(fx);
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        if c.tier == Tier::Large && area.w >= 120 && area.h >= 34 {
            self.draw_large(c, area);
            return;
        }
        let pal = c.pal;
        let w = area.w.min(74);
        let r = area.centre(w, area.h);
        let lesson = &LESSONS[self.lesson];
        let title = format!(
            "{} · {} of {}",
            lesson.level,
            self.lesson + 1,
            LESSONS.len()
        );
        let top = Rect::new(r.x, r.y, r.w, 7);
        c.titled(top, Line::Heavy, pal.link, &title, pal.bright);
        c.text(r.x + 2, r.y + 1, lesson.name, pal.link);
        let kb = format!("{} keyboard", self.layout.name());
        c.text(
            r.x + r.w - 2 - kb.chars().count() as i32,
            r.y + 1,
            &kb,
            pal.muted,
        );
        // the line to type, with what has been typed and any misses marked under it
        let line = self.line_text();
        let x0 = r.x + 2;
        let y = r.y + 3;
        for (i, ch) in line.iter().enumerate() {
            let x = x0 + i as i32;
            let (shown, colour) = if i < self.typed.len() {
                if self.typed[i] == *ch {
                    (*ch, pal.green)
                } else {
                    (*ch, pal.warning)
                }
            } else if i == self.typed.len() {
                (*ch, pal.bright)
            } else {
                (*ch, pal.muted)
            };
            let shown = if shown == ' ' && i <= self.typed.len() {
                '·'
            } else {
                shown
            };
            c.put(x, y, shown, colour);
            if i < self.wrong.len() && self.wrong[i] && i < self.typed.len() {
                c.put(x, y + 1, '^', pal.warning);
            }
            if i == self.typed.len() {
                c.put(x, y + 1, '‾', pal.bright);
            }
        }
        if let Some(next) = self.lines.get(self.line + 1) {
            c.text(x0, y + 2, next, pal.muted);
        }
        // the result once done; until then the next key and its finger
        let info_y = r.y + 7;
        match self.result {
            Some((acc, wpm, passed)) => {
                let text = if passed {
                    format!(
                        "{} Passed: {:.0}% at {:.0} wpm. Enter for the next lesson.",
                        g('✓', '+'),
                        acc * 100.0,
                        wpm
                    )
                } else {
                    format!(
                        "{} Not yet: {:.0}% (needs {:.0}%). Enter tries again.",
                        g('○', 'o'),
                        acc * 100.0,
                        TARGET * 100.0
                    )
                };
                c.text(
                    r.x + 2,
                    info_y,
                    &text,
                    if passed { pal.green } else { pal.warning },
                );
            }
            None => {
                if let Some(next) = line.get(self.typed.len()) {
                    let stroke = self.layout.find(*next);
                    let f = stroke.map_or(8, |k| k.finger) as usize;
                    let shift = match stroke {
                        Some(k) if k.option => " with Option",
                        Some(k) if k.shift => " with Shift",
                        _ => "",
                    };
                    let name = if *next == ' ' {
                        "space".to_string()
                    } else {
                        next.to_string()
                    };
                    c.text(
                        r.x + 2,
                        info_y,
                        &format!("Next: {name}{shift} · {}", FINGER_NAMES[f]),
                        finger_colour(c, f as u8),
                    );
                }
            }
        }
        // a short window keeps the one-line hint above and leaves the keyboard out
        if area.h >= 17 {
            draw_keyboard(
                c,
                self.layout,
                r.x + (r.w - 56) / 2,
                info_y + 1,
                line.get(self.typed.len())
                    .copied()
                    .filter(|_| self.result.is_none()),
            );
        }
        let weak: Vec<String> = self
            .weakest()
            .iter()
            .map(|(k, n)| {
                if *k == ' ' {
                    format!("space {n}")
                } else {
                    format!("{k} {n}")
                }
            })
            .collect();
        let foot = format!(
            "Weakest: {}",
            if weak.is_empty() {
                "none yet".to_string()
            } else {
                weak.join(", ")
            }
        );
        c.text(r.x + 2, r.y + r.h - 1, &foot, pal.text);
        if self.timed() {
            let left = self.started.map_or(TIMED_SECONDS, |s| {
                (TIMED_SECONDS - (self.clock - s)).max(0.0)
            });
            c.text(
                r.x + r.w - 14,
                r.y + r.h - 1,
                &format!("{left:.0} s left"),
                pal.link,
            );
        }
    }

    fn status(&self) -> Status {
        vec![
            ("WPM".into(), format!("{:.0}", self.wpm())),
            ("Acc".into(), format!("{:.0}%", self.accuracy() * 100.0)),
            (
                "Unlocked".into(),
                format!("{}/{}", self.unlocked + 1, LESSONS.len()),
            ),
        ]
    }

    fn score(&self) -> u64 {
        let best = self.best.values().copied().max().unwrap_or(0);
        self.unlocked as u64 * 100 + u64::from(best)
    }

    fn outcome(&self) -> Option<Outcome> {
        None
    }

    fn restore(&mut self, scores: &Scores) {
        self.unlocked = scores
            .get("typing-unlocked")
            .map_or(0, |v| (v as usize).min(LESSONS.len() - 1));
        for i in 0..LESSONS.len() {
            if let Some(w) = scores.get(&format!("typing-wpm-{i}")) {
                self.best.insert(i, w as u32);
            }
        }
        let layout = keyboard::chosen()
            .or_else(|| scores.get(keyboard::SETTING).and_then(Layout::from_index))
            .unwrap_or_else(keyboard::detected);
        if self.unlocked != self.lesson || layout != self.layout {
            self.layout = layout;
            self.start(self.unlocked);
        }
    }

    fn keep(&self, scores: &mut Scores) {
        scores.set(keyboard::SETTING, self.layout.index());
        if self.unlocked as u64 > scores.get("typing-unlocked").unwrap_or(0) {
            scores.set("typing-unlocked", self.unlocked as u64);
        }
        for (i, w) in &self.best {
            let key = format!("typing-wpm-{i}");
            if u64::from(*w) > scores.get(&key).unwrap_or(0) {
                scores.set(&key, u64::from(*w));
            }
        }
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        let line: Vec<char> = self.current().chars().collect();
        let mut keys = Vec::new();
        for (i, ch) in line.iter().take(12).enumerate() {
            // one deliberate slip, so a still shows how a miss is marked
            let typed = if i == 5 { 'x' } else { *ch };
            keys.push((Key::Char(typed), 0.15));
        }
        keys
    }
}

impl TouchTyping {
    /// The large layout: the lines to come, a keyboard of boxed keys with the finger inside
    /// each and a side panel of speed, accuracy, weak keys and progress.
    fn draw_large(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        let panel_w = 36;
        let left = Rect::new(area.x + 1, area.y, area.w - panel_w - 3, area.h);
        let lesson = &LESSONS[self.lesson];
        let title = format!(
            "{} · {} of {}",
            lesson.level,
            self.lesson + 1,
            LESSONS.len()
        );
        let top = Rect::new(left.x, left.y, left.w, 10);
        c.titled(top, Line::Heavy, pal.link, &title, pal.bright);
        c.text(top.x + 2, top.y + 1, lesson.name, pal.link);
        let kb = format!("{} keyboard", self.layout.name());
        c.text(
            top.x + top.w - 2 - kb.chars().count() as i32,
            top.y + 1,
            &kb,
            pal.muted,
        );
        let line = self.line_text();
        let (x0, y) = (top.x + 3, top.y + 3);
        for (i, ch) in line.iter().enumerate() {
            let x = x0 + i as i32 * 2;
            let colour = if i < self.typed.len() {
                if self.typed[i] == *ch {
                    pal.green
                } else {
                    pal.warning
                }
            } else if i == self.typed.len() {
                pal.bright
            } else {
                pal.muted
            };
            let shown = if *ch == ' ' && i <= self.typed.len() {
                '·'
            } else {
                *ch
            };
            c.put(x, y, shown, colour);
            if i < self.wrong.len() && self.wrong[i] && i < self.typed.len() {
                c.put(x, y + 1, '^', pal.warning);
            }
            if i == self.typed.len() {
                c.put(x, y + 1, '‾', pal.bright);
            }
        }
        for (k, next) in self.lines.iter().skip(self.line + 1).take(3).enumerate() {
            c.text(x0, y + 3 + k as i32, next, pal.muted);
        }
        let info_y = top.y + top.h + 1;
        let next = line
            .get(self.typed.len())
            .copied()
            .filter(|_| self.result.is_none());
        match (self.result, next) {
            (Some((acc, wpm, passed)), _) => {
                let text = if passed {
                    format!(
                        "{} Passed: {:.0}% at {:.0} wpm. Enter for the next lesson.",
                        g('✓', '+'),
                        acc * 100.0,
                        wpm
                    )
                } else {
                    format!(
                        "{} Not yet: {:.0}% (needs {:.0}%). Enter tries again.",
                        g('○', 'o'),
                        acc * 100.0,
                        TARGET * 100.0
                    )
                };
                c.text(
                    left.x + 2,
                    info_y,
                    &text,
                    if passed { pal.green } else { pal.warning },
                );
            }
            (None, Some(ch)) => {
                if let Some(k) = self.layout.find(ch) {
                    let modifier = if k.option {
                        " with Option"
                    } else if k.shift {
                        " with Shift"
                    } else {
                        ""
                    };
                    let name = if ch == ' ' {
                        "space".to_string()
                    } else {
                        ch.to_string()
                    };
                    c.text(
                        left.x + 2,
                        info_y,
                        &format!(
                            "Next: {name}{modifier} · {}",
                            FINGER_NAMES[k.finger as usize]
                        ),
                        finger_colour(c, k.finger),
                    );
                }
            }
            _ => {}
        }
        let target = next.and_then(|ch| self.layout.find(ch)).map(|k| k.key);
        let kx0 = left.x + (left.w - 70).max(0) / 2;
        let ky0 = info_y + 2;
        for (r, (indent, keys)) in self.layout.rows().iter().enumerate() {
            for (i, &(k, _)) in keys.iter().enumerate() {
                let f = self.layout.finger_at(r, i);
                let x = kx0 + indent * 2 + i as i32 * 5;
                let y = ky0 + r as i32 * 3;
                let chosen = target == Some(k);
                let colour = if chosen {
                    pal.bright
                } else {
                    finger_colour(c, f)
                };
                let line = if chosen { Line::Double } else { Line::Light };
                c.boxed(Rect::new(x, y, 5, 3), line, colour);
                let (open, close) = finger_brackets(f);
                c.put(x + 1, y + 1, open, colour);
                c.put(x + 2, y + 1, k.to_ascii_uppercase(), colour);
                c.put(x + 3, y + 1, close, colour);
            }
        }
        let space = target == Some(' ');
        let sy = ky0 + 12;
        let sb = Rect::new(kx0 + 17, sy, 31, 3);
        c.boxed(
            sb,
            if space { Line::Double } else { Line::Light },
            if space { pal.bright } else { pal.cyan },
        );
        c.text(
            sb.x + 12,
            sy + 1,
            "space",
            if space { pal.bright } else { pal.cyan },
        );
        c.text(
            kx0,
            sy + 4,
            "( ) little   { } ring   [ ] middle   < > index   space thumb",
            pal.muted,
        );

        let p = Rect::new(area.x + area.w - panel_w - 1, area.y, panel_w, area.h);
        c.titled(p, Line::Light, pal.muted, "Progress", pal.bright);
        let mut y = p.y + 2;
        let row = |c: &mut Canvas, y: i32, label: &str, value: String| {
            c.text(p.x + 2, y, label, pal.muted);
            c.text(p.x + 16, y, &value, pal.text);
        };
        row(c, y, "Speed", format!("{:.0} wpm", self.wpm()));
        y += 1;
        row(
            c,
            y,
            "Accuracy",
            format!(
                "{:.0}% (needs {:.0}%)",
                self.accuracy() * 100.0,
                TARGET * 100.0
            ),
        );
        y += 1;
        row(
            c,
            y,
            "Lines",
            format!(
                "{} of {}",
                self.line.min(self.lines.len()),
                self.lines.len()
            ),
        );
        y += 1;
        c.meter(
            p.x + 2,
            y,
            panel_w - 6,
            self.line as f32 / self.lines.len().max(1) as f32,
            pal.link,
            pal.muted,
        );
        y += 2;
        c.text(p.x + 2, y, "Weakest keys", pal.muted);
        y += 1;
        let weak = self.weakest();
        if weak.is_empty() {
            c.text(p.x + 4, y, "none yet", pal.text);
            y += 1;
        }
        for (k, n) in weak {
            let f = self.layout.find(k).map_or(8, |s| s.finger);
            let name = if k == ' ' {
                "space".to_string()
            } else {
                k.to_string()
            };
            c.text(
                p.x + 4,
                y,
                &format!(
                    "{name:<6} {n} missed · {}",
                    FINGER_NAMES[f as usize].split(' ').nth(1).unwrap_or("")
                ),
                finger_colour(c, f),
            );
            y += 1;
        }
        y += 1;
        c.text(p.x + 2, y, "Lessons unlocked", pal.muted);
        y += 1;
        c.meter(
            p.x + 2,
            y,
            panel_w - 10,
            (self.unlocked + 1) as f32 / LESSONS.len() as f32,
            pal.green,
            pal.muted,
        );
        c.text(
            p.x + panel_w - 7,
            y,
            &format!("{}/{}", self.unlocked + 1, LESSONS.len()),
            pal.text,
        );
        y += 2;
        // the levels with the lessons passed in each
        let mut levels: Vec<&str> = LESSONS.iter().map(|l| l.level).collect();
        levels.dedup();
        for level in levels {
            if y >= p.y + p.h - 2 {
                break;
            }
            let all: Vec<usize> = (0..LESSONS.len())
                .filter(|&i| LESSONS[i].level == level)
                .collect();
            let done = all.iter().filter(|&&i| i < self.unlocked).count();
            let mark = if done == all.len() {
                g('✓', '+')
            } else {
                ' '
            };
            c.text(
                p.x + 2,
                y,
                &format!("{mark} {level:<13} {done}/{}", all.len()),
                if done == all.len() {
                    pal.green
                } else {
                    pal.text
                },
            );
            y += 1;
        }
        if let Some(best) = self.best.get(&self.lesson) {
            c.text(
                p.x + 2,
                p.y + p.h - 2,
                &format!("Best here: {best} wpm"),
                pal.link,
            );
        }
    }
}

fn finger_colour(c: &Canvas, f: u8) -> zacess_scenes::colour::Rgb {
    let pal = c.pal;
    match f {
        0 | 7 => pal.magenta,
        1 | 6 => pal.blue,
        2 | 5 => pal.green,
        3 | 4 => pal.yellow,
        _ => pal.cyan,
    }
}

/// The brackets that show a finger by shape: little ( ), ring { }, middle [ ], index < >.
fn finger_brackets(f: u8) -> (char, char) {
    match f {
        0 | 7 => ('(', ')'),
        1 | 6 => ('{', '}'),
        2 | 5 => ('[', ']'),
        3 | 4 => ('<', '>'),
        _ => ('|', '|'),
    }
}

/// The keyboard: four rows of keys plus the space bar, each key in its finger's shape and colour,
/// with the next key marked » « in the brightest colour.
fn draw_keyboard(c: &mut Canvas, layout: Layout, x: i32, y: i32, next: Option<char>) {
    let pal = c.pal;
    let target = next.and_then(|ch| layout.find(ch)).map(|k| k.key);
    for (r, (indent, keys)) in layout.rows().iter().enumerate() {
        for (i, &(k, _)) in keys.iter().enumerate() {
            let kx = x + indent + i as i32 * 4;
            let f = layout.finger_at(r, i);
            let colour = finger_colour(c, f);
            let (open, close) = if target == Some(k) {
                ('»', '«')
            } else {
                finger_brackets(f)
            };
            let cap = if target == Some(k) {
                pal.bright
            } else {
                colour
            };
            c.put(kx, y + r as i32, open, cap);
            c.put(kx + 1, y + r as i32, k.to_ascii_uppercase(), cap);
            c.put(kx + 2, y + r as i32, close, cap);
        }
    }
    let space = target == Some(' ');
    let bar = if space {
        "»     space     «"
    } else {
        "|     space     |"
    };
    c.text(
        x + 14,
        y + 4,
        bar,
        if space { pal.bright } else { pal.cyan },
    );
    c.text(
        x,
        y + 5,
        "( ) little  { } ring  [ ] middle  < > index",
        pal.muted,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> TouchTyping {
        TouchTyping::new(Setup {
            seed: 4,
            reduced_motion: false,
            day: 0,
            practice: false,
        })
    }

    #[test]
    fn every_lesson_character_can_be_typed_on_every_layout() {
        let mut rng = Rng::new(3);
        for layout in Layout::ALL {
            for l in LESSONS {
                for line in lines_for(l, layout, &mut rng) {
                    for ch in line.chars() {
                        assert!(
                            layout.find(ch).is_some(),
                            "{} {}: {ch}",
                            layout.id(),
                            l.name
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn every_lesson_makes_lines_with_only_its_keys() {
        let mut rng = Rng::new(1);
        for l in LESSONS {
            let lines = lines_for(l, Layout::UkMac, &mut rng);
            assert_eq!(lines.len(), LINES, "{}", l.name);
            for line in &lines {
                assert!(
                    !line.is_empty() && line.chars().count() <= 48,
                    "{}: {line}",
                    l.name
                );
                if matches!(l.kind, Kind::Drill | Kind::Words) {
                    assert!(
                        line.chars().all(|c| c == ' ' || l.keys.contains(c)),
                        "{}: {line}",
                        l.name
                    );
                }
            }
        }
    }

    #[test]
    fn the_five_levels_come_in_order() {
        let mut levels: Vec<&str> = LESSONS.iter().map(|l| l.level).collect();
        levels.dedup();
        assert_eq!(
            levels,
            [
                "Beginner",
                "Elementary",
                "Intermediate",
                "Advanced",
                "Expert"
            ]
        );
    }

    #[test]
    fn a_clean_lesson_passes_and_unlocks_the_next() {
        let mut t = game();
        let mut fx = Fx::default();
        for _ in 0..LINES {
            let line: Vec<char> = t.current().chars().collect();
            for ch in line {
                t.clock += 0.2;
                t.key(Key::Char(ch), &mut fx);
            }
        }
        let (acc, wpm, passed) = t.result.unwrap();
        assert!(passed && acc == 1.0 && wpm > 0.0);
        assert_eq!(t.unlocked, 1);
        let mut scores = Scores::in_memory();
        t.keep(&mut scores);
        assert_eq!(scores.get("typing-unlocked"), Some(1));
        let mut again = game();
        again.restore(&scores);
        assert_eq!(again.lesson, 1);
    }

    #[test]
    fn misses_are_counted_by_key_and_can_fail_a_lesson() {
        let mut t = game();
        let mut fx = Fx::default();
        for _ in 0..LINES {
            let line: Vec<char> = t.current().chars().collect();
            for (i, ch) in line.into_iter().enumerate() {
                let typed = if i % 5 == 0 { '#' } else { ch };
                t.key(Key::Char(typed), &mut fx);
            }
        }
        let (_, _, passed) = t.result.unwrap();
        assert!(!passed);
        assert_eq!(t.unlocked, 0);
        assert!(!t.weakest().is_empty());
    }

    #[test]
    fn locked_lessons_cannot_be_chosen() {
        let mut t = game();
        t.key(Key::Right, &mut Fx::default());
        assert_eq!(t.lesson, 0);
    }
}
