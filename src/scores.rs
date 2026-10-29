//! Best scores and small records, one `key value` line each, in `$XDG_STATE_HOME/zacess-games`
//! (`~/.local/state/zacess-games` by default, beside the Zacess theme state file) or
//! `%LOCALAPPDATA%\zacess-games` on Windows. A missing or damaged file just means no records.

use std::collections::BTreeMap;
use std::path::PathBuf;

/// Where the records live. `None` without a home folder.
pub fn state_dir() -> Option<PathBuf> {
    let var = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty());
    if let Some(dir) = var("ZACESS_GAMES_STATE") {
        return Some(PathBuf::from(dir));
    }
    if let Some(dir) = var("XDG_STATE_HOME") {
        return Some(PathBuf::from(dir).join("zacess-games"));
    }
    if cfg!(windows)
        && let Some(dir) = var("LOCALAPPDATA")
    {
        return Some(PathBuf::from(dir).join("zacess-games"));
    }
    var("HOME").or_else(|| var("USERPROFILE")).map(|home| {
        PathBuf::from(home)
            .join(".local")
            .join("state")
            .join("zacess-games")
    })
}

/// The records, loaded once and saved after each change.
#[derive(Clone, Debug, Default)]
pub struct Scores {
    values: BTreeMap<String, u64>,
    path: Option<PathBuf>,
}

impl Scores {
    pub fn load() -> Self {
        let path = state_dir().map(|d| d.join("scores.txt"));
        let text = path
            .as_ref()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .unwrap_or_default();
        Self {
            values: parse(&text),
            path,
        }
    }

    /// Records kept only in memory, for tests and headless runs.
    pub fn in_memory() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<u64> {
        self.values.get(key).copied()
    }

    pub fn set(&mut self, key: &str, value: u64) {
        self.values.insert(key.to_string(), value);
        self.save();
    }

    /// Keeps `score` if it beats the best for `game`; returns whether it did.
    pub fn offer(&mut self, game: &str, score: u64) -> bool {
        if score == 0 || self.get(game).is_some_and(|best| best >= score) {
            return false;
        }
        self.set(game, score);
        true
    }

    fn save(&self) {
        let Some(path) = &self.path else { return };
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(path, render(&self.values));
    }
}

fn parse(text: &str) -> BTreeMap<String, u64> {
    text.lines()
        .filter_map(|line| {
            let (key, value) = line.trim().split_once(' ')?;
            Some((key.to_string(), value.trim().parse().ok()?))
        })
        .collect()
}

fn render(values: &BTreeMap<String, u64>) -> String {
    let mut out = String::from("# Zacess Games records: best scores and the daily Termle\n");
    for (key, value) in values {
        out.push_str(&format!("{key} {value}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_round_trip_and_skip_damaged_lines() {
        let mut values = BTreeMap::new();
        values.insert("stackline".to_string(), 1200);
        values.insert("termle-day-20390".to_string(), 4);
        let text = render(&values) + "nonsense\nrooms abc\n";
        assert_eq!(parse(&text), values);
    }

    #[test]
    fn only_a_better_score_is_kept() {
        let mut s = Scores::in_memory();
        assert!(s.offer("trace", 50));
        assert!(!s.offer("trace", 40));
        assert!(s.offer("trace", 60));
        assert!(!s.offer("drift", 0));
        assert_eq!(s.get("trace"), Some(60));
    }
}
