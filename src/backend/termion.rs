extern crate termion;

use std::io;
use std::io::Write;

use super::Backend;
use buffer::Cell;
use layout::Rect;
use style::{Color, Modifier, Style};

pub struct TermionBackend<W>
where
    W: Write,
{
    stdout: W,
}

impl<W> TermionBackend<W>
where
    W: Write,
{
    pub fn new(stdout: W) -> TermionBackend<W> {
        TermionBackend { stdout }
    }
}

impl<W> Write for TermionBackend<W>
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

impl<W> Backend for TermionBackend<W>
where
    W: Write,
{
    /// Clears the entire screen and move the cursor to the top left of the screen
    fn clear(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", termion::clear::All)?;
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1))?;
        self.stdout.flush()
    }

    /// Hides cursor
    fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", termion::cursor::Hide)?;
        self.stdout.flush()
    }

    /// Shows cursor
    fn show_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", termion::cursor::Show)?;
        self.stdout.flush()
    }

    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let mut string = String::with_capacity(content.size_hint().0 * 3);
        let mut style = Style::default();
        let mut last_y = 0;
        let mut last_x = 0;
        let mut inst = 0;
        for (x, y, cell) in content {
            if y != last_y || x != last_x + 1 {
                string.push_str(&format!("{}", termion::cursor::Goto(x + 1, y + 1)));
                inst += 1;
            }
            last_x = x;
            last_y = y;
            if cell.style.modifier != style.modifier {
                string.push_str(&cell.style.modifier.termion_modifier());
                style.modifier = cell.style.modifier;
                if style.modifier == Modifier::Reset {
                    style.bg = Color::Reset;
                    style.fg = Color::Reset;
                }
                inst += 1;
            }
            if cell.style.fg != style.fg {
                string.push_str(&cell.style.fg.termion_fg());
                style.fg = cell.style.fg;
                inst += 1;
            }
            if cell.style.bg != style.bg {
                string.push_str(&cell.style.bg.termion_bg());
                style.bg = cell.style.bg;
                inst += 1;
            }
            string.push_str(&cell.symbol);
            inst += 1;
        }
        debug!("{} instructions outputed.", inst);
        write!(
            self.stdout,
            "{}{}{}{}",
            string,
            Color::Reset.termion_fg(),
            Color::Reset.termion_bg(),
            Modifier::Reset.termion_modifier()
        )
    }

    /// Return the size of the terminal
    fn size(&self) -> io::Result<Rect> {
        let terminal = try!(termion::terminal_size());
        Ok(Rect::new(0, 0, terminal.0, terminal.1))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

macro_rules! termion_fg {
    ($color:ident) => {
        format!("{}", termion::color::Fg(termion::color::$color))
    };
}

macro_rules! termion_fg_rgb {
    ($r:expr, $g:expr, $b:expr) => {
        format!("{}", termion::color::Fg(termion::color::Rgb($r, $g, $b)))
    };
}

macro_rules! termion_bg {
    ($color:ident) => {
        format!("{}", termion::color::Bg(termion::color::$color))
    };
}

macro_rules! termion_bg_rgb {
    ($r:expr, $g:expr, $b:expr) => {
        format!("{}", termion::color::Bg(termion::color::Rgb($r, $g, $b)))
    };
}

macro_rules! termion_modifier {
    ($style:ident) => {
        format!("{}", termion::style::$style)
    };
}

impl Color {
    pub fn termion_fg(self) -> String {
        match self {
            Color::Reset => termion_fg!(Reset),
            Color::Black => termion_fg!(Black),
            Color::Red => termion_fg!(Red),
            Color::Green => termion_fg!(Green),
            Color::Yellow => termion_fg!(Yellow),
            Color::Blue => termion_fg!(Blue),
            Color::Magenta => termion_fg!(Magenta),
            Color::Cyan => termion_fg!(Cyan),
            Color::Gray => termion_fg!(White),
            Color::DarkGray => termion_fg!(LightBlack),
            Color::LightRed => termion_fg!(LightRed),
            Color::LightGreen => termion_fg!(LightGreen),
            Color::LightBlue => termion_fg!(LightBlue),
            Color::LightYellow => termion_fg!(LightYellow),
            Color::LightMagenta => termion_fg!(LightMagenta),
            Color::LightCyan => termion_fg!(LightCyan),
            Color::White => termion_fg!(LightWhite),
            Color::Rgb(r, g, b) => termion_fg_rgb!(r, g, b),
        }
    }
    pub fn termion_bg(self) -> String {
        match self {
            Color::Reset => termion_bg!(Reset),
            Color::Black => termion_bg!(Black),
            Color::Red => termion_bg!(Red),
            Color::Green => termion_bg!(Green),
            Color::Yellow => termion_bg!(Yellow),
            Color::Blue => termion_bg!(Blue),
            Color::Magenta => termion_bg!(Magenta),
            Color::Cyan => termion_bg!(Cyan),
            Color::Gray => termion_bg!(White),
            Color::DarkGray => termion_bg!(LightBlack),
            Color::LightRed => termion_bg!(LightRed),
            Color::LightGreen => termion_bg!(LightGreen),
            Color::LightBlue => termion_bg!(LightBlue),
            Color::LightYellow => termion_bg!(LightYellow),
            Color::LightMagenta => termion_bg!(LightMagenta),
            Color::LightCyan => termion_bg!(LightCyan),
            Color::White => termion_bg!(LightWhite),
            Color::Rgb(r, g, b) => termion_bg_rgb!(r, g, b),
        }
    }
}

impl Modifier {
    pub fn termion_modifier(self) -> String {
        match self {
            Modifier::Blink => termion_modifier!(Blink),
            Modifier::Bold => termion_modifier!(Bold),
            Modifier::CrossedOut => termion_modifier!(CrossedOut),
            Modifier::Faint => termion_modifier!(Faint),
            Modifier::Framed => termion_modifier!(Framed),
            Modifier::Invert => termion_modifier!(Invert),
            Modifier::Italic => termion_modifier!(Italic),
            Modifier::NoBlink => termion_modifier!(NoBlink),
            Modifier::NoBold => termion_modifier!(NoBold),
            Modifier::NoCrossedOut => termion_modifier!(NoCrossedOut),
            Modifier::NoFaint => termion_modifier!(NoFaint),
            Modifier::NoInvert => termion_modifier!(NoInvert),
            Modifier::NoItalic => termion_modifier!(NoItalic),
            Modifier::NoUnderline => termion_modifier!(NoUnderline),
            Modifier::Reset => termion_modifier!(Reset),
            Modifier::Underline => termion_modifier!(Underline),
        }
    }
}
