use crate::{
    backend::Backend,
    buffer::Cell,
    layout::Rect,
    style::{Color, Modifier},
};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{
        Attribute as CAttribute, Color as CColor, Print, SetAttribute, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to flush")]
    Flush(#[source] std::io::Error),

    #[error("failed to get terminal size")]
    GetTerminalSize(#[source] crossterm::ErrorKind),

    #[error("failed to draw background")]
    DrawBackground(#[source] crossterm::ErrorKind),

    #[error("failed to draw foreground")]
    DrawForeground(#[source] crossterm::ErrorKind),

    #[error("failed to move cursor to position in buffer: {1:?}")]
    MoveCursor(#[source] crossterm::ErrorKind, (u16, u16)),

    #[error("failed to get cursor position")]
    GetCursosPos(#[source] crossterm::ErrorKind),

    #[error("failed to show cursor")]
    ShowCursor(#[source] crossterm::ErrorKind),

    #[error("failed to hide cursor")]
    HideCursor(#[source] crossterm::ErrorKind),

    #[error("failed to clear terminal")]
    Clear(#[source] crossterm::ErrorKind),

    #[error("failed to reset terminal")]
    Reset(#[source] crossterm::ErrorKind),

    #[error("failed to draw symbol")]
    DrawSymbol(#[source] crossterm::ErrorKind),

    #[error("failed to set attribute: {1:?}")]
    SetAttribute(#[source] crossterm::ErrorKind, CAttribute),
}

pub struct CrosstermBackend<W: Write> {
    buffer: W,
}

impl<W> CrosstermBackend<W>
where
    W: Write,
{
    pub fn new(buffer: W) -> Self {
        Self { buffer }
    }
}

impl<W> Write for CrosstermBackend<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

impl<W> Backend for CrosstermBackend<W>
where
    W: Write,
{
    type Error = Error;
    fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        let mut modifier = Modifier::empty();
        let mut last_pos: Option<(u16, u16)> = None;
        for (x, y, cell) in content {
            // Move the cursor if the previous location was not (x - 1, y)
            if !matches!(last_pos, Some(p) if x == p.0 + 1 && y == p.1) {
                queue!(self.buffer, MoveTo(x, y)).map_err(|e| Error::MoveCursor(e, (x, y)))?;
            }
            last_pos = Some((x, y));
            if cell.modifier != modifier {
                let diff = ModifierDiff {
                    from: modifier,
                    to: cell.modifier,
                };
                diff.queue(&mut self.buffer)?;
                modifier = cell.modifier;
            }
            if cell.fg != fg {
                let color = CColor::from(cell.fg);
                queue!(self.buffer, SetForegroundColor(color)).map_err(Error::DrawForeground)?;
                fg = cell.fg;
            }
            if cell.bg != bg {
                let color = CColor::from(cell.bg);
                queue!(self.buffer, SetBackgroundColor(color)).map_err(Error::DrawBackground)?;
                bg = cell.bg;
            }

            queue!(self.buffer, Print(&cell.symbol)).map_err(Error::DrawSymbol)?;
        }

        queue!(
            self.buffer,
            SetForegroundColor(CColor::Reset),
            SetBackgroundColor(CColor::Reset),
            SetAttribute(CAttribute::Reset)
        )
        .map_err(Error::Reset)
    }

    fn hide_cursor(&mut self) -> Result<(), Self::Error> {
        execute!(self.buffer, Hide).map_err(Error::HideCursor)
    }

    fn show_cursor(&mut self) -> Result<(), Self::Error> {
        execute!(self.buffer, Show).map_err(Error::ShowCursor)
    }

    fn get_cursor(&mut self) -> Result<(u16, u16), Self::Error> {
        crossterm::cursor::position().map_err(Error::GetCursosPos)
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), Self::Error> {
        execute!(self.buffer, MoveTo(x, y)).map_err(|e| Error::MoveCursor(e, (x, y)))
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        execute!(self.buffer, Clear(ClearType::All)).map_err(Error::Clear)
    }

    fn size(&self) -> Result<Rect, Self::Error> {
        let (width, height) = terminal::size().map_err(Error::GetTerminalSize)?;

        Ok(Rect::new(0, 0, width, height))
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.buffer.flush().map_err(Error::Flush)
    }
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

impl ModifierDiff {
    fn queue<W>(&self, mut w: W) -> Result<(), Error>
    where
        W: io::Write,
    {
        let removed = self.from - self.to;
        if removed.contains(Modifier::REVERSED) {
            queue!(w, SetAttribute(CAttribute::NoReverse))
                .map_err(|e| Error::SetAttribute(e, CAttribute::NoReverse))?;
        }
        if removed.contains(Modifier::BOLD) {
            queue!(w, SetAttribute(CAttribute::NormalIntensity))
                .map_err(|e| Error::SetAttribute(e, CAttribute::NormalIntensity))?;
            if self.to.contains(Modifier::DIM) {
                queue!(w, SetAttribute(CAttribute::Dim))
                    .map_err(|e| Error::SetAttribute(e, CAttribute::Dim))?;
            }
        }
        if removed.contains(Modifier::ITALIC) {
            queue!(w, SetAttribute(CAttribute::NoItalic))
                .map_err(|e| Error::SetAttribute(e, CAttribute::NoItalic))?;
        }
        if removed.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttribute(CAttribute::NoUnderline))
                .map_err(|e| Error::SetAttribute(e, CAttribute::NoUnderline))?;
        }
        if removed.contains(Modifier::DIM) {
            queue!(w, SetAttribute(CAttribute::NormalIntensity))
                .map_err(|e| Error::SetAttribute(e, CAttribute::NormalIntensity))?;
        }
        if removed.contains(Modifier::CROSSED_OUT) {
            queue!(w, SetAttribute(CAttribute::NotCrossedOut))
                .map_err(|e| Error::SetAttribute(e, CAttribute::NotCrossedOut))?;
        }
        if removed.contains(Modifier::SLOW_BLINK) || removed.contains(Modifier::RAPID_BLINK) {
            queue!(w, SetAttribute(CAttribute::NoBlink))
                .map_err(|e| Error::SetAttribute(e, CAttribute::NoBlink))?;
        }

        let added = self.to - self.from;
        if added.contains(Modifier::REVERSED) {
            queue!(w, SetAttribute(CAttribute::Reverse))
                .map_err(|e| Error::SetAttribute(e, CAttribute::Reverse))?;
        }
        if added.contains(Modifier::BOLD) {
            queue!(w, SetAttribute(CAttribute::Bold))
                .map_err(|e| Error::SetAttribute(e, CAttribute::Bold))?;
        }
        if added.contains(Modifier::ITALIC) {
            queue!(w, SetAttribute(CAttribute::Italic))
                .map_err(|e| Error::SetAttribute(e, CAttribute::Italic))?;
        }
        if added.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttribute(CAttribute::Underlined))
                .map_err(|e| Error::SetAttribute(e, CAttribute::Underlined))?;
        }
        if added.contains(Modifier::DIM) {
            queue!(w, SetAttribute(CAttribute::Dim))
                .map_err(|e| Error::SetAttribute(e, CAttribute::Dim))?;
        }
        if added.contains(Modifier::CROSSED_OUT) {
            queue!(w, SetAttribute(CAttribute::CrossedOut))
                .map_err(|e| Error::SetAttribute(e, CAttribute::CrossedOut))?;
        }
        if added.contains(Modifier::SLOW_BLINK) {
            queue!(w, SetAttribute(CAttribute::SlowBlink))
                .map_err(|e| Error::SetAttribute(e, CAttribute::SlowBlink))?;
        }
        if added.contains(Modifier::RAPID_BLINK) {
            queue!(w, SetAttribute(CAttribute::RapidBlink))
                .map_err(|e| Error::SetAttribute(e, CAttribute::RapidBlink))?;
        }

        Ok(())
    }
}
