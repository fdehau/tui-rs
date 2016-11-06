use termion;
use rustbox;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Modifier {
    Blink,
    Bold,
    CrossedOut,
    Faint,
    Framed,
    Invert,
    Italic,
    NoBlink,
    NoBold,
    NoCrossedOut,
    NoFaint,
    NoInvert,
    NoItalic,
    NoUnderline,
    Reset,
    Underline,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub modifier: Modifier,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::Reset,
        }
    }
}

impl Style {
    pub fn reset(&mut self) {
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        self.modifier = Modifier::Reset;
    }

    pub fn fg(mut self, color: Color) -> Style {
        self.fg = color;
        self
    }
    pub fn bg(mut self, color: Color) -> Style {
        self.bg = color;
        self
    }
    pub fn modifier(mut self, modifier: Modifier) -> Style {
        self.modifier = modifier;
        self
    }
}

macro_rules! termion_fg {
    ($color:ident) => (format!("{}", termion::color::Fg(termion::color::$color)));
}

macro_rules! termion_fg_rgb {
    ($r:expr, $g:expr, $b:expr) => (format!("{}", termion::color::Fg(termion::color::Rgb($r, $g, $b))));
}

macro_rules! termion_bg {
    ($color:ident) => (format!("{}", termion::color::Bg(termion::color::$color)));
}

macro_rules! termion_bg_rgb {
    ($r:expr, $g:expr, $b:expr) => (format!("{}", termion::color::Bg(termion::color::Rgb($r, $g, $b))));
}

macro_rules! termion_modifier {
    ($style:ident) => (format!("{}", termion::style::$style));
}

impl Color {
    pub fn termion_fg(&self) -> String {
        match *self {
            Color::Reset => termion_fg!(Reset),
            Color::Black => termion_fg!(Black),
            Color::Red => termion_fg!(Red),
            Color::Green => termion_fg!(Green),
            Color::Yellow => termion_fg!(Yellow),
            Color::Magenta => termion_fg!(Magenta),
            Color::Cyan => termion_fg!(Cyan),
            Color::Gray => termion_fg_rgb!(146, 131, 116),
            Color::DarkGray => termion_fg_rgb!(80, 73, 69),
            Color::LightRed => termion_fg!(LightRed),
            Color::LightGreen => termion_fg!(LightGreen),
            Color::LightYellow => termion_fg!(LightYellow),
            Color::LightMagenta => termion_fg!(LightMagenta),
            Color::LightCyan => termion_fg!(LightCyan),
            Color::White => termion_fg!(White),
            Color::Rgb(r, g, b) => termion_fg_rgb!(r, g, b),
        }
    }
    pub fn termion_bg(&self) -> String {
        match *self {
            Color::Reset => termion_bg!(Reset),
            Color::Black => termion_bg!(Black),
            Color::Red => termion_bg!(Red),
            Color::Green => termion_bg!(Green),
            Color::Yellow => termion_bg!(Yellow),
            Color::Magenta => termion_bg!(Magenta),
            Color::Cyan => termion_bg!(Cyan),
            Color::Gray => termion_bg_rgb!(146, 131, 116),
            Color::DarkGray => termion_bg_rgb!(80, 73, 69),
            Color::LightRed => termion_bg!(LightRed),
            Color::LightGreen => termion_bg!(LightGreen),
            Color::LightYellow => termion_bg!(LightYellow),
            Color::LightMagenta => termion_bg!(LightMagenta),
            Color::LightCyan => termion_bg!(LightCyan),
            Color::White => termion_bg!(White),
            Color::Rgb(r, g, b) => termion_bg_rgb!(r, g, b),
        }
    }
}

fn rgb_to_byte(r: u8, g: u8, b: u8) -> u16 {
    (((r & 255 & 0xC0) + (g & 255 & 0xE0) >> 2 + (b & 0xE0) >> 5) & 0xFF) as u16
}

impl Into<rustbox::Color> for Color {
    fn into(self) -> rustbox::Color {
        match self {
            Color::Reset => rustbox::Color::Default,
            Color::Black => rustbox::Color::Black,
            Color::Red => rustbox::Color::Red,
            Color::Green => rustbox::Color::Green,
            Color::Yellow => rustbox::Color::Yellow,
            Color::Magenta => rustbox::Color::Magenta,
            Color::Cyan => rustbox::Color::Cyan,
            Color::Gray => rustbox::Color::Black,
            Color::DarkGray => rustbox::Color::Black,
            Color::LightRed => rustbox::Color::Red,
            Color::LightGreen => rustbox::Color::Green,
            Color::LightYellow => rustbox::Color::Yellow,
            Color::LightMagenta => rustbox::Color::Magenta,
            Color::LightCyan => rustbox::Color::Cyan,
            Color::White => rustbox::Color::White,
            Color::Rgb(r, g, b) => rustbox::Color::Byte(rgb_to_byte(r, g, b)),
        }
    }
}

impl Modifier {
    pub fn termion_modifier(&self) -> String {
        match *self {
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

impl Into<rustbox::Style> for Modifier {
    fn into(self) -> rustbox::Style {
        match self {
            Modifier::Reset => rustbox::RB_NORMAL,
            Modifier::Bold => rustbox::RB_BOLD,
            Modifier::Underline => rustbox::RB_UNDERLINE,
            Modifier::Invert => rustbox::RB_REVERSE,
            _ => rustbox::RB_NORMAL,
        }
    }
}
