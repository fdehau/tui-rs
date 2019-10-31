use std::{fmt, io};

use crate::backend::Backend;
use crate::style::{Color, Modifier};
use crate::{buffer::Cell, layout::Rect, style};
use crossterm::{
    execute, queue, terminal, Clear, ClearType, Crossterm, Goto, Hide, Output, SetAttr, SetBg,
    SetFg, Show,
};
use std::io::Write;

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
                map_error(queue!(string, Goto(x, y)))?;
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
                let color = crossterm::Color::from(cell.style.fg);
                map_error(queue!(string, SetFg(color)))?;
                style.fg = cell.style.fg;
                inst += 1;
            }
            if cell.style.bg != style.bg {
                let color = crossterm::Color::from(cell.style.bg);
                map_error(queue!(string, SetBg(color)))?;
                style.bg = cell.style.bg;
                inst += 1;
            }

            string.push_str(&cell.symbol);
            inst += 1;
        }

        map_error(queue!(
            self.stdout,
            Output(string),
            SetFg(crossterm::Color::Reset),
            SetBg(crossterm::Color::Reset),
            SetAttr(crossterm::Attribute::Reset)
        ))
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        map_error(execute!(self.stdout, Hide))
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        map_error(execute!(self.stdout, Show))
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        let cursor = crossterm::cursor();
        cursor
            .pos()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        map_error(execute!(self.stdout, Goto(x, y)))
    }

    fn clear(&mut self) -> io::Result<()> {
        map_error(execute!(self.stdout, Clear(ClearType::All)))
    }

    fn size(&self) -> io::Result<Rect> {
        let terminal = terminal();
        let (width, height) = terminal
            .size()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        Ok(Rect::new(0, 0, width, height))
    }

    fn flush(&mut self) -> io::Result<()> {
        <CrosstermBackend<W> as Write>::flush(self)
    }
}

fn map_error(error: crossterm::Result<()>) -> io::Result<()> {
    error.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}

impl From<Color> for crossterm::Color {
    fn from(color: Color) -> Self {
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
            map_error(queue!(w, SetAttr(Attribute::NormalIntensity)))?;
        }
        if removed.contains(Modifier::UNDERLINED) {
            map_error(queue!(w, SetAttr(Attribute::NoUnderline)))?;
        }

        let added = self.to - self.from;
        if added.contains(Modifier::BOLD) {
            map_error(queue!(w, SetAttr(Attribute::Bold)))?;
        }
        if added.contains(Modifier::UNDERLINED) {
            map_error(queue!(w, SetAttr(Attribute::Underlined)))?;
        }
        Ok(())
    }
}
