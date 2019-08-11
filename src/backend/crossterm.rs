use std::{
    fmt,
    io::{self, Write},
};

use crossterm::{
    execute, queue, terminal, Clear, ClearType, Crossterm, ErrorKind, Goto, Hide, SetAttr, SetBg,
    SetFg, Show,
};

use crate::{
    backend::Backend,
    buffer::Cell,
    layout::Rect,
    style::{Color, Modifier, Style},
};

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
        execute!(self.stdout, Clear(ClearType::All)).map_err(convert_error)
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        execute!(self.stdout, Hide).map_err(convert_error)
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        execute!(self.stdout, Show).map_err(convert_error)
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        let cursor = crossterm::cursor();
        Ok(cursor.pos())
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        execute!(self.stdout, Goto(x, y)).map_err(convert_error)
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
        let mut style = Style::default();
        let mut last_y = 0;
        let mut last_x = 0;
        let mut inst = 0;

        for (x, y, cell) in content {
            if y != last_y || x != last_x + 1 || inst == 0 {
                queue!(string, Goto(x, y))?;
            }
            last_x = x;
            last_y = y;
            if cell.style.modifier != style.modifier {
                let diff = ModifierDiff {
                    from: style.modifier,
                    to: cell.style.modifier,
                };
                diff.queue(&mut string)?;
                inst += 1;
                style.modifier = cell.style.modifier;
            }
            if cell.style.fg != style.fg {
                let color = to_crossterm_color(cell.style.fg);
                queue!(string, SetFg(color))?;
                style.fg = cell.style.fg;
                inst += 1;
            }
            if cell.style.bg != style.bg {
                let color = to_crossterm_color(cell.style.bg);
                queue!(string, SetBg(color))?;
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
        )?;

        Crossterm::new().color().reset()?;

        Ok(())
    }
}

#[derive(Debug)]
struct ModifierDiff {
    pub from: Modifier,
    pub to: Modifier,
}

#[cfg(unix)]
impl ModifierDiff {
    fn queue<W>(&self, mut w: W) -> io::Result<()>
    where
        W: fmt::Write,
    {
        use crossterm::Attribute;
        let removed = self.from - self.to;
        if removed.contains(Modifier::REVERSED) {
            queue!(w, SetAttr(Attribute::NoInverse))?;
        }
        if removed.contains(Modifier::BOLD) {
            queue!(w, SetAttr(Attribute::NormalIntensity))?;
            if self.to.contains(Modifier::DIM) {
                queue!(w, SetAttr(Attribute::Dim))?;
            }
        }
        if removed.contains(Modifier::ITALIC) {
            queue!(w, SetAttr(Attribute::NoItalic))?;
        }
        if removed.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttr(Attribute::NoUnderline))?;
        }
        if removed.contains(Modifier::DIM) {
            queue!(w, SetAttr(Attribute::NormalIntensity))?;
        }
        if removed.contains(Modifier::CROSSED_OUT) {
            queue!(w, SetAttr(Attribute::NotCrossedOut))?;
        }
        if removed.contains(Modifier::SLOW_BLINK) || removed.contains(Modifier::RAPID_BLINK) {
            queue!(w, SetAttr(Attribute::NoBlink))?;
        }

        let added = self.to - self.from;
        if added.contains(Modifier::REVERSED) {
            queue!(w, SetAttr(Attribute::Reverse))?;
        }
        if added.contains(Modifier::BOLD) {
            queue!(w, SetAttr(Attribute::Bold))?;
        }
        if added.contains(Modifier::ITALIC) {
            queue!(w, SetAttr(Attribute::Italic))?;
        }
        if added.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttr(Attribute::Underlined))?;
        }
        if added.contains(Modifier::DIM) {
            queue!(w, SetAttr(Attribute::Dim))?;
        }
        if added.contains(Modifier::CROSSED_OUT) {
            queue!(w, SetAttr(Attribute::CrossedOut))?;
        }
        if added.contains(Modifier::SLOW_BLINK) {
            queue!(w, SetAttr(Attribute::SlowBlink))?;
        }
        if added.contains(Modifier::RAPID_BLINK) {
            queue!(w, SetAttr(Attribute::RapidBlink))?;
        }

        Ok(())
    }
}

#[cfg(windows)]
impl ModifierDiff {
    fn queue<W>(&self, mut w: W) -> io::Result<()>
    where
        W: fmt::Write,
    {
        use crossterm::Attribute;

        let removed = self.from - self.to;
        if removed.contains(Modifier::BOLD) {
            queue!(w, SetAttr(Attribute::NormalIntensity))?;
        }
        if removed.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttr(Attribute::NoUnderline))?;
        }

        let added = self.to - self.from;
        if added.contains(Modifier::BOLD) {
            queue!(w, SetAttr(Attribute::Bold))?;
        }
        if added.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttr(Attribute::Underlined))?;
        }
        Ok(())
    }
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
