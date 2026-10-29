//! Level files written in Lua. A file returns a table or a list of tables, each with a `name`,
//! a `map` (a list of strings, one per row) and the numbers a game reads, such as `par`. Lua lets
//! a level be built with a loop or a function as well as typed out.
//!
//! A file runs in a sandbox like the one Zacess Scenes gives its scenes: no file, process,
//! network or module access, a cap on memory and a budget of instructions, so a broken file ends
//! in an error message rather than a frozen terminal. The levels that ship are built into the
//! program; files in `~/.config/zacess/games/<game>` are added after them.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

use mlua::{HookTriggers, Lua, LuaOptions, StdLib, Table, Value, VmState};

const BUDGET: i64 = 20_000_000;
const HOOK_STEP: u32 = 1_000;
const MEMORY_LIMIT: usize = 16 * 1024 * 1024;
/// The largest board a level may have, so it always fits a 64 by 24 terminal.
pub const MAX_W: usize = 18;
pub const MAX_H: usize = 14;

/// One level as the file gives it.
#[derive(Clone, Debug, PartialEq)]
pub struct Level {
    pub name: String,
    pub note: String,
    pub map: Vec<Vec<char>>,
    pub par: u32,
    pub budget: u32,
}

impl Level {
    pub fn width(&self) -> usize {
        self.map.first().map_or(0, Vec::len)
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }

    /// Whether every cell on the map's edge is wall.
    pub fn walled(&self) -> bool {
        let (w, h) = (self.width(), self.height());
        w > 2
            && h > 2
            && (0..h).all(|y| {
                (0..w).all(|x| (x > 0 && y > 0 && x < w - 1 && y < h - 1) || self.map[y][x] == '#')
            })
    }
}

pub fn sandbox() -> mlua::Result<Lua> {
    // only the libraries that touch nothing outside the state
    let lua = Lua::new_with(
        StdLib::STRING | StdLib::TABLE | StdLib::MATH,
        LuaOptions::default(),
    )?;
    let globals = lua.globals();
    for name in [
        "dofile",
        "loadfile",
        "load",
        "loadstring",
        "require",
        "collectgarbage",
        "print",
        "io",
        "os",
        "package",
        "debug",
        "coroutine",
    ] {
        globals.raw_set(name, Value::Nil)?;
    }
    if let Ok(string) = globals.get::<Table>("string") {
        string.raw_set("dump", Value::Nil)?;
    }
    lua.set_memory_limit(MEMORY_LIMIT)?;
    let remaining = Arc::new(AtomicI64::new(BUDGET / i64::from(HOOK_STEP)));
    lua.set_hook(
        HookTriggers::new().every_nth_instruction(HOOK_STEP),
        move |_, _| {
            if remaining.fetch_sub(1, Ordering::Relaxed) <= 0 {
                Err(mlua::Error::runtime(
                    "ran past its instruction budget, which usually means an endless loop",
                ))
            } else {
                Ok(VmState::Continue)
            }
        },
    )?;
    Ok(lua)
}

fn one(table: &Table, legal: &str) -> Result<Level, String> {
    let name: String = table
        .get::<Option<String>>("name")
        .map_err(|e| e.to_string())?
        .unwrap_or_else(|| "Untitled".into());
    let note: String = table
        .get::<Option<String>>("note")
        .map_err(|e| e.to_string())?
        .unwrap_or_default();
    let rows: Vec<String> = table
        .get::<Option<Vec<String>>>("map")
        .map_err(|e| format!("{name}: map {e}"))?
        .ok_or_else(|| format!("{name}: no map"))?;
    let width = rows.iter().map(|r| r.chars().count()).max().unwrap_or(0);
    if rows.is_empty() || width == 0 {
        return Err(format!("{name}: the map is empty"));
    }
    if width > MAX_W || rows.len() > MAX_H {
        return Err(format!(
            "{name}: the map is {width} by {}, larger than {MAX_W} by {MAX_H}",
            rows.len()
        ));
    }
    let mut map = Vec::new();
    for row in &rows {
        let mut cells: Vec<char> = row.chars().collect();
        if let Some(bad) = cells.iter().find(|c| !legal.contains(**c)) {
            return Err(format!("{name}: \"{bad}\" is not a tile this game knows"));
        }
        // short rows are padded with wall, so a ragged map still has a closed edge
        cells.resize(width, '#');
        map.push(cells);
    }
    let num = |key: &str| -> Result<u32, String> {
        Ok(table
            .get::<Option<u32>>(key)
            .map_err(|e| format!("{name}: {key} {e}"))?
            .unwrap_or(0))
    };
    Ok(Level {
        par: num("par")?,
        budget: num("budget")?,
        name,
        note,
        map,
    })
}

/// Runs one file's text and returns its levels. `legal` lists every tile character allowed.
pub fn parse(source: &str, chunk: &str, legal: &str) -> Result<Vec<Level>, String> {
    let lua = sandbox().map_err(|e| e.to_string())?;
    let value: Value = lua
        .load(source)
        .set_name(chunk)
        .eval()
        .map_err(|e| e.to_string().lines().next().unwrap_or("error").to_string())?;
    let Value::Table(table) = value else {
        return Err(format!("{chunk} must return a table"));
    };
    // a list of levels or a single level
    if table.contains_key("map").map_err(|e| e.to_string())? {
        return Ok(vec![one(&table, legal)?]);
    }
    let mut levels = Vec::new();
    for item in table.sequence_values::<Table>() {
        levels.push(one(&item.map_err(|e| e.to_string())?, legal)?);
    }
    Ok(levels)
}

/// The folder for a player's own level files.
pub fn user_dir(game: &str) -> Option<PathBuf> {
    let var = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty());
    let base = if let Some(dir) = var("XDG_CONFIG_HOME") {
        PathBuf::from(dir)
    } else if cfg!(windows) {
        PathBuf::from(var("APPDATA")?)
    } else {
        PathBuf::from(var("HOME")?).join(".config")
    };
    Some(base.join("zacess").join("games").join(game))
}

/// The built-in levels followed by the player's own. A broken file is skipped and its problem is
/// returned beside the levels, so the game can say what went wrong.
pub fn load(game: &str, built_in: &[(&str, &str)], legal: &str) -> (Vec<Level>, Vec<String>) {
    let mut levels = Vec::new();
    let mut problems = Vec::new();
    for (name, source) in built_in {
        match parse(source, name, legal) {
            Ok(mut l) => levels.append(&mut l),
            Err(e) => problems.push(e),
        }
    }
    if std::env::var_os("ZACESS_GAMES_NO_USER_LEVELS").is_none()
        && let Some(dir) = user_dir(game)
        && let Ok(entries) = std::fs::read_dir(dir)
    {
        let mut paths: Vec<PathBuf> = entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "lua"))
            .collect();
        paths.sort();
        for path in paths {
            let chunk = path.display().to_string();
            match std::fs::read_to_string(&path)
                .map_err(|e| e.to_string())
                .and_then(|s| parse(&s, &chunk, legal))
            {
                Ok(mut l) => levels.append(&mut l),
                Err(e) => problems.push(e),
            }
        }
    }
    (levels, problems)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_level_and_a_list_both_load() {
        let one = parse(
            "return { name = 'A', map = { '#S#', '#N' }, par = 3 }",
            "t",
            "#SN",
        )
        .unwrap();
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].par, 3);
        assert_eq!(one[0].map[1], vec!['#', 'N', '#']);
        let two = parse(
            "local l = {} for i = 1, 2 do l[i] = { name = 'L' .. i, map = { 'SN' } } end return l",
            "t",
            "SN",
        )
        .unwrap();
        assert_eq!(two.len(), 2);
        assert_eq!(two[1].name, "L2");
    }

    #[test]
    fn unknown_tiles_and_huge_maps_are_refused() {
        assert!(parse("return { map = { 'S?N' } }", "t", "SN").is_err());
        let big = format!("return {{ map = {{ '{}' }} }}", "S".repeat(MAX_W + 1));
        assert!(parse(&big, "t", "S").is_err());
    }

    #[test]
    fn the_sandbox_refuses_escapes_and_endless_loops() {
        for source in [
            "return io.open('x')",
            "return os.exit()",
            "return require('x')",
            "return load('return 1')",
            "while true do end",
        ] {
            assert!(parse(source, "t", "S").is_err(), "{source}");
        }
    }
}
