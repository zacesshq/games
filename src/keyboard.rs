//! Keyboard layouts for the typing trainer: Apple British on a Mac, UK on Windows and Linux and
//! US ANSI. Each layout lists its keys row by row with the character each gives alone and with
//! Shift, so the finger map, the Shift hint and the symbol lessons follow the layout in use.
//!
//! The layout is found in this order: the `--layout` flag, the `ZACESS_KEYBOARD` variable, the
//! setting saved last time, then the system's own setting. When the system cannot be read the
//! fallback is UK (Apple British on a Mac), never US.

use std::process::Command;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    UkMac,
    UkPc,
    Us,
}

impl Layout {
    pub const ALL: [Layout; 3] = [Layout::UkMac, Layout::UkPc, Layout::Us];

    #[cfg(test)]
    pub fn id(self) -> &'static str {
        match self {
            Layout::UkMac => "uk-mac",
            Layout::UkPc => "uk-pc",
            Layout::Us => "us",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Layout::UkMac => "UK Mac (Apple British)",
            Layout::UkPc => "UK PC",
            Layout::Us => "US",
        }
    }

    pub fn parse(text: &str) -> Option<Layout> {
        match text.trim().to_ascii_lowercase().replace('_', "-").as_str() {
            "uk-mac" | "ukmac" | "mac" | "apple-british" => Some(Layout::UkMac),
            "uk-pc" | "ukpc" | "uk" | "gb" | "pc" => Some(Layout::UkPc),
            "us" | "ansi" | "us-ansi" => Some(Layout::Us),
            _ => None,
        }
    }

    pub fn index(self) -> u64 {
        Layout::ALL.iter().position(|l| *l == self).unwrap_or(0) as u64
    }

    pub fn from_index(i: u64) -> Option<Layout> {
        Layout::ALL.get(i as usize).copied()
    }

    pub fn next(self) -> Layout {
        Layout::ALL[(self.index() as usize + 1) % Layout::ALL.len()]
    }

    /// The rows of keys with how far each row is indented on screen. Each key is the character
    /// alone then with Shift.
    pub fn rows(self) -> [(i32, &'static [(char, char)]); 4] {
        const NUM_US: &[(char, char)] = &[
            ('`', '~'),
            ('1', '!'),
            ('2', '@'),
            ('3', '#'),
            ('4', '$'),
            ('5', '%'),
            ('6', '^'),
            ('7', '&'),
            ('8', '*'),
            ('9', '('),
            ('0', ')'),
            ('-', '_'),
            ('=', '+'),
        ];
        const NUM_PC: &[(char, char)] = &[
            ('`', '¬'),
            ('1', '!'),
            ('2', '"'),
            ('3', '£'),
            ('4', '$'),
            ('5', '%'),
            ('6', '^'),
            ('7', '&'),
            ('8', '*'),
            ('9', '('),
            ('0', ')'),
            ('-', '_'),
            ('=', '+'),
        ];
        const NUM_MAC: &[(char, char)] = &[
            ('§', '±'),
            ('1', '!'),
            ('2', '@'),
            ('3', '£'),
            ('4', '$'),
            ('5', '%'),
            ('6', '^'),
            ('7', '&'),
            ('8', '*'),
            ('9', '('),
            ('0', ')'),
            ('-', '_'),
            ('=', '+'),
        ];
        const TOP_US: &[(char, char)] = &[
            ('q', 'Q'),
            ('w', 'W'),
            ('e', 'E'),
            ('r', 'R'),
            ('t', 'T'),
            ('y', 'Y'),
            ('u', 'U'),
            ('i', 'I'),
            ('o', 'O'),
            ('p', 'P'),
            ('[', '{'),
            (']', '}'),
            ('\\', '|'),
        ];
        const TOP_UK: &[(char, char)] = &[
            ('q', 'Q'),
            ('w', 'W'),
            ('e', 'E'),
            ('r', 'R'),
            ('t', 'T'),
            ('y', 'Y'),
            ('u', 'U'),
            ('i', 'I'),
            ('o', 'O'),
            ('p', 'P'),
            ('[', '{'),
            (']', '}'),
        ];
        const HOME_US: &[(char, char)] = &[
            ('a', 'A'),
            ('s', 'S'),
            ('d', 'D'),
            ('f', 'F'),
            ('g', 'G'),
            ('h', 'H'),
            ('j', 'J'),
            ('k', 'K'),
            ('l', 'L'),
            (';', ':'),
            ('\'', '"'),
        ];
        const HOME_PC: &[(char, char)] = &[
            ('a', 'A'),
            ('s', 'S'),
            ('d', 'D'),
            ('f', 'F'),
            ('g', 'G'),
            ('h', 'H'),
            ('j', 'J'),
            ('k', 'K'),
            ('l', 'L'),
            (';', ':'),
            ('\'', '@'),
            ('#', '~'),
        ];
        const HOME_MAC: &[(char, char)] = &[
            ('a', 'A'),
            ('s', 'S'),
            ('d', 'D'),
            ('f', 'F'),
            ('g', 'G'),
            ('h', 'H'),
            ('j', 'J'),
            ('k', 'K'),
            ('l', 'L'),
            (';', ':'),
            ('\'', '"'),
            ('\\', '|'),
        ];
        const LOW_US: &[(char, char)] = &[
            ('z', 'Z'),
            ('x', 'X'),
            ('c', 'C'),
            ('v', 'V'),
            ('b', 'B'),
            ('n', 'N'),
            ('m', 'M'),
            (',', '<'),
            ('.', '>'),
            ('/', '?'),
        ];
        const LOW_PC: &[(char, char)] = &[
            ('\\', '|'),
            ('z', 'Z'),
            ('x', 'X'),
            ('c', 'C'),
            ('v', 'V'),
            ('b', 'B'),
            ('n', 'N'),
            ('m', 'M'),
            (',', '<'),
            ('.', '>'),
            ('/', '?'),
        ];
        const LOW_MAC: &[(char, char)] = &[
            ('`', '~'),
            ('z', 'Z'),
            ('x', 'X'),
            ('c', 'C'),
            ('v', 'V'),
            ('b', 'B'),
            ('n', 'N'),
            ('m', 'M'),
            (',', '<'),
            ('.', '>'),
            ('/', '?'),
        ];
        match self {
            Layout::Us => [(0, NUM_US), (2, TOP_US), (3, HOME_US), (5, LOW_US)],
            Layout::UkPc => [(0, NUM_PC), (2, TOP_UK), (3, HOME_PC), (1, LOW_PC)],
            Layout::UkMac => [(0, NUM_MAC), (2, TOP_UK), (3, HOME_MAC), (1, LOW_MAC)],
        }
    }

    /// Characters typed with Option (Alt) on top of a key: `#` is Option+3 on a Mac.
    pub fn option_keys(self) -> &'static [(char, char)] {
        match self {
            Layout::UkMac => &[('#', '3')],
            _ => &[],
        }
    }

    /// Where a character is typed: its key, its finger and the modifier it needs.
    pub fn find(self, ch: char) -> Option<Stroke> {
        if ch == ' ' {
            return Some(Stroke {
                key: ' ',
                finger: 8,
                shift: false,
                option: false,
            });
        }
        for (row, (_, keys)) in self.rows().iter().enumerate() {
            for (col, &(plain, shifted)) in keys.iter().enumerate() {
                if ch == plain || ch == shifted {
                    return Some(Stroke {
                        key: plain,
                        finger: self.finger_at(row, col),
                        shift: ch == shifted && ch != plain,
                        option: false,
                    });
                }
            }
        }
        for &(ch2, key) in self.option_keys() {
            if ch2 == ch {
                let mut s = self.find(key)?;
                s.shift = false;
                s.option = true;
                return Some(s);
            }
        }
        None
    }

    /// The finger for the key at a row and column, 0 to 7 from the left little finger.
    pub fn finger_at(self, row: usize, col: usize) -> u8 {
        // the bottom row of a UK keyboard has an extra key at the left, so its letters sit one
        // column further along than on a US keyboard
        let col = if row == 3 && self != Layout::Us {
            col.saturating_sub(1)
        } else {
            col
        };
        match row {
            0 => match col {
                0..=1 => 0,
                2 => 1,
                3 => 2,
                4 | 5 => 3,
                6 | 7 => 4,
                8 => 5,
                9 => 6,
                _ => 7,
            },
            _ => match col {
                0 => 0,
                1 => 1,
                2 => 2,
                3 | 4 => 3,
                5 | 6 => 4,
                7 => 5,
                8 => 6,
                _ => 7,
            },
        }
    }

    /// The symbols the layout types with Shift or Option, for the symbol lesson.
    pub fn symbols(self) -> Vec<char> {
        let mut out: Vec<char> = Vec::new();
        for (_, keys) in self.rows() {
            for &(plain, shifted) in keys {
                for c in [plain, shifted] {
                    if !c.is_ascii_alphanumeric() && !out.contains(&c) {
                        out.push(c);
                    }
                }
            }
        }
        out.extend(self.option_keys().iter().map(|(c, _)| *c));
        out
    }
}

/// How one character is typed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stroke {
    pub key: char,
    pub finger: u8,
    pub shift: bool,
    pub option: bool,
}

/// A macOS input source id, as `defaults read` gives it.
pub fn parse_mac(id: &str) -> Option<Layout> {
    let id = id.trim();
    if id.ends_with("British-PC") {
        Some(Layout::UkPc)
    } else if id.ends_with(".British") {
        Some(Layout::UkMac)
    } else if id.ends_with(".US") || id.ends_with(".ABC") || id.ends_with(".USExtended") {
        Some(Layout::Us)
    } else {
        None
    }
}

/// A Windows layout id or language tag, such as `00000809` or `0809:00000809` or `en-GB`.
pub fn parse_windows(text: &str) -> Option<Layout> {
    let t = text.to_ascii_lowercase();
    if t.contains("00000809") || t.contains("en-gb") {
        Some(Layout::UkPc)
    } else if t.contains("00000409") || t.contains("en-us") {
        Some(Layout::Us)
    } else {
        None
    }
}

/// `localectl status`, `setxkbmap -query` or a bare XKB layout name such as `gb`.
pub fn parse_linux(text: &str) -> Option<Layout> {
    let pick = |name: &str| match name.split(',').next().unwrap_or("").trim() {
        "gb" | "uk" => Some(Layout::UkPc),
        "us" => Some(Layout::Us),
        _ => None,
    };
    for line in text.lines() {
        let l = line.trim();
        if let Some(rest) = l
            .strip_prefix("X11 Layout:")
            .or_else(|| l.strip_prefix("layout:"))
        {
            return pick(rest);
        }
        if let Some(rest) = l.strip_prefix("VC Keymap:") {
            let k = rest.trim();
            if k.starts_with("gb") || k.starts_with("uk") {
                return Some(Layout::UkPc);
            }
            if k.starts_with("us") {
                return Some(Layout::Us);
            }
        }
    }
    if !text.contains(':') {
        pick(text)
    } else {
        None
    }
}

fn output(program: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(program).args(args).output().ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The fallback when the system cannot be read: UK, in the Mac's form on a Mac.
pub fn fallback() -> Layout {
    if cfg!(target_os = "macos") {
        Layout::UkMac
    } else {
        Layout::UkPc
    }
}

/// The layout the system is set to. The fallback when it cannot be read.
pub fn detect() -> Layout {
    let found = if cfg!(target_os = "macos") {
        let home = std::env::var("HOME").unwrap_or_default();
        let plist = format!("{home}/Library/Preferences/com.apple.HIToolbox");
        output(
            "defaults",
            &["read", &plist, "AppleCurrentKeyboardLayoutInputSourceID"],
        )
        .and_then(|s| parse_mac(&s))
    } else if cfg!(windows) {
        output(
            "reg",
            &["query", r"HKCU\Keyboard Layout\Preload", "/v", "1"],
        )
        .and_then(|s| parse_windows(&s))
    } else {
        std::env::var("XKB_DEFAULT_LAYOUT")
            .ok()
            .and_then(|s| parse_linux(&s))
            .or_else(|| output("localectl", &["status"]).and_then(|s| parse_linux(&s)))
            .or_else(|| output("setxkbmap", &["-query"]).and_then(|s| parse_linux(&s)))
    };
    found.unwrap_or_else(fallback)
}

/// The system's layout, read once a run.
pub fn detected() -> Layout {
    static FOUND: OnceLock<Layout> = OnceLock::new();
    *FOUND.get_or_init(detect)
}

/// The variable that names a layout.
pub const ENV_VAR: &str = "ZACESS_KEYBOARD";
/// The key the chosen layout is saved under in the records.
pub const SETTING: &str = "setting-keyboard";

static FLAG: OnceLock<Layout> = OnceLock::new();

/// Records the `--layout` flag for this run.
pub fn set_flag(layout: Layout) {
    let _ = FLAG.set(layout);
}

/// The layout given for this run by the flag or the variable, if any.
pub fn chosen() -> Option<Layout> {
    FLAG.get()
        .copied()
        .or_else(|| std::env::var(ENV_VAR).ok().and_then(|v| Layout::parse(&v)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_home_row_is_the_same_on_every_layout() {
        for l in Layout::ALL {
            for (ch, finger) in [
                ('a', 0),
                ('s', 1),
                ('d', 2),
                ('f', 3),
                ('j', 4),
                ('k', 5),
                ('l', 6),
                (';', 7),
            ] {
                assert_eq!(l.find(ch).unwrap().finger, finger, "{} {ch}", l.id());
            }
            for (ch, finger) in [
                ('z', 0),
                ('x', 1),
                ('c', 2),
                ('v', 3),
                ('m', 4),
                (',', 5),
                ('.', 6),
                ('/', 7),
            ] {
                assert_eq!(l.find(ch).unwrap().finger, finger, "{} {ch}", l.id());
            }
            assert_eq!(
                l.find('Q').unwrap(),
                Stroke {
                    key: 'q',
                    finger: 0,
                    shift: true,
                    option: false
                }
            );
            assert_eq!(l.find(' ').unwrap().finger, 8);
        }
    }

    #[test]
    fn apple_british_puts_at_on_2_and_hash_on_option_3() {
        let l = Layout::UkMac;
        assert_eq!(
            l.find('@').unwrap(),
            Stroke {
                key: '2',
                finger: 1,
                shift: true,
                option: false
            }
        );
        assert_eq!(l.find('"').unwrap().key, '\'');
        assert_eq!(l.find('£').unwrap().key, '3');
        assert_eq!(
            l.find('#').unwrap(),
            Stroke {
                key: '3',
                finger: 2,
                shift: false,
                option: true
            }
        );
        assert_eq!(l.find('§').unwrap().finger, 0);
        assert!(l.find('±').unwrap().shift);
        assert_eq!(l.find('`').unwrap().finger, 0);
        assert!(l.find('~').unwrap().shift);
        assert_eq!(l.find('~').unwrap().key, '`');
    }

    #[test]
    fn uk_pc_puts_the_quote_on_2_and_hash_by_enter() {
        let l = Layout::UkPc;
        assert_eq!(l.find('"').unwrap().key, '2');
        assert_eq!(l.find('@').unwrap().key, '\'');
        assert_eq!(l.find('£').unwrap().key, '3');
        let hash = l.find('#').unwrap();
        assert_eq!((hash.key, hash.finger, hash.shift), ('#', 7, false));
        assert_eq!(l.find('~').unwrap().key, '#');
        assert_eq!(l.find('\\').unwrap().finger, 0);
        assert_eq!(l.find('|').unwrap().key, '\\');
        assert_eq!(l.find('¬').unwrap().key, '`');
    }

    #[test]
    fn us_keeps_the_ansi_places() {
        let l = Layout::Us;
        assert_eq!(l.find('@').unwrap().key, '2');
        assert_eq!(l.find('"').unwrap().key, '\'');
        assert_eq!(l.find('#').unwrap().key, '3');
        assert_eq!(l.find('\\').unwrap().finger, 7);
        assert!(l.find('£').is_none());
    }

    #[test]
    fn the_symbols_lesson_follows_the_layout() {
        assert!(Layout::UkMac.symbols().contains(&'£'));
        assert!(Layout::UkMac.symbols().contains(&'#'));
        assert!(Layout::UkPc.symbols().contains(&'¬'));
        assert!(!Layout::Us.symbols().contains(&'£'));
        for l in Layout::ALL {
            for c in l.symbols() {
                assert!(l.find(c).is_some(), "{} {c}", l.id());
            }
        }
    }

    #[test]
    fn detection_reads_each_system() {
        assert_eq!(
            parse_mac("com.apple.keylayout.British\n"),
            Some(Layout::UkMac)
        );
        assert_eq!(
            parse_mac("com.apple.keylayout.British-PC"),
            Some(Layout::UkPc)
        );
        assert_eq!(parse_mac("com.apple.keylayout.ABC"), Some(Layout::Us));
        assert_eq!(parse_mac("com.apple.keylayout.Dvorak"), None);
        assert_eq!(
            parse_windows("    1    REG_SZ    00000809\r\n"),
            Some(Layout::UkPc)
        );
        assert_eq!(parse_windows("0409:00000409"), Some(Layout::Us));
        assert_eq!(parse_windows("en-GB"), Some(Layout::UkPc));
        assert_eq!(
            parse_linux(
                "   System Locale: LANG=en_GB.UTF-8\n       VC Keymap: gb\n      X11 Layout: gb\n"
            ),
            Some(Layout::UkPc)
        );
        assert_eq!(
            parse_linux("rules:      evdev\nmodel:      pc105\nlayout:     us\n"),
            Some(Layout::Us)
        );
        assert_eq!(parse_linux("gb"), Some(Layout::UkPc));
        assert_eq!(parse_linux("de"), None);
        assert_eq!(Layout::parse("UK_MAC"), Some(Layout::UkMac));
        assert_ne!(fallback(), Layout::Us);
    }
}
