use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

impl Color {
    /// Returns a short code associated with the color, used for debug purpose
    /// only
    pub(crate) fn code(self) -> &'static str {
        match self {
            Color::Reset => "X",
            Color::Black => "b",
            Color::Red => "r",
            Color::Green => "c",
            Color::Yellow => "y",
            Color::Blue => "b",
            Color::Magenta => "m",
            Color::Cyan => "c",
            Color::Gray => "w",
            Color::DarkGray => "B",
            Color::LightRed => "R",
            Color::LightGreen => "G",
            Color::LightYellow => "Y",
            Color::LightBlue => "B",
            Color::LightMagenta => "M",
            Color::LightCyan => "C",
            Color::White => "W",
            Color::Indexed(_) => "i",
            Color::Rgb(_, _, _) => "o",
        }
    }
}

bitflags! {
    pub struct Modifier: u16 {
        const BOLD              = 0b0000_0000_0001;
        const DIM               = 0b0000_0000_0010;
        const ITALIC            = 0b0000_0000_0100;
        const UNDERLINED        = 0b0000_0000_1000;
        const SLOW_BLINK        = 0b0000_0001_0000;
        const RAPID_BLINK       = 0b0000_0010_0000;
        const REVERSED          = 0b0000_0100_0000;
        const HIDDEN            = 0b0000_1000_0000;
        const CROSSED_OUT       = 0b0001_0000_0000;
    }
}

impl Modifier {
    /// Returns a short code associated with the color, used for debug purpose
    /// only
    pub(crate) fn code(self) -> String {
        use std::fmt::Write;

        let mut result = String::new();

        if self.contains(Modifier::BOLD) {
            write!(result, "BO").unwrap();
        }
        if self.contains(Modifier::DIM) {
            write!(result, "DI").unwrap();
        }
        if self.contains(Modifier::ITALIC) {
            write!(result, "IT").unwrap();
        }
        if self.contains(Modifier::UNDERLINED) {
            write!(result, "UN").unwrap();
        }
        if self.contains(Modifier::SLOW_BLINK) {
            write!(result, "SL").unwrap();
        }
        if self.contains(Modifier::RAPID_BLINK) {
            write!(result, "RA").unwrap();
        }
        if self.contains(Modifier::REVERSED) {
            write!(result, "RE").unwrap();
        }
        if self.contains(Modifier::HIDDEN) {
            write!(result, "HI").unwrap();
        }
        if self.contains(Modifier::CROSSED_OUT) {
            write!(result, "CR").unwrap();
        }

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub modifier: Modifier,
}

impl Default for Style {
    fn default() -> Style {
        Style::new()
    }
}

impl Style {
    pub const fn new() -> Self {
        Style {
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
        }
    }
    pub fn reset(&mut self) {
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        self.modifier = Modifier::empty();
    }

    pub const fn fg(mut self, color: Color) -> Style {
        self.fg = color;
        self
    }
    pub const fn bg(mut self, color: Color) -> Style {
        self.bg = color;
        self
    }
    pub const fn modifier(mut self, modifier: Modifier) -> Style {
        self.modifier = modifier;
        self
    }
}
