//! Colours, glyphs and the frame every game is drawn inside: a title bar in the top border, a
//! status bar, the play area, a key hint line and the overlays for pause, help and the end of a
//! game. Every colour comes from a palette role that is checked against the theme's floor, so no
//! game can draw below 4.5:1 (7:1 on the High Contrast and Deep Contrast themes).

use std::sync::atomic::{AtomicBool, Ordering};

use zacess_scenes::colour::Rgb;
use zacess_scenes::frame::Frame;
use zacess_scenes::lua::Role;
use zacess_scenes::theme::ThemeData;

/// The smallest terminal the games draw in, about a quarter of a tmux window. Below it a plain
/// message asks for more room.
pub const MIN_WIDTH: usize = 60;
pub const MIN_HEIGHT: usize = 20;
/// From here on the window is standard: two key lines and the full side panels.
pub const STANDARD: (usize, usize) = (100, 30);
/// From here on the window is large: boards drawn at twice the size.
pub const LARGE: (usize, usize) = (160, 45);

/// How much room there is, chosen from the terminal size on every frame, so a resize mid-game
/// changes the layout at once and loses nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    /// Below the minimum: only a message.
    TooSmall,
    /// One status line, one key line, no side panels where they would not fit.
    Compact,
    Standard,
    /// Boards at twice the size and fuller panels.
    Large,
}

pub fn tier(w: usize, h: usize) -> Tier {
    if w < MIN_WIDTH || h < MIN_HEIGHT {
        Tier::TooSmall
    } else if w >= LARGE.0 && h >= LARGE.1 {
        Tier::Large
    } else if w >= STANDARD.0 && h >= STANDARD.1 {
        Tier::Standard
    } else {
        Tier::Compact
    }
}

static BASIC: AtomicBool = AtomicBool::new(false);

/// Limits the glyphs to ASCII plus box drawing and blocks, for fonts without geometric shapes.
pub fn set_basic(on: bool) {
    BASIC.store(on, Ordering::Relaxed);
}

/// A geometric shape or its plain stand-in where the font may lack it.
pub fn g(fancy: char, plain: char) -> char {
    if BASIC.load(Ordering::Relaxed) {
        plain
    } else {
        fancy
    }
}

/// The colours a game draws with, each one already checked against the theme's floor.
#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub theme: &'static ThemeData,
    pub text: Rgb,
    pub muted: Rgb,
    pub bright: Rgb,
    pub accent: Rgb,
    pub error: Rgb,
    pub warning: Rgb,
    pub info: Rgb,
    pub red: Rgb,
    pub green: Rgb,
    pub yellow: Rgb,
    pub blue: Rgb,
    pub magenta: Rgb,
    pub cyan: Rgb,
    pub keyword: Rgb,
    pub number: Rgb,
    pub link: Rgb,
    pub property: Rgb,
    pub function: Rgb,
    pub string: Rgb,
}

impl Palette {
    pub fn new(theme: &'static ThemeData) -> Self {
        let c = |role: Role| role.colour(theme);
        Self {
            theme,
            text: c(Role::Text),
            muted: c(Role::Muted),
            bright: c(Role::Bright),
            accent: c(Role::Accent),
            error: c(Role::Error),
            warning: c(Role::Warning),
            info: c(Role::Info),
            red: c(Role::Red),
            green: c(Role::Green),
            yellow: c(Role::Yellow),
            blue: c(Role::Blue),
            magenta: c(Role::Magenta),
            cyan: c(Role::Cyan),
            keyword: c(Role::Syntax(0)),
            number: c(Role::Syntax(5)),
            link: c(Role::Link),
            property: c(Role::Syntax(7)),
            function: c(Role::Syntax(2)),
            string: c(Role::Syntax(4)),
        }
    }

    /// Every colour of the palette, for the contrast tests.
    #[cfg(test)]
    pub fn all(&self) -> Vec<Rgb> {
        vec![
            self.text,
            self.muted,
            self.bright,
            self.accent,
            self.error,
            self.warning,
            self.info,
            self.red,
            self.green,
            self.yellow,
            self.blue,
            self.magenta,
            self.cyan,
            self.keyword,
            self.number,
            self.link,
            self.property,
            self.function,
            self.string,
        ]
    }

    /// A game's own colour, by the name its info gives.
    pub fn identity(&self, hue: Hue) -> Rgb {
        match hue {
            Hue::Cyan => self.cyan,
            Hue::Magenta => self.magenta,
            Hue::Yellow => self.yellow,
            Hue::Blue => self.blue,
            Hue::Green => self.green,
            Hue::String => self.string,
            Hue::Red => self.red,
            Hue::Function => self.function,
            Hue::Keyword => self.keyword,
            Hue::Number => self.number,
            Hue::Accent => self.accent,
            Hue::Link => self.link,
            Hue::Property => self.property,
            Hue::Info => self.info,
        }
    }
}

/// The colour that sets a game apart in the menu and on its frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hue {
    Cyan,
    Magenta,
    Yellow,
    Blue,
    Green,
    String,
    Red,
    Function,
    Keyword,
    Number,
    Accent,
    Link,
    Property,
    Info,
}

/// A rectangle of cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    /// A rectangle of `w` by `h` centred inside this one.
    pub fn centre(&self, w: i32, h: i32) -> Rect {
        Rect::new(
            self.x + (self.w - w).max(0) / 2,
            self.y + (self.h - h).max(0) / 2,
            w,
            h,
        )
    }
}

/// Box drawing styles. Rounded for the outer window, heavy for a game's own board and double for
/// the overlays, so each layer has its own outline as well as its own colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line {
    Round,
    Light,
    Heavy,
    Double,
}

impl Line {
    fn parts(self) -> [char; 6] {
        match self {
            Line::Round => ['╭', '╮', '╰', '╯', '─', '│'],
            Line::Light => ['┌', '┐', '└', '┘', '─', '│'],
            Line::Heavy => ['┏', '┓', '┗', '┛', '━', '┃'],
            Line::Double => ['╔', '╗', '╚', '╝', '═', '║'],
        }
    }
}

/// A frame with a palette, the drawing surface every game gets.
pub struct Canvas<'a> {
    pub frame: &'a mut Frame,
    pub pal: Palette,
    /// The size class of the whole window, for games that lay themselves out by it.
    pub tier: Tier,
    /// Where drawing is allowed. A cell outside it is dropped and counted, so the tests can
    /// prove a game never spills past its play area at any size.
    pub clip: Rect,
    pub outside: usize,
}

impl<'a> Canvas<'a> {
    pub fn new(frame: &'a mut Frame, pal: Palette) -> Self {
        let (w, h) = (frame.width(), frame.height());
        Self {
            frame,
            pal,
            tier: tier(w, h),
            clip: Rect::new(0, 0, w as i32, h as i32),
            outside: 0,
        }
    }

    fn inside(&self, x: i32, y: i32) -> bool {
        x >= self.clip.x
            && y >= self.clip.y
            && x < self.clip.x + self.clip.w
            && y < self.clip.y + self.clip.h
    }

    pub fn put(&mut self, x: i32, y: i32, ch: char, colour: Rgb) {
        if self.inside(x, y) {
            self.frame.put(x, y, ch, Some(colour));
        } else if ch != ' ' {
            self.outside += 1;
        }
    }

    pub fn text(&mut self, x: i32, y: i32, text: &str, colour: Rgb) {
        for (i, ch) in text.chars().enumerate() {
            self.put(x + i as i32, y, ch, colour);
        }
    }

    /// Blanks a rectangle, so an overlay hides what is under it.
    pub fn blank(&mut self, r: Rect) {
        for y in r.y..r.y + r.h {
            for x in r.x..r.x + r.w {
                if self.inside(x, y) {
                    self.frame.put(x, y, ' ', None);
                }
            }
        }
    }

    pub fn boxed(&mut self, r: Rect, line: Line, colour: Rgb) {
        let [tl, tr, bl, br, h, v] = line.parts();
        let (x2, y2) = (r.x + r.w - 1, r.y + r.h - 1);
        for x in r.x + 1..x2 {
            self.put(x, r.y, h, colour);
            self.put(x, y2, h, colour);
        }
        for y in r.y + 1..y2 {
            self.put(r.x, y, v, colour);
            self.put(x2, y, v, colour);
        }
        self.put(r.x, r.y, tl, colour);
        self.put(x2, r.y, tr, colour);
        self.put(r.x, y2, bl, colour);
        self.put(x2, y2, br, colour);
    }

    /// A box with a label set into its top edge.
    pub fn titled(&mut self, r: Rect, line: Line, colour: Rgb, title: &str, title_colour: Rgb) {
        self.boxed(r, line, colour);
        if !title.is_empty() {
            let label = format!(" {title} ");
            self.text(r.x + 2, r.y, &label, title_colour);
        }
    }

    /// A horizontal rule joined to the outer window's sides.
    pub fn rule(&mut self, x: i32, y: i32, w: i32, colour: Rgb) {
        for i in 1..w - 1 {
            self.put(x + i, y, '─', colour);
        }
        self.put(x, y, '├', colour);
        self.put(x + w - 1, y, '┤', colour);
    }

    /// A bar of `width` cells filled to `share`, with eighths for the end so it moves smoothly.
    pub fn meter(&mut self, x: i32, y: i32, width: i32, share: f32, colour: Rgb, track: Rgb) {
        let share = share.clamp(0.0, 1.0);
        let eighths = (share * width as f32 * 8.0).round() as i32;
        for i in 0..width {
            let filled = (eighths - i * 8).clamp(0, 8);
            let ch = match filled {
                0 => '·',
                8 => '█',
                n => ['▏', '▎', '▍', '▌', '▋', '▊', '▉'][(n - 1) as usize],
            };
            self.put(x + i, y, ch, if filled == 0 { track } else { colour });
        }
    }
}

/// What the status bar shows: label and value pairs.
pub type Status = Vec<(String, String)>;

/// The parts of the window around the play area.
pub struct Chrome<'a> {
    pub title: &'a str,
    pub icon: char,
    pub identity: Rgb,
    pub status: &'a Status,
    pub hints: &'a str,
    /// The keys every game shares, on a line of their own under the game's keys.
    pub common: &'a str,
    /// The effects and music settings, always shown at the right of the status bar.
    pub sound: &'a str,
    /// The shared keys in a few words, for the single key line of a compact window.
    pub common_short: &'a str,
    /// A short line about what just happened, written so sound is never the only signal.
    pub toast: Option<&'a str>,
}

/// The longest stretch of a key line that fits, cut at a ` · ` so no hint is half shown.
pub fn fit_hints(text: &str, room: usize) -> String {
    if text.chars().count() <= room {
        return text.to_string();
    }
    let mut out = String::new();
    for part in text.split(" · ") {
        let next = if out.is_empty() {
            part.to_string()
        } else {
            format!("{out} · {part}")
        };
        if next.chars().count() > room {
            break;
        }
        out = next;
    }
    out
}

/// Draws the window and returns the play area inside it. Compact windows get one status line
/// and one key line; standard and large ones a status bar and two key lines.
pub fn chrome(c: &mut Canvas, chrome: &Chrome) -> Rect {
    let (w, h) = (c.frame.width() as i32, c.frame.height() as i32);
    let pal = c.pal;
    let compact = c.tier == Tier::Compact;
    c.boxed(Rect::new(0, 0, w, h), Line::Round, pal.muted);
    // the title set into the top border, the theme's name at the right when there is room
    let title = format!(" {} {} ", chrome.icon, chrome.title);
    c.text(2, 0, &title, chrome.identity);
    let after = 2 + title.chars().count() as i32;
    let theme = format!(" {} ", pal.theme.name.trim_start_matches("Zacess "));
    if !compact {
        c.text(after, 0, "· Zacess Games ", pal.muted);
    }
    let theme_x = w - 2 - theme.chars().count() as i32;
    if theme_x > after + if compact { 1 } else { 16 } {
        c.text(theme_x, 0, &theme, pal.muted);
    }

    // the status bar: labels muted, values in the text colour, split by bars; the sound settings
    // sit at the right end so they are always in the same place
    let sound_x = w - 2 - chrome.sound.chars().count() as i32;
    let mut x = 2;
    let gap = if compact { " · " } else { " │ " };
    for (i, (label, value)) in chrome.status.iter().enumerate() {
        let wide = 3 + label.chars().count() as i32 + 1 + value.chars().count() as i32;
        if x + wide >= sound_x - 1 {
            break;
        }
        if i > 0 {
            c.text(x, 1, gap, pal.muted);
            x += 3;
        }
        c.text(x, 1, label, pal.muted);
        x += label.chars().count() as i32 + 1;
        c.text(x, 1, value, pal.text);
        x += value.chars().count() as i32;
    }
    c.text(sound_x, 1, chrome.sound, pal.text);
    c.rule(0, 2, w, pal.muted);
    let room = (w - 4) as usize;
    if compact {
        // one key line: the game's own keys as far as they fit, then the shared ones
        let mut line = fit_hints(chrome.hints, room);
        let extra = chrome.common_short;
        if line.chars().count() + 3 + extra.chars().count() <= room {
            line = format!("{line} · {extra}");
        }
        c.text(2, h - 2, &line, pal.text);
        if let Some(toast) = chrome.toast {
            let t = format!(" {} {toast} ", g('◆', '*'));
            let tw = t.chars().count() as i32;
            if tw < w - 6 {
                c.text(w - 3 - tw, h - 1, &t, chrome.identity);
            }
        }
        return Rect::new(1, 3, w - 2, h - 5);
    }
    c.rule(0, h - 4, w, pal.muted);
    // a note about what just happened, set into the rule above the keys
    if let Some(toast) = chrome.toast {
        let t = format!(" {} {toast} ", g('◆', '*'));
        let tw = t.chars().count() as i32;
        if tw < w - 6 {
            c.text(w - 3 - tw, h - 4, &t, chrome.identity);
        }
    }
    c.text(2, h - 3, &fit_hints(chrome.hints, room), pal.text);
    c.text(2, h - 2, &fit_hints(chrome.common, room), pal.muted);
    Rect::new(1, 3, w - 2, h - 7)
}

/// The cell a glyph becomes when a drawing is stretched to twice its width: lines and blocks
/// carry on into the new cell; everything else leaves it blank.
fn stretch_right(ch: char) -> char {
    match ch {
        '─' | '┌' | '└' | '├' | '┬' | '┴' | '┼' | '╭' | '╰' | '╶' | '╼' => {
            '─'
        }
        '━' | '┏' | '┗' | '┣' | '┳' | '┻' | '╋' | '╺' => '━',
        '═' | '╔' | '╚' | '╠' | '╦' | '╩' | '╬' => '═',
        '┄' => '┄',
        '┈' => '┈',
        '█' | '▓' | '▒' | '░' | '▀' | '▄' | '▁' | '▂' | '▃' | '▅' | '▆' | '▇' | '▤' => {
            ch
        }
        _ => ' ',
    }
}

/// The cell under a glyph when a drawing is stretched to twice its height.
fn stretch_down(ch: char) -> char {
    match ch {
        '│' | '┌' | '┐' | '├' | '┤' | '┬' | '┼' | '╭' | '╮' | '╷' | '┊' => {
            '│'
        }
        '┃' | '┏' | '┓' | '┣' | '┫' | '┳' | '╋' | '╻' | '╽' => '┃',
        '║' | '╔' | '╗' | '╠' | '╣' | '╦' | '╬' => '║',
        '█' | '▓' | '▒' | '░' | '▤' => ch,
        '▄' | '▁' | '▂' | '▃' | '▅' | '▆' | '▇' => '█',
        _ => ' ',
    }
}

/// Copies a drawing into `into` at twice its size, at `at`. Lines and blocks are stretched so
/// boards and boxes stay joined up. Runs of text keep their letters together, so words stay
/// readable rather than spaced out.
pub fn zoom(from: &Frame, into: &mut Frame, at: (i32, i32)) {
    for y in 0..from.height() as i32 {
        let mut x = 0;
        let w = from.width() as i32;
        while x < w {
            let Some(cell) = from.get(x, y) else { break };
            let is_text = |c: char| c.is_alphanumeric() || ".,:;!?'\"%()/+-=<>[]_#@&°".contains(c);
            if is_text(cell.ch) {
                // a run of text, spaces between words included, is written once at the start
                let mut run = Vec::new();
                let mut end = x;
                while end < w {
                    let Some(c) = from.get(end, y) else { break };
                    let gap = c.ch == ' ' && from.get(end + 1, y).is_some_and(|n| is_text(n.ch));
                    if is_text(c.ch) || gap {
                        run.push(c);
                        end += 1;
                    } else {
                        break;
                    }
                }
                // the run sits in the middle of its doubled span; a line it interrupts, such as a
                // box edge around a title, carries on either side of it
                let n = run.len() as i32;
                let off = n / 2;
                let before = if x > 0 { from.get(x - 1, y) } else { None };
                let fill = before.map_or(' ', |b| stretch_right(b.ch));
                let fill_colour = before.and_then(|b| b.colour);
                for i in 0..2 * n {
                    if i < off || i >= off + n {
                        into.put(at.0 + 2 * x + i, at.1 + 2 * y, fill, fill_colour);
                    }
                }
                for (i, c) in run.iter().enumerate() {
                    into.put(at.0 + 2 * x + off + i as i32, at.1 + 2 * y, c.ch, c.colour);
                }
                x = end;
                continue;
            }
            let right = stretch_right(cell.ch);
            into.put(at.0 + 2 * x, at.1 + 2 * y, cell.ch, cell.colour);
            into.put(at.0 + 2 * x + 1, at.1 + 2 * y, right, cell.colour);
            let down = stretch_down(cell.ch);
            into.put(at.0 + 2 * x, at.1 + 2 * y + 1, down, cell.colour);
            let corner = if stretch_right(down) != ' ' && down != '│' && down != '┃' && down != '║'
            {
                stretch_right(down)
            } else {
                ' '
            };
            into.put(at.0 + 2 * x + 1, at.1 + 2 * y + 1, corner, cell.colour);
            x += 1;
        }
    }
}

/// Splits text into lines of at most `width` characters at spaces.
pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

/// The lines of a How to play page at a width: `# ` lines become headings with a gap above,
/// every other line a wrapped paragraph. A line starting `- ` keeps a hanging indent.
pub fn page_lines(source: &[String], width: usize) -> Vec<(bool, String)> {
    let mut out: Vec<(bool, String)> = Vec::new();
    for line in source {
        if let Some(head) = line.strip_prefix("# ") {
            if !out.is_empty() {
                out.push((false, String::new()));
            }
            out.push((true, head.to_string()));
        } else if let Some(item) = line.strip_prefix("- ") {
            for (i, l) in wrap(item, width.saturating_sub(2)).into_iter().enumerate() {
                out.push((
                    false,
                    if i == 0 {
                        format!("- {l}")
                    } else {
                        format!("  {l}")
                    },
                ));
            }
        } else {
            out.extend(wrap(line, width).into_iter().map(|l| (false, l)));
        }
    }
    out
}

/// A How to play page over the whole play area, scrolled to `scroll`, with headings in the
/// game's colour. Returns the largest useful scroll.
pub fn page(
    c: &mut Canvas,
    area: Rect,
    title: &str,
    source: &[String],
    scroll: usize,
    colour: Rgb,
) -> usize {
    let pal = c.pal;
    let w = (area.w - 2).min(72);
    let r = Rect::new(area.x + (area.w - w) / 2, area.y, w, area.h);
    c.blank(r);
    c.titled(r, Line::Double, colour, title, pal.bright);
    let width = (r.w - 4) as usize;
    let lines = page_lines(source, width);
    let rows = (r.h - 3).max(1) as usize;
    let most = lines.len().saturating_sub(rows);
    let scroll = scroll.min(most);
    for (i, (head, text)) in lines.iter().skip(scroll).take(rows).enumerate() {
        let colour = if *head { colour } else { pal.text };
        c.text(r.x + 2, r.y + 1 + i as i32, text, colour);
    }
    let foot = if most == 0 {
        "Any key closes this page".to_string()
    } else {
        format!(
            "↑ ↓ scroll ({} of {}) · any other key closes",
            scroll + 1,
            most + 1
        )
    };
    c.text(r.x + 2, r.y + r.h - 2, &foot, pal.muted);
    most
}

/// The message drawn when the terminal is too small to play in.
pub fn too_small(c: &mut Canvas) {
    let pal = c.pal;
    let (w, h) = (c.frame.width() as i32, c.frame.height() as i32);
    let lines = [
        "The terminal is too small.".to_string(),
        format!("It needs {MIN_WIDTH} by {MIN_HEIGHT}."),
        format!("It is {w} by {h} now."),
        "Esc quits.".to_string(),
    ];
    let top = ((h - lines.len() as i32) / 2).max(0);
    for (i, line) in lines.iter().enumerate() {
        let x = ((w - line.chars().count() as i32) / 2).max(0);
        c.text(
            x,
            top + i as i32,
            line,
            if i == 0 { pal.text } else { pal.muted },
        );
    }
}

/// A box in the middle of the play area with a heading and lines of text, for pause, help and
/// the end of a game.
pub fn overlay(c: &mut Canvas, area: Rect, heading: &str, lines: &[String], colour: Rgb) {
    let pal = c.pal;
    let inner = lines
        .iter()
        .map(|l| l.chars().count())
        .chain(std::iter::once(heading.chars().count() + 4))
        .max()
        .unwrap_or(10) as i32;
    let w = (inner + 6).min(area.w);
    let h = (lines.len() as i32 + 4).min(area.h);
    let r = area.centre(w, h);
    c.blank(r);
    c.titled(r, Line::Double, colour, heading, pal.bright);
    for (i, line) in lines.iter().enumerate() {
        if 2 + i as i32 >= h - 1 {
            break;
        }
        c.text(r.x + 3, r.y + 2 + i as i32, line, pal.text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zacess_scenes::theme;

    #[test]
    fn every_palette_colour_keeps_the_theme_floor() {
        for t in theme::all() {
            let pal = Palette::new(t);
            for colour in pal.all() {
                assert!(
                    t.contrast_of(colour) >= t.floor(),
                    "{} {} is {:.2}:1",
                    t.name,
                    colour.hex(),
                    t.contrast_of(colour)
                );
            }
        }
    }

    #[test]
    fn the_strong_themes_use_the_seven_to_one_floor() {
        for t in theme::all() {
            let strong = t.name.contains("High Contrast") || t.name.contains("Deep Contrast");
            assert_eq!(t.floor() >= 7.0, strong, "{}", t.name);
        }
    }

    #[test]
    fn the_meter_fills_in_eighths() {
        let mut frame = Frame::new(10, 1);
        let pal = Palette::new(theme::default_theme());
        let mut c = Canvas::new(&mut frame, pal);
        c.meter(0, 0, 4, 0.5, pal.green, pal.muted);
        assert_eq!(frame.row_text(0).trim_end(), "██··");
    }
}
