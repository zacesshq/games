//! The only module that touches a real terminal. It prints a plain line first, so a screen reader
//! hears what started and how to leave, then takes the alternate screen. The terminal is given
//! back on quit, on a panic and on Ctrl+C (a key in raw mode); on Unix also on SIGTERM, SIGHUP,
//! SIGQUIT and SIGINT. Every cell is painted on the theme's own background, so the contrast the
//! palette promises holds even when the terminal is set to another theme.

use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crossterm::cursor;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{
    self, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{execute, queue};
use zacess_scenes::colour::{ColourDepth, Rgb, nearest_256};
use zacess_scenes::frame::Frame;

static ACTIVE: AtomicBool = AtomicBool::new(false);

/// The line printed once the terminal is given back, on every way out.
pub const STOPPED: &str = "Zacess Games stopped.";

/// A key as the games see it. Letters keep their case, so a game can tell `r` from `R`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    None,
    Up,
    Down,
    Left,
    Right,
    Enter,
    Tab,
    Backspace,
    /// Space: pause in every game.
    Pause,
    /// F1. Also `?` in games that are not typed into.
    Help,
    /// Another function key, such as F2 for a hint or F5 to run.
    F(u8),
    /// A letter with Ctrl held.
    Ctrl(char),
    /// Ctrl+C, which always leaves.
    Quit,
    /// Esc: back or out in most games, pause in the games that take any typed character.
    Esc,
    Char(char),
    Resize(usize, usize),
}

/// Maps a key event. Releases and repeats of the pause key are ignored, since Windows reports
/// both edges and holding Space should not toggle the pause over and over.
pub fn map_key(key: KeyEvent) -> Key {
    if key.kind == KeyEventKind::Release {
        return Key::None;
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return match key.code {
            KeyCode::Char('c' | 'C') => Key::Quit,
            KeyCode::Char(c) if c.is_ascii_alphabetic() => Key::Ctrl(c.to_ascii_lowercase()),
            _ => Key::None,
        };
    }
    match key.code {
        KeyCode::Up => Key::Up,
        KeyCode::Down => Key::Down,
        KeyCode::Left => Key::Left,
        KeyCode::Right => Key::Right,
        KeyCode::Enter => Key::Enter,
        KeyCode::Tab | KeyCode::BackTab => Key::Tab,
        KeyCode::Backspace | KeyCode::Delete => Key::Backspace,
        KeyCode::Esc => Key::Esc,
        KeyCode::F(1) => Key::Help,
        KeyCode::F(n) => Key::F(n),
        KeyCode::Char(' ') if key.kind == KeyEventKind::Repeat => Key::None,
        KeyCode::Char(' ') => Key::Pause,
        KeyCode::Char(c) => Key::Char(c),
        _ => Key::None,
    }
}

fn restore() {
    // the music player is stopped on every way out, so none is left playing behind the shell
    crate::music::stop();
    if ACTIVE.swap(false, Ordering::SeqCst) {
        let mut out = io::stdout();
        let _ = execute!(
            out,
            EndSynchronizedUpdate,
            ResetColor,
            cursor::Show,
            LeaveAlternateScreen
        );
        let _ = terminal::disable_raw_mode();
        let _ = writeln!(out, "{STOPPED}");
        let _ = out.flush();
    }
}

#[cfg(unix)]
fn watch_signals(stop: Arc<AtomicBool>) -> io::Result<()> {
    use signal_hook::consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
    let mut signals = signal_hook::iterator::Signals::new([SIGTERM, SIGHUP, SIGQUIT, SIGINT])?;
    std::thread::spawn(move || {
        if let Some(signal) = signals.forever().next() {
            stop.store(true, Ordering::SeqCst);
            // when the terminal itself has gone the read loop may never return, so after a
            // second the terminal is restored here and the program ends
            std::thread::sleep(Duration::from_secs(1));
            if ACTIVE.load(Ordering::SeqCst) {
                restore();
                std::process::exit(128 + signal);
            }
        }
    });
    Ok(())
}

/// A taken-over terminal. Dropping it gives the terminal back.
pub struct Term {
    buffer: Vec<u8>,
    previous: Option<Frame>,
    colour: Option<Option<Rgb>>,
    depth: ColourDepth,
    background: Rgb,
    floor: f64,
    mapped: HashMap<Rgb, u8>,
    stop: Arc<AtomicBool>,
}

impl Term {
    pub fn open(intro: &str, depth: ColourDepth, background: Rgb, floor: f64) -> io::Result<Self> {
        let stop = Arc::new(AtomicBool::new(false));
        #[cfg(unix)]
        watch_signals(Arc::clone(&stop))?;
        let mut out = io::stdout();
        writeln!(out, "{intro}")?;
        out.flush()?;
        terminal::enable_raw_mode()?;
        ACTIVE.store(true, Ordering::SeqCst);
        if let Err(error) = execute!(out, EnterAlternateScreen, cursor::Hide) {
            restore();
            return Err(error);
        }
        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            restore();
            previous_hook(info);
        }));
        let mut term = Self {
            buffer: Vec::with_capacity(64 * 1024),
            previous: None,
            colour: None,
            depth,
            background,
            floor,
            mapped: HashMap::new(),
            stop,
        };
        term.reset()?;
        Ok(term)
    }

    pub fn size(&self) -> (usize, usize) {
        match terminal::size() {
            Ok((w, h)) if w > 0 && h > 0 => (w as usize, h as usize),
            _ => (80, 24),
        }
    }

    fn colour(&mut self, c: Rgb) -> Color {
        match self.depth {
            ColourDepth::Ansi256 => {
                // the background itself is matched as closely as the table allows, with no floor
                let floor = if c == self.background {
                    1.0
                } else {
                    self.floor
                };
                let bg = self.background;
                Color::AnsiValue(
                    *self
                        .mapped
                        .entry(c)
                        .or_insert_with(|| nearest_256(c, bg, floor)),
                )
            }
            ColourDepth::TrueColour => Color::Rgb {
                r: c.r,
                g: c.g,
                b: c.b,
            },
        }
    }

    /// Clears to the theme background and forgets the last frame.
    pub fn reset(&mut self) -> io::Result<()> {
        self.previous = None;
        self.colour = None;
        let bg = self.colour(self.background);
        execute!(io::stdout(), SetBackgroundColor(bg), Clear(ClearType::All))
    }

    /// Writes the cells that changed since the last frame.
    pub fn draw(&mut self, frame: &Frame) -> io::Result<()> {
        self.buffer.clear();
        let bg = self.colour(self.background);
        queue!(self.buffer, BeginSynchronizedUpdate, SetBackgroundColor(bg))?;
        let mut cursor_at: Option<(usize, usize)> = None;
        for y in 0..frame.height() {
            for x in 0..frame.width() {
                // printing in the last cell makes some terminals scroll
                if x + 1 == frame.width() && y + 1 == frame.height() {
                    continue;
                }
                let Some(cell) = frame.get(x as i32, y as i32) else {
                    continue;
                };
                if self
                    .previous
                    .as_ref()
                    .is_some_and(|p| p.get(x as i32, y as i32) == Some(cell))
                {
                    continue;
                }
                if cursor_at != Some((x, y)) {
                    queue!(self.buffer, cursor::MoveTo(x as u16, y as u16))?;
                }
                if self.colour != Some(cell.colour) {
                    let fg = match cell.colour {
                        Some(c) => self.colour(c),
                        None => Color::Reset,
                    };
                    queue!(self.buffer, SetForegroundColor(fg))?;
                    self.colour = Some(cell.colour);
                }
                queue!(self.buffer, Print(cell.ch))?;
                // ambiguous width glyphs can take two columns in some terminals, so only ASCII
                // is trusted to move the cursor on by exactly one
                cursor_at = cell.ch.is_ascii().then_some((x + 1, y));
            }
        }
        let changed = self.previous.as_ref() != Some(frame);
        self.previous = Some(frame.clone());
        if !changed {
            return Ok(());
        }
        queue!(self.buffer, cursor::MoveTo(0, 0), EndSynchronizedUpdate)?;
        let mut out = io::stdout().lock();
        out.write_all(&self.buffer)?;
        out.flush()
    }

    /// Waits up to `timeout` for a key or a resize.
    pub fn poll(&mut self, timeout: Duration) -> io::Result<Key> {
        if self.stop.load(Ordering::SeqCst) {
            return Ok(Key::Quit);
        }
        if !event::poll(timeout)? {
            return Ok(Key::None);
        }
        Ok(match event::read()? {
            Event::Key(key) => map_key(key),
            Event::Resize(..) => {
                let (w, h) = self.size();
                Key::Resize(w, h)
            }
            _ => Key::None,
        })
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        restore();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn press(code: KeyCode, modifiers: KeyModifiers) -> Key {
        map_key(KeyEvent::new(code, modifiers))
    }

    #[test]
    fn esc_and_ctrl_c_have_their_own_keys() {
        assert_eq!(press(KeyCode::Esc, KeyModifiers::NONE), Key::Esc);
        assert_eq!(press(KeyCode::Char('c'), KeyModifiers::CONTROL), Key::Quit);
        assert_eq!(
            press(KeyCode::Char('c'), KeyModifiers::NONE),
            Key::Char('c')
        );
    }

    #[test]
    fn space_pauses_and_question_mark_helps() {
        assert_eq!(press(KeyCode::Char(' '), KeyModifiers::NONE), Key::Pause);
        assert_eq!(
            press(KeyCode::Char('?'), KeyModifiers::SHIFT),
            Key::Char('?')
        );
        assert_eq!(
            press(KeyCode::Char('r'), KeyModifiers::CONTROL),
            Key::Ctrl('r')
        );
        assert_eq!(press(KeyCode::F(1), KeyModifiers::NONE), Key::Help);
    }

    #[test]
    fn letters_keep_their_case() {
        assert_eq!(
            press(KeyCode::Char('R'), KeyModifiers::SHIFT),
            Key::Char('R')
        );
    }
}
