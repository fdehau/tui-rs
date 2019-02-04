use std::io;

use crate::backend::Backend;
use crate::buffer::Cell;
use crate::layout::Rect;
use crate::style::{Color, Modifier};

pub struct CrosstermBackend {
    screen: crossterm::Screen,
}

impl CrosstermBackend {
    pub fn new() -> CrosstermBackend {
        CrosstermBackend {
            screen: crossterm::Screen::default(),
        }
    }

    pub fn with_screen(screen: crossterm::Screen) -> CrosstermBackend {
        CrosstermBackend { screen: screen }
    }

    pub fn screen(&self) -> &crossterm::Screen {
        &self.screen
    }
}

impl Backend for CrosstermBackend {
    fn clear(&mut self) -> io::Result<()> {
        let terminal = crossterm::terminal::terminal();
        terminal.clear(crossterm::terminal::ClearType::All);
        Ok(())
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        let cursor = crossterm::cursor();
        cursor.hide();
        Ok(())
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        let cursor = crossterm::cursor();
        cursor.show();
        Ok(())
    }

    fn size(&self) -> io::Result<Rect> {
        let terminal = crossterm::terminal::terminal();
        let (width, height) = terminal.terminal_size();
        Ok(Rect::new(0, 0, width, height))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let cursor = crossterm::cursor();
        let mut last_y = 0;
        let mut last_x = 0;
        let mut first = true;
        for (x, y, cell) in content {
            if y != last_y || x != last_x + 1 || first {
                cursor.goto(x, y);
                first = false;
            }
            last_x = x;
            last_y = y;

            let mut s = crossterm::style(&cell.symbol);
            if let Some(color) = cell.style.fg.into() {
                s = s.with(color)
            }
            if let Some(color) = cell.style.bg.into() {
                s = s.on(color)
            }
            if let Some(attr) = cell.style.modifier.into() {
                s = s.attr(attr)
            }
            s.paint(&self.screen);
        }
        Ok(())
    }
}

impl From<Color> for Option<crossterm::Color> {
    fn from(color: Color) -> Option<crossterm::Color> {
        match color {
            Color::Reset => None,
            Color::Black => Some(crossterm::Color::Black),
            Color::Red => Some(crossterm::Color::DarkRed),
            Color::Green => Some(crossterm::Color::DarkGreen),
            Color::Yellow => Some(crossterm::Color::DarkYellow),
            Color::Blue => Some(crossterm::Color::DarkBlue),
            Color::Magenta => Some(crossterm::Color::DarkMagenta),
            Color::Cyan => Some(crossterm::Color::DarkCyan),
            Color::Gray => Some(crossterm::Color::Grey),
            Color::DarkGray => Some(crossterm::Color::Grey),
            Color::LightRed => Some(crossterm::Color::Red),
            Color::LightGreen => Some(crossterm::Color::Green),
            Color::LightBlue => Some(crossterm::Color::Blue),
            Color::LightYellow => Some(crossterm::Color::Yellow),
            Color::LightMagenta => Some(crossterm::Color::Magenta),
            Color::LightCyan => Some(crossterm::Color::Cyan),
            Color::White => Some(crossterm::Color::White),
            Color::Rgb(r, g, b) => Some(crossterm::Color::Rgb { r, g, b }),
        }
    }
}

impl From<Modifier> for Option<crossterm::Attribute> {
    #[cfg(unix)]
    fn from(modifier: Modifier) -> Option<crossterm::Attribute> {
        match modifier {
            Modifier::Blink => Some(crossterm::Attribute::SlowBlink),
            Modifier::Bold => Some(crossterm::Attribute::Bold),
            Modifier::CrossedOut => Some(crossterm::Attribute::CrossedOut),
            Modifier::Faint => Some(crossterm::Attribute::Dim),
            Modifier::Invert => Some(crossterm::Attribute::Reverse),
            Modifier::Italic => Some(crossterm::Attribute::Italic),
            Modifier::Underline => Some(crossterm::Attribute::Underlined),
            _ => None,
        }
    }

    #[cfg(windows)]
    fn from(modifier: Modifier) -> Option<crossterm::Attribute> {
        match modifier {
            Modifier::Bold => Some(crossterm::Attribute::Bold),
            Modifier::Underline => Some(crossterm::Attribute::Underlined),
            _ => None,
        }
    }
}
