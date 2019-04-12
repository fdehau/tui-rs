use std::io;

use crate::backend::Backend;
use crate::buffer::Cell;
use crate::layout::Rect;
use crate::style::{Color, Modifier};
use crossterm::error::ErrorKind;

pub struct CrosstermBackend {
    screen: Option<crossterm::Screen>,
    crossterm: crossterm::Crossterm,
    // Need to keep the AlternateScreen around even when not using it directly,
    // see https://github.com/TimonPost/crossterm/issues/88
    alternate_screen: Option<crossterm::AlternateScreen>,
}

impl Default for CrosstermBackend {
    fn default() -> CrosstermBackend {
        let screen = crossterm::Screen::default();
        let crossterm = crossterm::Crossterm::from_screen(&screen);
        CrosstermBackend {
            screen: Some(screen),
            crossterm,
            alternate_screen: None,
        }
    }
}

impl CrosstermBackend {
    pub fn new() -> CrosstermBackend {
        CrosstermBackend::default()
    }

    pub fn with_screen(screen: crossterm::Screen) -> CrosstermBackend {
        let crossterm = crossterm::Crossterm::from_screen(&screen);
        CrosstermBackend {
            screen: Some(screen),
            crossterm,
            alternate_screen: None,
        }
    }

    pub fn with_alternate_screen(
        alternate_screen: crossterm::AlternateScreen,
    ) -> Result<CrosstermBackend, io::Error> {
        let crossterm = crossterm::Crossterm::from_screen(&alternate_screen.screen);
        Ok(CrosstermBackend {
            screen: None,
            crossterm,
            alternate_screen: Some(alternate_screen),
        })
    }

    pub fn screen(&self) -> Option<&crossterm::Screen> {
        match &self.screen {
            Some(screen) => Some(&screen),
            None => None,
        }
    }

    pub fn alternate_screen(&self) -> Option<&crossterm::AlternateScreen> {
        match &self.alternate_screen {
            Some(alt_screen) => Some(&alt_screen),
            None => None,
        }
    }

    pub fn crossterm(&self) -> &crossterm::Crossterm {
        &self.crossterm
    }
}

// TODO: consider associated Error type on Backend to allow custom error types
// per backend
fn convert_error(error: ErrorKind) -> io::Error {
    match error {
        ErrorKind::IoError(err) => err,
        ErrorKind::FmtError(err) => {
            io::Error::new(io::ErrorKind::Other, format!("Invalid formatting: {}", err))
        }
        ErrorKind::ResizingTerminalFailure(err) => io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to resize terminal: {}", err),
        ),
        _ => io::Error::new(io::ErrorKind::Other, "Unknown crossterm error"),
    }
}

impl Backend for CrosstermBackend {
    fn clear(&mut self) -> io::Result<()> {
        let terminal = self.crossterm.terminal();
        terminal
            .clear(crossterm::ClearType::All)
            .map_err(convert_error)?;
        Ok(())
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        let cursor = self.crossterm.cursor();
        cursor.hide().map_err(convert_error)?;
        Ok(())
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        let cursor = self.crossterm.cursor();
        cursor.show().map_err(convert_error)?;
        Ok(())
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        let cursor = crossterm::cursor();
        Ok(cursor.pos())
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        let cursor = crossterm::cursor();
        cursor.goto(x, y).map_err(convert_error)
    }

    fn size(&self) -> io::Result<Rect> {
        let terminal = self.crossterm.terminal();
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
        let cursor = self.crossterm.cursor();
        let mut last_y = 0;
        let mut last_x = 0;
        let mut first = true;

        let stdout = io::stdout();
        let mut handle = stdout.lock();

        for (x, y, cell) in content {
            if y != last_y || x != last_x + 1 || first {
                cursor.goto(x, y).map_err(convert_error)?;
                first = false;
            }
            last_x = x;
            last_y = y;
            let mut s = self.crossterm.style(&cell.symbol);
            if let Some(color) = cell.style.fg.into() {
                s = s.with(color)
            }
            if let Some(color) = cell.style.bg.into() {
                s = s.on(color)
            }
            s.object_style.attrs = cell.style.modifier.into();

           write!(handle, "{}", s);
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
            Color::Indexed(i) => Some(crossterm::Color::AnsiValue(i)),
            Color::Rgb(r, g, b) => Some(crossterm::Color::Rgb { r, g, b }),
        }
    }
}

impl From<Modifier> for Vec<crossterm::Attribute> {
    #[cfg(unix)]
    fn from(modifier: Modifier) -> Vec<crossterm::Attribute> {
        let mut result = Vec::new();

        if modifier.contains(Modifier::BOLD) {
            result.push(crossterm::Attribute::Bold)
        }
        if modifier.contains(Modifier::DIM) {
            result.push(crossterm::Attribute::Dim)
        }
        if modifier.contains(Modifier::ITALIC) {
            result.push(crossterm::Attribute::Italic)
        }
        if modifier.contains(Modifier::UNDERLINED) {
            result.push(crossterm::Attribute::Underlined)
        }
        if modifier.contains(Modifier::SLOW_BLINK) {
            result.push(crossterm::Attribute::SlowBlink)
        }
        if modifier.contains(Modifier::RAPID_BLINK) {
            result.push(crossterm::Attribute::RapidBlink)
        }
        if modifier.contains(Modifier::REVERSED) {
            result.push(crossterm::Attribute::Reverse)
        }
        if modifier.contains(Modifier::HIDDEN) {
            result.push(crossterm::Attribute::Hidden)
        }
        if modifier.contains(Modifier::CROSSED_OUT) {
            result.push(crossterm::Attribute::CrossedOut)
        }

        result
    }

    #[cfg(windows)]
    fn from(modifier: Modifier) -> Vec<crossterm::Attribute> {
        let mut result = Vec::new();

        if modifier.contains(Modifier::BOLD) {
            result.push(crossterm::Attribute::Bold)
        }
        if modifier.contains(Modifier::UNDERLINED) {
            result.push(crossterm::Attribute::Underlined)
        }

        result
    }
}
