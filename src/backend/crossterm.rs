use std::{
    fmt,
    io::{self, Write},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    screen::AlternateScreen,
    style::{
        Attribute as CAttribute, Color as CColor, SetAttribute, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{self, Clear, ClearType},
    Output,
};

use crate::backend::Backend;
use crate::style::{Color, Modifier};
use crate::{buffer::Cell, layout::Rect, style};

pub struct CrosstermBackend<W: Write> {
    alternate_screen: Option<AlternateScreen>,
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
        alternate_screen: AlternateScreen,
    ) -> Result<CrosstermBackend<W>, io::Error> {
        Ok(CrosstermBackend {
            alternate_screen: Some(alternate_screen),
            stdout,
        })
    }

    pub fn alternate_screen(&self) -> Option<&AlternateScreen> {
        match &self.alternate_screen {
            Some(alt_screen) => Some(&alt_screen),
            None => None,
        }
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
        use fmt::Write;

        let mut string = String::with_capacity(content.size_hint().0 * 3);
        let mut style = style::Style::default();
        let mut last_y = 0;
        let mut last_x = 0;
        let mut inst = 0;

        for (x, y, cell) in content {
            if y != last_y || x != last_x + 1 || inst == 0 {
                map_error(queue!(string, MoveTo(x, y)))?;
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
                let color = CColor::from(cell.style.fg);
                map_error(queue!(string, SetForegroundColor(color)))?;
                style.fg = cell.style.fg;
                inst += 1;
            }
            if cell.style.bg != style.bg {
                let color = CColor::from(cell.style.bg);
                map_error(queue!(string, SetBackgroundColor(color)))?;
                style.bg = cell.style.bg;
                inst += 1;
            }

            string.push_str(&cell.symbol);
            inst += 1;
        }

        map_error(queue!(
            self.stdout,
            Output(string),
            SetForegroundColor(CColor::Reset),
            SetBackgroundColor(CColor::Reset),
            SetAttribute(CAttribute::Reset)
        ))
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        map_error(execute!(self.stdout, Hide))
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        map_error(execute!(self.stdout, Show))
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        crossterm::cursor::position()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        map_error(execute!(self.stdout, MoveTo(x, y)))
    }

    fn clear(&mut self) -> io::Result<()> {
        map_error(execute!(self.stdout, Clear(ClearType::All)))
    }

    fn size(&self) -> io::Result<Rect> {
        let (width, height) =
            terminal::size().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        Ok(Rect::new(0, 0, width, height))
    }

    fn flush(&mut self) -> io::Result<()> {
        <CrosstermBackend<W> as Write>::flush(self)
    }
}

fn map_error(error: crossterm::Result<()>) -> io::Result<()> {
    error.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}

impl From<Color> for CColor {
    fn from(color: Color) -> Self {
        match color {
            Color::Reset => CColor::Reset,
            Color::Black => CColor::Black,
            Color::Red => CColor::DarkRed,
            Color::Green => CColor::DarkGreen,
            Color::Yellow => CColor::DarkYellow,
            Color::Blue => CColor::DarkBlue,
            Color::Magenta => CColor::DarkMagenta,
            Color::Cyan => CColor::DarkCyan,
            Color::Gray => CColor::Grey,
            Color::DarkGray => CColor::DarkGrey,
            Color::LightRed => CColor::Red,
            Color::LightGreen => CColor::Green,
            Color::LightBlue => CColor::Blue,
            Color::LightYellow => CColor::Yellow,
            Color::LightMagenta => CColor::Magenta,
            Color::LightCyan => CColor::Cyan,
            Color::White => CColor::White,
            Color::Indexed(i) => CColor::AnsiValue(i),
            Color::Rgb(r, g, b) => CColor::Rgb { r, g, b },
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
            queue!(w, SetAttribute(CAttribute::NoInverse))?;
        }
        if removed.contains(Modifier::BOLD) {
            queue!(w, SetAttribute(CAttribute::NormalIntensity))?;
            if self.to.contains(Modifier::DIM) {
                queue!(w, SetAttribute(CAttribute::Dim))?;
            }
        }
        if removed.contains(Modifier::ITALIC) {
            queue!(w, SetAttribute(CAttribute::NoItalic))?;
        }
        if removed.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttribute(CAttribute::NoUnderline))?;
        }
        if removed.contains(Modifier::DIM) {
            queue!(w, SetAttribute(CAttribute::NormalIntensity))?;
        }
        if removed.contains(Modifier::CROSSED_OUT) {
            queue!(w, SetAttribute(CAttribute::NotCrossedOut))?;
        }
        if removed.contains(Modifier::SLOW_BLINK) || removed.contains(Modifier::RAPID_BLINK) {
            queue!(w, SetAttribute(CAttribute::NoBlink))?;
        }

        let added = self.to - self.from;
        if added.contains(Modifier::REVERSED) {
            queue!(w, SetAttribute(CAttribute::Reverse))?;
        }
        if added.contains(Modifier::BOLD) {
            queue!(w, SetAttribute(CAttribute::Bold))?;
        }
        if added.contains(Modifier::ITALIC) {
            queue!(w, SetAttribute(CAttribute::Italic))?;
        }
        if added.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttribute(CAttribute::Underlined))?;
        }
        if added.contains(Modifier::DIM) {
            queue!(w, SetAttribute(CAttribute::Dim))?;
        }
        if added.contains(Modifier::CROSSED_OUT) {
            queue!(w, SetAttribute(CAttribute::CrossedOut))?;
        }
        if added.contains(Modifier::SLOW_BLINK) {
            queue!(w, SetAttribute(CAttribute::SlowBlink))?;
        }
        if added.contains(Modifier::RAPID_BLINK) {
            queue!(w, SetAttribute(CAttribute::RapidBlink))?;
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
        let removed = self.from - self.to;
        if removed.contains(Modifier::BOLD) {
            map_error(queue!(w, SetAttribute(CAttribute::NormalIntensity)))?;
        }
        if removed.contains(Modifier::UNDERLINED) {
            map_error(queue!(w, SetAttribute(CAttribute::NoUnderline)))?;
        }

        let added = self.to - self.from;
        if added.contains(Modifier::BOLD) {
            map_error(queue!(w, SetAttribute(CAttribute::Bold)))?;
        }
        if added.contains(Modifier::UNDERLINED) {
            map_error(queue!(w, SetAttribute(CAttribute::Underlined)))?;
        }
        Ok(())
    }
}
