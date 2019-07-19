use std::io;

use crate::backend::Backend;
use crate::style::{Color, Modifier};
use crate::{buffer::Cell, layout::Rect, style};
use crossterm::{
    execute, queue, terminal, Clear, ClearType, Command, Crossterm, ErrorKind, Goto, Hide, Output,
    SetAttr, SetBg, SetFg, Show,
};
use std::io::{stdout, Stdout, Write};

pub struct CrosstermBackend<W: Write> {
    alternate_screen: Option<crossterm::AlternateScreen>,
    stdout: W,
}

impl<W> CrosstermBackend<W>
where
    W: Write,
{
    pub fn new(stdout: W) -> CrosstermBackend<W> {
        CrosstermBackend {
            alternate_screen: None,
            stdout,
        }
    }

    pub fn with_alternate_screen(
        stdout: W,
        alternate_screen: crossterm::AlternateScreen,
    ) -> Result<CrosstermBackend<W>, io::Error> {
        Ok(CrosstermBackend {
            alternate_screen: Some(alternate_screen),
            stdout,
        })
    }

    pub fn alternate_screen(&self) -> Option<&crossterm::AlternateScreen> {
        match &self.alternate_screen {
            Some(alt_screen) => Some(&alt_screen),
            None => None,
        }
    }

    pub fn crossterm(&self) -> crossterm::Crossterm {
        Crossterm::new()
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

impl<W> Write for CrosstermBackend<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl<W> Backend for CrosstermBackend<W>
where
    W: Write,
{
    fn clear(&mut self) -> io::Result<()> {
        queue!(self.stdout, Clear(ClearType::All));
        self.stdout.flush();
        Ok(())
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        execute!(self.stdout, Hide);
        self.stdout.flush();
        Ok(())
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        execute!(self.stdout, Show);
        self.stdout.flush();
        Ok(())
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        let cursor = crossterm::cursor();
        Ok(cursor.pos())
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        queue!(self.stdout, Goto(x, y));
        self.stdout.flush();
        Ok(())
    }

    fn size(&self) -> io::Result<Rect> {
        let terminal = terminal();
        let (width, height) = terminal.terminal_size();
        // crossterm reports max 0-based col/row index instead of count
        Ok(Rect::new(0, 0, width, height))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        use std::fmt::Write;

        let mut string = String::with_capacity(content.size_hint().0 * 3);
        let mut style = style::Style::default();
        let mut last_y = 0;
        let mut last_x = 0;
        let mut inst = 0;

        for (x, y, cell) in content {
            if y != last_y || x != last_x + 1 || inst == 0 {
                queue!(string, Goto(x, y));
            }
            last_x = x;
            last_y = y;
            if cell.style.modifier != style.modifier {
                for attr in to_crossterm_attributes(cell.style.modifier) {
                    queue!(string, SetAttr(attr));
                }
                inst += 1;
                style.modifier = cell.style.modifier;
            }
            if cell.style.fg != style.fg {
                let color = to_crossterm_color(cell.style.fg);
                queue!(string, SetFg(color));
                style.fg = cell.style.fg;
                inst += 1;
            }
            if cell.style.bg != style.bg {
                let color = to_crossterm_color(cell.style.bg);
                queue!(string, SetBg(color));
                style.bg = cell.style.bg;
                inst += 1;
            }

            string.push_str(&cell.symbol);
            inst += 1;
        }

        write!(
            self.stdout,
            "{}{}{}{}",
            string,
            SetFg(crossterm::Color::Reset),
            SetBg(crossterm::Color::Reset),
            SetAttr(crossterm::Attribute::Reset)
        );

        Crossterm::new().color().reset();

        Ok(())
    }
}

#[cfg(unix)]
fn to_crossterm_attributes(modifier: Modifier) -> Vec<crossterm::Attribute> {
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
fn to_crossterm_attributes(modifier: Modifier) -> Vec<crossterm::Attribute> {
    let mut result = Vec::new();

    if modifier.contains(Modifier::BOLD) {
        result.push(crossterm::Attribute::Bold)
    }
    if modifier.contains(Modifier::UNDERLINED) {
        result.push(crossterm::Attribute::Underlined)
    }

    result
}

fn to_crossterm_color(color: Color) -> crossterm::Color {
    match color {
        Color::Reset => crossterm::Color::Reset,
        Color::Black => crossterm::Color::Black,
        Color::Red => crossterm::Color::DarkRed,
        Color::Green => crossterm::Color::DarkGreen,
        Color::Yellow => crossterm::Color::DarkYellow,
        Color::Blue => crossterm::Color::DarkBlue,
        Color::Magenta => crossterm::Color::DarkMagenta,
        Color::Cyan => crossterm::Color::DarkCyan,
        Color::Gray => crossterm::Color::Grey,
        Color::DarkGray => crossterm::Color::DarkGrey,
        Color::LightRed => crossterm::Color::Red,
        Color::LightGreen => crossterm::Color::Green,
        Color::LightBlue => crossterm::Color::Blue,
        Color::LightYellow => crossterm::Color::Yellow,
        Color::LightMagenta => crossterm::Color::Magenta,
        Color::LightCyan => crossterm::Color::Cyan,
        Color::White => crossterm::Color::White,
        Color::Indexed(i) => crossterm::Color::AnsiValue(i),
        Color::Rgb(r, g, b) => crossterm::Color::Rgb { r, g, b },
    }
}
