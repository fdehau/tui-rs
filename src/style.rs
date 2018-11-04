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
}

impl Color {
    /// Returns a short code associated with the color, used for debug purpose
    /// only
    pub(crate) fn code(&self) -> &str {
        match self {
            Color::Reset => "X",
            Color::Black => "b",
            Color::Red => "r",
            Color::Green => "c",
            Color::Yellow => "y",
            Color::Blue => "b",
            Color::Magenta => "m",
            Color::Cyan => "c",
            Color::Gray => "g",
            Color::DarkGray => "G",
            Color::LightRed => "R",
            Color::LightGreen => "G",
            Color::LightYellow => "Y",
            Color::LightBlue => "B",
            Color::LightMagenta => "M",
            Color::LightCyan => "C",
            Color::White => "w",
            Color::Rgb(_, _, _) => "o",
        }
    }
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

impl Modifier {
    /// Returns a short code associated with the color, used for debug purpose
    /// only
    pub(crate) fn code(&self) -> &str {
        match self {
            Modifier::Blink => "bl",
            Modifier::Bold => "bo",
            Modifier::CrossedOut => "cr",
            Modifier::Faint => "fa",
            Modifier::Framed => "fr",
            Modifier::Invert => "in",
            Modifier::Italic => "it",
            Modifier::NoBlink => "BL",
            Modifier::NoBold => "BO",
            Modifier::NoCrossedOut => "CR",
            Modifier::NoFaint => "FA",
            Modifier::NoInvert => "IN",
            Modifier::NoItalic => "IT",
            Modifier::NoUnderline => "UN",
            Modifier::Reset => "re",
            Modifier::Underline => "un",
        }
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
