//! Heatsink: a typing game. Electronics terms and short code fragments drift along five heat
//! pipes towards a chip. Typing one in full clears it and cools the chip; one that reaches the
//! chip heats it. Letters match in either case, so the challenge is accuracy rather than Shift.
//! In calm mode (Tab) the chip never overheats and the words drift at half pace; the game ends
//! when the player chooses.

use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, Tier, g};

pub const INFO: Info = Info {
    id: "heatsink",
    name: "Heatsink",
    icon: ('▣', '#'),
    hue: Hue::Accent,
    blurb: "type terms to cool a chip",
    hints: "Type the words · Backspace undo · Tab calm mode · Enter end",
    help: &[
        "# Goal",
        "Keep the chip (U1) cool by typing the words and code fragments that drift towards it.",
        "# Rules",
        "- Words drift from the right along five heat pipes. Start typing any word: the one nearest the chip that starts with your letter is picked and marked ▸.",
        "- Type it to the end to clear it. Letters match in upper or lower case; symbols and spaces must match.",
        "- A wrong key is counted as a miss and nothing is typed. Backspace takes back the last letter of the picked word.",
        "- Each word that reaches the chip heats it. Each word cleared cools it a little.",
        "- Tab switches calm mode: half the pace and the chip never overheats.",
        "# Scoring",
        "10 points for each character of a cleared word, doubled while your accuracy is 95% or better. The status bar shows words a minute and accuracy.",
        "# Winning and losing",
        "The game ends when the chip reaches 100°C. In calm mode it ends only when you press Enter.",
        "# Keys",
        "- Any character types, the space included.",
        "- Backspace takes back a letter. Tab switches calm mode. Enter ends a calm round.",
        "Tip: finish the word nearest the chip before starting another.",
    ],
    input: Input::Text,
    unit: "points",
};

const PIPES: usize = 5;
/// Words and fragments to type. None needs a key the shared keys use while paused.
pub const WORDS: &[&str] = &[
    "resistor",
    "diode",
    "anode",
    "cathode",
    "voltage",
    "current",
    "ground",
    "relay",
    "mosfet",
    "inductor",
    "crystal",
    "clock",
    "latch",
    "buffer",
    "kernel",
    "socket",
    "thread",
    "mutex",
    "stack",
    "heap",
    "pointer",
    "array",
    "string",
    "struct",
    "enum",
    "trait",
    "lambda",
    "closure",
    "compile",
    "debug",
    "commit",
    "merge",
    "x += 1",
    "i++",
    "fn main()",
    "let mut n",
    "if x > 5",
    "while true",
    "return 0;",
    "#include",
    "int *p",
    "a[i] = 0",
    "self.next",
    "Vec<u8>",
    "Ok(())",
    "print(x)",
    "for i in xs",
    "x != y",
    "a && b",
    "a || b",
    "0xFF",
    "2.2k ohm",
    "5V rail",
    "PWM",
    "I2C bus",
    "UART",
    "GPIO 13",
    "SPI",
    "ADC",
    "f = 1/T",
    "V = IR",
    "P = VI",
];

#[derive(Clone, Debug, PartialEq)]
pub struct Drop {
    pub text: Vec<char>,
    pub pipe: usize,
    /// Columns from the chip; it heats the chip at 0.
    pub x: f32,
    pub typed: usize,
}

pub struct Heatsink {
    pub drops: Vec<Drop>,
    pub target: Option<usize>,
    pub heat: f32,
    pub calm: bool,
    rng: Rng,
    spawn_in: f32,
    elapsed: f32,
    pub hits: u32,
    pub misses: u32,
    pub cleared: u32,
    chars: u32,
    score: u64,
    ended: Option<String>,
    reduced: bool,
}

const AMBIENT: f32 = 25.0;
const LIMIT: f32 = 100.0;
const LANE: f32 = 48.0;

fn same(a: char, b: char) -> bool {
    a.eq_ignore_ascii_case(&b)
}

impl Heatsink {
    pub fn new(setup: Setup) -> Self {
        let mut h = Self {
            drops: Vec::new(),
            target: None,
            heat: AMBIENT,
            calm: false,
            rng: Rng::new(setup.seed),
            spawn_in: 0.5,
            elapsed: 0.0,
            hits: 0,
            misses: 0,
            cleared: 0,
            chars: 0,
            score: 0,
            ended: None,
            reduced: setup.reduced_motion,
        };
        h.spawn();
        h
    }

    fn pace(&self) -> f32 {
        let base = 2.2 + self.cleared as f32 * 0.04;
        let slow = if self.calm || self.reduced { 0.5 } else { 1.0 };
        base.min(4.5) * slow
    }

    fn spawn(&mut self) {
        let busy: Vec<usize> = self
            .drops
            .iter()
            .filter(|d| d.x > LANE - 18.0)
            .map(|d| d.pipe)
            .collect();
        let free: Vec<usize> = (0..PIPES).filter(|p| !busy.contains(p)).collect();
        if free.is_empty() {
            return;
        }
        let pipe = *self.rng.pick(&free);
        let text: Vec<char> = self.rng.pick(WORDS).chars().collect();
        self.drops.push(Drop {
            text,
            pipe,
            x: LANE,
            typed: 0,
        });
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
        if self.elapsed < 1.0 {
            0.0
        } else {
            self.chars as f32 / 5.0 / (self.elapsed / 60.0)
        }
    }

    fn type_char(&mut self, ch: char, fx: &mut Fx) {
        let target = match self.target {
            Some(t) => Some(t),
            None => self
                .drops
                .iter()
                .enumerate()
                .filter(|(_, d)| d.text.first().is_some_and(|c| same(*c, ch)))
                .min_by(|a, b| a.1.x.total_cmp(&b.1.x))
                .map(|(i, _)| i),
        };
        let Some(t) = target else {
            self.misses += 1;
            fx.sound(Sfx::Low);
            return;
        };
        let d = &mut self.drops[t];
        if d.text.get(d.typed).is_some_and(|c| same(*c, ch)) {
            d.typed += 1;
            self.hits += 1;
            self.target = Some(t);
            fx.sound(Sfx::Tick);
            if d.typed == d.text.len() {
                let len = d.text.len() as u64;
                self.drops.remove(t);
                self.target = None;
                self.cleared += 1;
                self.chars += len as u32;
                let bonus = if self.accuracy() >= 0.95 { 2 } else { 1 };
                self.score += 10 * len * bonus;
                self.heat = (self.heat - 3.0).max(AMBIENT);
                fx.sound(Sfx::Good);
            }
        } else {
            self.misses += 1;
            fx.sound(Sfx::Low);
        }
    }
}

impl Game for Heatsink {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn zoomable(&self) -> bool {
        false
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        match key {
            Key::Char(c) => self.type_char(c, fx),
            Key::Backspace => {
                if let Some(t) = self.target {
                    let d = &mut self.drops[t];
                    d.typed = d.typed.saturating_sub(1);
                    if d.typed == 0 {
                        self.target = None;
                    }
                    fx.sound(Sfx::Tick);
                }
            }
            Key::Tab => {
                self.calm = !self.calm;
                fx.say(if self.calm {
                    "Calm mode: no overheating"
                } else {
                    "Normal mode"
                });
                fx.sound(Sfx::Tap);
            }
            Key::Enter if self.calm => {
                self.ended = Some(format!("A calm round of {} words.", self.cleared));
                fx.sound(Sfx::Win);
            }
            _ => {}
        }
    }

    fn tick(&mut self, dt: f32, fx: &mut Fx) {
        self.elapsed += dt;
        let pace = self.pace();
        for d in &mut self.drops {
            d.x -= pace * dt;
        }
        let mut i = 0;
        while i < self.drops.len() {
            if self.drops[i].x <= 0.0 {
                let len = self.drops[i].text.len() as f32;
                self.drops.remove(i);
                match self.target {
                    Some(t) if t == i => self.target = None,
                    Some(t) if t > i => self.target = Some(t - 1),
                    _ => {}
                }
                if !self.calm {
                    self.heat += 10.0 + len;
                }
                fx.sound(Sfx::Low);
                fx.say(format!("The chip is at {:.0}°C", self.heat));
            } else {
                i += 1;
            }
        }
        if self.heat >= LIMIT && !self.calm {
            self.ended = Some(format!("The chip overheated after {} words.", self.cleared));
            return;
        }
        self.spawn_in -= dt;
        if self.spawn_in <= 0.0 {
            self.spawn();
            let gap = (3.0 - self.cleared as f32 * 0.03).max(1.4);
            self.spawn_in = if self.calm || self.reduced {
                gap * 2.0
            } else {
                gap
            };
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        // a large window gives the board its width; otherwise it keeps a readable size
        let w = area.w.min(if c.tier == Tier::Large { 150 } else { 76 });
        let h = PIPES as i32 * 2 + 3;
        let r = area.centre(w, h + 2);
        let frame = Rect::new(r.x, r.y, w, h);
        c.titled(frame, Line::Heavy, pal.accent, "Board", pal.bright);
        // the chip: a package with legs, its heat written beside it
        let chip_x = frame.x + 2;
        let top = frame.y + 2;
        let hot = (self.heat - AMBIENT) / (LIMIT - AMBIENT);
        let chip_colour = if hot > 0.7 {
            pal.warning
        } else if hot > 0.4 {
            pal.yellow
        } else {
            pal.green
        };
        for row in 0..(PIPES as i32 * 2 - 1) {
            c.put(chip_x, top + row, '┤', pal.muted);
            c.text(chip_x + 1, top + row, "███", chip_colour);
            c.put(chip_x + 4, top + row, '├', pal.muted);
        }
        c.text(chip_x + 1, top + PIPES as i32 - 1, "U1", pal.bright);
        let field_x = chip_x + 6;
        let field_w = frame.w - 9 - 10;
        for p in 0..PIPES {
            let y = top + p as i32 * 2;
            for x in (field_x..field_x + field_w).step_by(3) {
                c.put(x, y, '┄', pal.muted);
            }
        }
        for (i, d) in self.drops.iter().enumerate() {
            let y = top + d.pipe as i32 * 2;
            let x = field_x + (d.x / LANE * field_w as f32).round() as i32;
            let picked = self.target == Some(i);
            if picked {
                c.put(x - 1, y, g('▸', '>'), pal.bright);
            }
            for (k, ch) in d.text.iter().enumerate() {
                let colour = if k < d.typed {
                    pal.green
                } else if picked {
                    pal.bright
                } else {
                    pal.text
                };
                let shown = if k < d.typed && *ch == ' ' { '·' } else { *ch };
                c.put(x + k as i32, y, shown, colour);
            }
        }
        // the heat gauge on the right, filling upwards, with the degrees written under it
        let gx = frame.x + frame.w - 8;
        c.text(gx, frame.y + 1, "Heat", pal.muted);
        let rows = PIPES as i32 * 2 - 1;
        for row in 0..rows {
            let filled = (hot * rows as f32).round() as i32 >= rows - row;
            let (glyph, colour) = if filled {
                ('█', chip_colour)
            } else {
                ('·', pal.muted)
            };
            c.put(gx + 1, top + row, glyph, colour);
            c.put(gx + 2, top + row, glyph, colour);
        }
        c.text(gx, top + rows, &format!("{:.0}°C", self.heat), chip_colour);
        let mode = if self.calm {
            "Calm mode"
        } else {
            "Normal mode"
        };
        c.text(
            frame.x + 2,
            frame.y + frame.h,
            mode,
            if self.calm { pal.green } else { pal.muted },
        );
        c.text(
            frame.x + 16,
            frame.y + frame.h,
            &format!(
                "{} cleared · {:.0} wpm · {:.0}% accurate",
                self.cleared,
                self.wpm(),
                self.accuracy() * 100.0
            ),
            pal.text,
        );
    }

    fn status(&self) -> Status {
        vec![
            ("Score".into(), self.score.to_string()),
            ("Heat".into(), format!("{:.0}°C", self.heat)),
            ("WPM".into(), format!("{:.0}", self.wpm())),
            ("Acc".into(), format!("{:.0}%", self.accuracy() * 100.0)),
        ]
    }

    fn score(&self) -> u64 {
        self.score
    }

    fn score_key(&self) -> String {
        if self.calm {
            "heatsink-calm".into()
        } else {
            "heatsink".into()
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.ended.as_ref().map(|t| {
            if self.calm {
                Outcome::Won(t.clone())
            } else {
                Outcome::Lost(t.clone())
            }
        })
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        let mut keys = vec![(Key::None, 6.0)];
        if let Some(d) = self.drops.first() {
            for ch in d.text.iter().take(d.text.len().saturating_sub(2)) {
                keys.push((Key::Char(*ch), 0.1));
            }
        }
        keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> Heatsink {
        Heatsink::new(Setup {
            seed: 2,
            reduced_motion: false,
            day: 0,
            practice: false,
        })
    }

    #[test]
    fn typing_a_word_clears_it_and_scores() {
        let mut h = game();
        h.drops = vec![Drop {
            text: "Vec<u8>".chars().collect(),
            pipe: 0,
            x: 20.0,
            typed: 0,
        }];
        let mut fx = Fx::default();
        for ch in "vEC<u8>".chars() {
            h.key(Key::Char(ch), &mut fx);
        }
        assert!(h.drops.is_empty());
        assert_eq!(h.cleared, 1);
        assert_eq!(h.score, 140, "7 characters, doubled for full accuracy");
    }

    #[test]
    fn a_wrong_key_is_a_miss_and_types_nothing() {
        let mut h = game();
        h.drops = vec![Drop {
            text: "diode".chars().collect(),
            pipe: 1,
            x: 20.0,
            typed: 0,
        }];
        let mut fx = Fx::default();
        h.key(Key::Char('d'), &mut fx);
        h.key(Key::Char('x'), &mut fx);
        assert_eq!(h.drops[0].typed, 1);
        assert_eq!(h.misses, 1);
        assert!(h.accuracy() < 1.0);
    }

    #[test]
    fn a_word_reaching_the_chip_heats_it_and_too_much_heat_ends_it() {
        let mut h = game();
        let mut fx = Fx::default();
        for _ in 0..20 {
            h.drops = vec![Drop {
                text: "heap".chars().collect(),
                pipe: 0,
                x: 0.01,
                typed: 0,
            }];
            h.tick(0.1, &mut fx);
            if h.outcome().is_some() {
                break;
            }
        }
        assert!(matches!(h.outcome(), Some(Outcome::Lost(_))));
    }

    #[test]
    fn calm_mode_never_overheats_and_ends_on_enter() {
        let mut h = game();
        let mut fx = Fx::default();
        h.key(Key::Tab, &mut fx);
        for _ in 0..20 {
            h.drops = vec![Drop {
                text: "heap".chars().collect(),
                pipe: 0,
                x: 0.01,
                typed: 0,
            }];
            h.tick(0.1, &mut fx);
        }
        assert!(h.outcome().is_none());
        h.key(Key::Enter, &mut fx);
        assert!(matches!(h.outcome(), Some(Outcome::Won(_))));
    }

    #[test]
    fn every_word_fits_a_pipe() {
        for w in WORDS {
            assert!(!w.is_empty() && w.len() <= 12, "{w}");
        }
    }
}
