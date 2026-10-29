//! Termle: guess an electronics or programming term from a category clue and letter feedback.
//!
//! Each letter of a guess gets one of three marks: `=` when it sits in the right place, an arrow
//! when the term has it somewhere else (the arrow points the way to the nearest place it belongs)
//! and `·` when the term does not use it again. Once a round, Tab spends a guess to reveal one
//! letter. The daily term is the same for everyone on a given day and is played once a day, with a
//! streak kept for days in a row. Once it is done (and after Enter at the end) the game plays a
//! practice term.

use zacess_scenes::colour::Rgb;
use zacess_scenes::rng::Rng;

use crate::game::{Fx, Game, Info, Input, Outcome, Setup};
use crate::scores::Scores;
use crate::sound::Sfx;
use crate::term::Key;
use crate::ui::{Canvas, Hue, Line, Rect, Status, g};

pub const INFO: Info = Info {
    id: "termle",
    name: "Termle",
    icon: ('▤', '#'),
    hue: Hue::Yellow,
    blurb: "guess the day's tech term",
    hints: "Type a term · Enter guess · Backspace delete · Tab reveal",
    help: &[
        "# Goal",
        "Guess the day's electronics or programming term in six tries.",
        "# Rules",
        "- The category is the clue and the board shows how many letters the term has.",
        "- Type a guess of that length and press Enter.",
        "- Each letter is marked: = when it is in the right place, ◂ or ▸ when the term has it somewhere else (pointing the way to the nearest place it belongs) and · when the term does not use it again.",
        "- Once a round, Tab reveals one letter for the cost of one try.",
        "- The daily term is played once a day. Practice terms follow once it is done and after Enter at the end.",
        "# Scoring",
        "100 points for each try left when you solve it, plus 100. Solving the daily term on days in a row builds a streak.",
        "# Winning and losing",
        "Solve the term to win. Six tries without it shows the answer and resets the streak.",
        "# Keys",
        "- Letters type, in upper or lower case alike.",
        "- Backspace deletes. Enter guesses. Tab reveals a letter.",
        "Tip: an arrow tells you which way to move a letter, so two guesses can place most of them.",
    ],
    input: Input::Letters,
    unit: "points",
};

const TRIES: usize = 6;

/// The terms and their category clues. Every term is 4 to 8 letters, A to Z only.
pub const WORDS: &[(&str, &str)] = &[
    ("DIODE", "Component"),
    ("RELAY", "Component"),
    ("FUSE", "Component"),
    ("SWITCH", "Component"),
    ("MOSFET", "Component"),
    ("TRIODE", "Component"),
    ("VARISTOR", "Component"),
    ("CRYSTAL", "Component"),
    ("SENSOR", "Component"),
    ("BUZZER", "Component"),
    ("MOTOR", "Component"),
    ("ZENER", "Component"),
    ("ANTENNA", "Component"),
    ("RESISTOR", "Component"),
    ("INDUCTOR", "Component"),
    ("VOLT", "SI unit"),
    ("AMPERE", "SI unit"),
    ("WATT", "SI unit"),
    ("FARAD", "SI unit"),
    ("HENRY", "SI unit"),
    ("HERTZ", "SI unit"),
    ("JOULE", "SI unit"),
    ("COULOMB", "SI unit"),
    ("TESLA", "SI unit"),
    ("WEBER", "SI unit"),
    ("SIEMENS", "SI unit"),
    ("KELVIN", "SI unit"),
    ("RUST", "Programming language"),
    ("PYTHON", "Programming language"),
    ("KOTLIN", "Programming language"),
    ("SWIFT", "Programming language"),
    ("HASKELL", "Programming language"),
    ("ELIXIR", "Programming language"),
    ("RUBY", "Programming language"),
    ("SCALA", "Programming language"),
    ("JULIA", "Programming language"),
    ("FORTRAN", "Programming language"),
    ("PASCAL", "Programming language"),
    ("COBOL", "Programming language"),
    ("VERILOG", "Hardware language"),
    ("WHILE", "Keyword"),
    ("MATCH", "Keyword"),
    ("RETURN", "Keyword"),
    ("STRUCT", "Keyword"),
    ("CLASS", "Keyword"),
    ("BREAK", "Keyword"),
    ("YIELD", "Keyword"),
    ("ASYNC", "Keyword"),
    ("AWAIT", "Keyword"),
    ("CONST", "Keyword"),
    ("STATIC", "Keyword"),
    ("TRAIT", "Keyword"),
    ("LAMBDA", "Keyword"),
    ("ARRAY", "Data structure"),
    ("STACK", "Data structure"),
    ("QUEUE", "Data structure"),
    ("HEAP", "Data structure"),
    ("GRAPH", "Data structure"),
    ("TRIE", "Data structure"),
    ("TUPLE", "Data structure"),
    ("VECTOR", "Data structure"),
    ("BITSET", "Data structure"),
    ("BUFFER", "Data structure"),
    ("CLOCK", "Digital logic"),
    ("LATCH", "Digital logic"),
    ("ADDER", "Digital logic"),
    ("COUNTER", "Digital logic"),
    ("REGISTER", "Digital logic"),
    ("DECODER", "Digital logic"),
    ("ENCODER", "Digital logic"),
    ("GATE", "Digital logic"),
    ("PULSE", "Signals"),
    ("PHASE", "Signals"),
    ("NOISE", "Signals"),
    ("CARRIER", "Signals"),
    ("FILTER", "Signals"),
    ("SIGNAL", "Signals"),
    ("GROUND", "Circuits"),
    ("CIRCUIT", "Circuits"),
    ("NODE", "Circuits"),
    ("MESH", "Circuits"),
    ("SHORT", "Circuits"),
    ("COMPILER", "Tool"),
    ("LINKER", "Tool"),
    ("DEBUGGER", "Tool"),
    ("PROFILER", "Tool"),
    ("SERIAL", "Interface"),
    ("UART", "Interface"),
    ("SOCKET", "Interface"),
    ("ETHERNET", "Interface"),
    ("POINTER", "Programming idea"),
    ("CLOSURE", "Programming idea"),
    ("BOOLEAN", "Programming idea"),
    ("INTEGER", "Programming idea"),
    ("BRANCH", "Programming idea"),
    ("COMMIT", "Version control"),
    ("MERGE", "Version control"),
    ("REBASE", "Version control"),
];

/// One letter's mark.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mark {
    Here,
    Left,
    Right,
    Absent,
}

/// Marks a guess against the answer. Right-place letters are matched first, so a letter that
/// appears once in the answer gives only one arrow however often it is guessed. An arrow points
/// to the nearest unmatched place that letter has in the answer.
pub fn mark(answer: &[char], guess: &[char]) -> Vec<Mark> {
    let n = answer.len();
    let mut marks = vec![Mark::Absent; n];
    let mut taken = vec![false; n];
    for i in 0..n {
        if guess[i] == answer[i] {
            marks[i] = Mark::Here;
            taken[i] = true;
        }
    }
    for i in 0..n {
        if marks[i] == Mark::Here {
            continue;
        }
        let best = (0..n)
            .filter(|&j| !taken[j] && answer[j] == guess[i])
            .min_by_key(|&j| (j as i32 - i as i32).abs());
        if let Some(j) = best {
            taken[j] = true;
            marks[i] = if j < i { Mark::Left } else { Mark::Right };
        }
    }
    marks
}

pub struct Termle {
    answer: Vec<char>,
    category: &'static str,
    daily: Option<u64>,
    guesses: Vec<(Vec<char>, Vec<Mark>)>,
    typing: Vec<char>,
    revealed: Option<usize>,
    done: Option<Outcome>,
    seed: u64,
    /// Days in a row the daily term has been solved and the longest run.
    streak: u64,
    best_streak: u64,
    /// Set when today's term was already played, so this round is practice.
    daily_done: bool,
}

/// The record keys: the last day the daily term was played and the streaks.
pub const DAY_KEY: &str = "termle-day";
pub const STREAK_KEY: &str = "termle-streak";
pub const BEST_STREAK_KEY: &str = "termle-best-streak";

/// The streak after finishing the daily term of `day`: one more after solving yesterday's, a
/// fresh start of one after a gap and nothing after a miss.
pub fn next_streak(last_day: Option<u64>, streak: u64, day: u64, solved: bool) -> u64 {
    if !solved {
        0
    } else if last_day.is_some_and(|d| d + 1 == day) {
        streak + 1
    } else {
        1
    }
}

/// The day's term: a fixed shuffle of the list, so no term repeats until every one has been used.
pub fn daily_index(day: u64) -> usize {
    let n = WORDS.len();
    let mut order: Vec<usize> = (0..n).collect();
    let mut rng = Rng::new(0x7e57_1e00 + day / n as u64);
    for i in (1..n).rev() {
        order.swap(i, rng.below(i + 1));
    }
    order[(day % n as u64) as usize]
}

impl Termle {
    pub fn new(setup: Setup) -> Self {
        let (index, daily) = if setup.practice {
            (Rng::new(setup.seed).below(WORDS.len()), None)
        } else {
            (daily_index(setup.day), Some(setup.day))
        };
        let (word, category) = WORDS[index];
        Self {
            answer: word.chars().collect(),
            category,
            daily,
            guesses: Vec::new(),
            typing: Vec::new(),
            revealed: None,
            done: None,
            seed: setup.seed,
            streak: 0,
            best_streak: 0,
            daily_done: false,
        }
    }

    fn practice_word(&mut self) {
        let (word, category) = WORDS[Rng::new(self.seed).below(WORDS.len())];
        self.answer = word.chars().collect();
        self.category = category;
        self.daily = None;
    }

    fn tries_left(&self) -> usize {
        TRIES - self.guesses.len() - usize::from(self.revealed.is_some())
    }

    fn submit(&mut self, fx: &mut Fx) {
        if self.typing.len() != self.answer.len() {
            fx.say(format!("The term has {} letters", self.answer.len()));
            fx.sound(Sfx::Low);
            return;
        }
        let guess = std::mem::take(&mut self.typing);
        let marks = mark(&self.answer, &guess);
        let solved = marks.iter().all(|m| *m == Mark::Here);
        let here = marks.iter().filter(|m| **m == Mark::Here).count();
        self.guesses.push((guess, marks));
        if solved {
            self.done = Some(Outcome::Won(format!(
                "{} solved in {}.",
                self.answer.iter().collect::<String>(),
                self.guesses.len()
            )));
            fx.sound(Sfx::Win);
            fx.say("Solved");
        } else if self.tries_left() == 0 {
            self.done = Some(Outcome::Lost(format!(
                "The term was {}.",
                self.answer.iter().collect::<String>()
            )));
            fx.sound(Sfx::Low);
        } else {
            fx.sound(Sfx::Tap);
            fx.say(format!("{here} in place"));
        }
    }

    fn reveal(&mut self, fx: &mut Fx) {
        if self.revealed.is_some() {
            fx.say("The reveal is already used");
            return;
        }
        if self.tries_left() <= 1 {
            fx.say("No guess left to spend");
            return;
        }
        // the first place no guess has yet put right
        let known: Vec<bool> = (0..self.answer.len())
            .map(|i| self.guesses.iter().any(|(_, m)| m[i] == Mark::Here))
            .collect();
        let mut rng = Rng::new(self.seed ^ 0x5eed);
        let open: Vec<usize> = (0..self.answer.len()).filter(|&i| !known[i]).collect();
        if open.is_empty() {
            return;
        }
        let i = *rng.pick(&open);
        self.revealed = Some(i);
        fx.say(format!("Letter {} is {}", i + 1, self.answer[i]));
        fx.sound(Sfx::Good);
    }

    /// What is known about each letter, for the keyboard strip.
    fn letter_state(&self, ch: char) -> Option<Mark> {
        let mut best: Option<Mark> = None;
        for (guess, marks) in &self.guesses {
            for (g, m) in guess.iter().zip(marks) {
                if *g != ch {
                    continue;
                }
                best = Some(match (best, m) {
                    (_, Mark::Here) | (Some(Mark::Here), _) => Mark::Here,
                    (_, Mark::Left | Mark::Right) | (Some(Mark::Left | Mark::Right), _) => {
                        Mark::Right
                    }
                    _ => Mark::Absent,
                });
            }
        }
        best
    }
}

/// A day count since 1970 as a date such as `29 Oct 2026`, by the civil calendar algorithm.
pub fn date(days: u64) -> String {
    let z = days as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    format!("{d} {} {y}", MONTHS[(m - 1) as usize])
}

fn mark_glyph(m: Mark) -> char {
    match m {
        Mark::Here => '=',
        Mark::Left => g('◂', '<'),
        Mark::Right => g('▸', '>'),
        Mark::Absent => '·',
    }
}

impl Game for Termle {
    fn info(&self) -> &'static Info {
        &INFO
    }

    fn animated(&self) -> bool {
        false
    }

    fn key(&mut self, key: Key, fx: &mut Fx) {
        if self.done.is_some() {
            return;
        }
        match key {
            Key::Char(c) if c.is_ascii_alphabetic() => {
                if self.typing.len() < self.answer.len() {
                    self.typing.push(c.to_ascii_uppercase());
                    fx.sound(Sfx::Tick);
                }
            }
            Key::Backspace => {
                if self.typing.pop().is_some() {
                    fx.sound(Sfx::Tick);
                }
            }
            Key::Enter => self.submit(fx),
            Key::Tab => self.reveal(fx),
            _ => {}
        }
    }

    fn draw(&self, c: &mut Canvas, area: Rect) {
        let pal = c.pal;
        let n = self.answer.len() as i32;
        let tile = 5;
        let board_w = (n * tile + 4).max(36);
        let board_h = TRIES as i32 * 2 + 3;
        // the side panel is left out when the window is too narrow for it; the status bar
        // carries what matters
        let panel_w = if area.w >= board_w + 2 + 26 { 26 } else { 0 };
        let total = board_w + 2 + panel_w;
        let r = area.centre(total, board_h.max(14));
        let board = Rect::new(r.x, r.y, board_w, board_h);
        let title = match self.daily {
            None if self.daily_done => "Practice · today's term is done".into(),
            Some(day) => format!("Daily {}", date(day)),
            None => "Practice".into(),
        };
        c.titled(board, Line::Heavy, pal.yellow, &title, pal.bright);
        let x0 = board.x + (board_w - n * tile) / 2;
        for row in 0..TRIES {
            let y = board.y + 2 + row as i32 * 2;
            for i in 0..n {
                let x = x0 + i * tile;
                let (ch, m): (char, Option<Mark>) =
                    if let Some((guess, marks)) = self.guesses.get(row) {
                        (guess[i as usize], Some(marks[i as usize]))
                    } else if row == self.guesses.len() && self.done.is_none() {
                        (self.typing.get(i as usize).copied().unwrap_or(' '), None)
                    } else {
                        (' ', None)
                    };
                let colour: Rgb = match m {
                    Some(Mark::Here) => pal.green,
                    Some(Mark::Left | Mark::Right) => pal.yellow,
                    Some(Mark::Absent) => pal.muted,
                    None => pal.text,
                };
                let current = row == self.guesses.len() && self.done.is_none();
                let (open, close) = match m {
                    Some(Mark::Here) => ('[', ']'),
                    Some(_) => (' ', ' '),
                    None if current => ('‹', '›'),
                    None => (' ', ' '),
                };
                if ch == ' ' && m.is_none() {
                    c.put(x + 1, y, '_', if current { pal.text } else { pal.muted });
                } else {
                    c.put(x, y, open, colour);
                    c.put(x + 1, y, ch, colour);
                    c.put(x + 2, y, close, colour);
                    if let Some(m) = m {
                        c.put(x + 3, y, mark_glyph(m), colour);
                    }
                }
            }
        }
        // the panel: the clue, the reveal and the letters tried
        let p = Rect::new(board.x + board_w + 2, board.y, panel_w, board_h.max(14));
        if panel_w == 0 {
            return;
        }
        c.titled(p, Line::Light, pal.muted, "Clue", pal.bright);
        c.text(p.x + 2, p.y + 2, self.category, pal.yellow);
        c.text(
            p.x + 2,
            p.y + 3,
            &format!("{} letters", self.answer.len()),
            pal.text,
        );
        c.text(
            p.x + 2,
            p.y + 4,
            &format!("{} tries left", self.tries_left()),
            pal.text,
        );
        match self.revealed {
            Some(i) => c.text(
                p.x + 2,
                p.y + 5,
                &format!("Letter {} is {}", i + 1, self.answer[i]),
                pal.green,
            ),
            None => c.text(p.x + 2, p.y + 5, "Tab: reveal a letter", pal.muted),
        }
        c.text(
            p.x + 2,
            p.y + 6,
            &format!("Streak {} · best {}", self.streak, self.best_streak),
            pal.yellow,
        );
        let letters: Vec<char> = ('A'..='Z').collect();
        for (k, chunk) in letters.chunks(7).enumerate() {
            let y = p.y + 7 + k as i32 * 2;
            for (j, &ch) in chunk.iter().enumerate() {
                let x = p.x + 2 + j as i32 * 3;
                let (colour, after) = match self.letter_state(ch) {
                    Some(Mark::Here) => (pal.green, '='),
                    Some(Mark::Absent) => (pal.muted, '·'),
                    Some(_) => (pal.yellow, '~'),
                    None => (pal.text, ' '),
                };
                c.put(x, y, ch, colour);
                c.put(x + 1, y, after, colour);
            }
        }
    }

    fn status(&self) -> Status {
        vec![
            ("Clue".into(), self.category.into()),
            ("Tries".into(), format!("{}/{TRIES}", self.guesses.len())),
            ("Streak".into(), self.streak.to_string()),
            ("Score".into(), self.score().to_string()),
        ]
    }

    fn score(&self) -> u64 {
        match self.done {
            Some(Outcome::Won(_)) => (self.tries_left() as u64 + 1) * 100,
            _ => 0,
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.done.clone()
    }

    fn restore(&mut self, scores: &Scores) {
        let last = scores.get(DAY_KEY);
        self.best_streak = scores.get(BEST_STREAK_KEY).unwrap_or(0);
        // a streak only stands while yesterday or today was played
        self.streak = match (last, self.daily) {
            (Some(d), Some(today)) if d + 1 >= today => scores.get(STREAK_KEY).unwrap_or(0),
            (Some(_), None) => scores.get(STREAK_KEY).unwrap_or(0),
            _ => 0,
        };
        if let Some(today) = self.daily
            && last == Some(today)
        {
            self.daily_done = true;
            self.practice_word();
        }
    }

    fn keep(&self, scores: &mut Scores) {
        let Some(day) = self.daily else { return };
        let Some(outcome) = &self.done else { return };
        let solved = matches!(outcome, Outcome::Won(_));
        let streak = next_streak(
            scores.get(DAY_KEY),
            scores.get(STREAK_KEY).unwrap_or(0),
            day,
            solved,
        );
        scores.set(DAY_KEY, day);
        scores.set(STREAK_KEY, streak);
        if streak > scores.get(BEST_STREAK_KEY).unwrap_or(0) {
            scores.set(BEST_STREAK_KEY, streak);
        }
    }

    fn demo(&self) -> Vec<(Key, f32)> {
        let mut keys = Vec::new();
        let n = self.answer.len();
        // two plausible guesses of the right length built from common letters
        for word in ["STARLIGHT", "CODEWORK"] {
            for ch in word.chars().take(n) {
                keys.push((Key::Char(ch), 0.0));
            }
            keys.push((Key::Enter, 0.0));
        }
        for ch in self.answer.iter().take(n.saturating_sub(2)) {
            keys.push((Key::Char(*ch), 0.0));
        }
        keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn every_term_is_four_to_eight_letters_and_unique() {
        let mut seen = std::collections::HashSet::new();
        for (word, category) in WORDS {
            assert!((4..=8).contains(&word.len()), "{word}");
            assert!(word.chars().all(|c| c.is_ascii_uppercase()), "{word}");
            assert!(!category.is_empty());
            assert!(seen.insert(*word), "{word} twice");
        }
    }

    #[test]
    fn marks_place_letters_and_point_the_way() {
        let m = mark(&chars("DIODE"), &chars("ODDIE"));
        // O belongs at 2, right of where it was guessed; the nearer D is at 0, to the left
        assert_eq!(m[0], Mark::Right);
        assert_eq!(m[1], Mark::Left);
        assert_eq!(m[4], Mark::Here);
        assert_eq!(mark(&chars("RUST"), &chars("RUST")), vec![Mark::Here; 4]);
        assert_eq!(mark(&chars("HEAP"), &chars("XXXX")), vec![Mark::Absent; 4]);
    }

    #[test]
    fn a_letter_once_in_the_answer_gets_one_arrow() {
        let m = mark(&chars("STACK"), &chars("TTTTT"));
        assert_eq!(m[1], Mark::Here);
        assert!(m.iter().filter(|x| **x != Mark::Absent).count() == 1);
    }

    #[test]
    fn the_streak_grows_on_following_days_and_resets() {
        assert_eq!(next_streak(None, 0, 10, true), 1);
        assert_eq!(next_streak(Some(9), 4, 10, true), 5);
        assert_eq!(next_streak(Some(7), 4, 10, true), 1);
        assert_eq!(next_streak(Some(9), 4, 10, false), 0);
    }

    #[test]
    fn a_played_day_turns_into_practice_and_keeps_the_streak() {
        let setup = Setup {
            seed: 3,
            reduced_motion: false,
            day: 70,
            practice: false,
        };
        let mut scores = Scores::in_memory();
        scores.set(DAY_KEY, 69);
        scores.set(STREAK_KEY, 2);
        let mut t = Termle::new(setup);
        t.restore(&scores);
        assert_eq!(t.daily, Some(70));
        let answer: String = t.answer.iter().collect();
        let mut fx = Fx::default();
        for ch in answer.chars() {
            t.key(Key::Char(ch), &mut fx);
        }
        t.key(Key::Enter, &mut fx);
        t.keep(&mut scores);
        assert_eq!(scores.get(STREAK_KEY), Some(3));
        assert_eq!(scores.get(BEST_STREAK_KEY), Some(3));
        assert_eq!(scores.get(DAY_KEY), Some(70));
        // the same day again is practice, with the streak still shown
        let mut again = Termle::new(setup);
        again.restore(&scores);
        assert!(again.daily.is_none() && again.daily_done);
        assert_eq!(again.streak, 3);
        // practice never touches the records
        again.keep(&mut scores);
        assert_eq!(scores.get(STREAK_KEY), Some(3));
    }

    #[test]
    fn days_become_dates() {
        assert_eq!(date(0), "1 Jan 1970");
        assert_eq!(date(20_755), "29 Oct 2026");
        assert_eq!(date(11_016), "29 Feb 2000");
    }

    #[test]
    fn the_daily_term_is_fixed_for_a_day_and_cycles_the_list() {
        assert_eq!(daily_index(100), daily_index(100));
        let n = WORDS.len() as u64;
        let start = n * 3;
        let mut seen: Vec<usize> = (start..start + n).map(daily_index).collect();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), WORDS.len());
    }

    #[test]
    fn solving_wins_and_six_misses_lose() {
        let setup = Setup {
            seed: 3,
            reduced_motion: false,
            day: 50,
            practice: false,
        };
        let mut t = Termle::new(setup);
        let answer: String = t.answer.iter().collect();
        let mut fx = Fx::default();
        for ch in answer.chars() {
            t.key(Key::Char(ch.to_ascii_lowercase()), &mut fx);
        }
        t.key(Key::Enter, &mut fx);
        assert!(matches!(t.outcome(), Some(Outcome::Won(_))));
        assert_eq!(t.score(), 600);

        let mut t = Termle::new(setup);
        for _ in 0..TRIES {
            for _ in 0..t.answer.len() {
                t.key(Key::Char('q'), &mut fx);
            }
            t.key(Key::Enter, &mut fx);
        }
        assert!(matches!(t.outcome(), Some(Outcome::Lost(_))));
    }

    #[test]
    fn the_reveal_costs_a_try() {
        let mut t = Termle::new(Setup {
            seed: 9,
            reduced_motion: false,
            day: 4,
            practice: true,
        });
        let mut fx = Fx::default();
        t.key(Key::Tab, &mut fx);
        assert_eq!(t.tries_left(), TRIES - 1);
        t.key(Key::Tab, &mut fx);
        assert_eq!(t.tries_left(), TRIES - 1);
    }
}
