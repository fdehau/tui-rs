use termion;

#[derive(Debug, Clone, Copy)]
pub enum Color {
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

impl Color {
    pub fn fg(&self) -> String {
        match *self {
            Color::Black => format!("{}", termion::color::Fg(termion::color::Black)),
            Color::Red => format!("{}", termion::color::Fg(termion::color::Red)),
            Color::Green => format!("{}", termion::color::Fg(termion::color::Green)),
            Color::Yellow => format!("{}", termion::color::Fg(termion::color::Yellow)),
            Color::Magenta => format!("{}", termion::color::Fg(termion::color::Magenta)),
            Color::Cyan => format!("{}", termion::color::Fg(termion::color::Cyan)),
            Color::Gray => format!("{}", termion::color::Fg(termion::color::Rgb(146, 131, 116))),
            Color::DarkGray => format!("{}", termion::color::Fg(termion::color::Rgb(80, 73, 69))),
            Color::LightRed => format!("{}", termion::color::Fg(termion::color::LightRed)),
            Color::LightGreen => format!("{}", termion::color::Fg(termion::color::LightGreen)),
            Color::LightYellow => format!("{}", termion::color::Fg(termion::color::LightYellow)),
            Color::LightMagenta => format!("{}", termion::color::Fg(termion::color::LightMagenta)),
            Color::LightCyan => format!("{}", termion::color::Fg(termion::color::LightCyan)),
            Color::White => format!("{}", termion::color::Fg(termion::color::White)),
            Color::Rgb(r, g, b) => format!("{}", termion::color::Fg(termion::color::Rgb(r, g, b))),
        }
    }
    pub fn bg(&self) -> String {
        match *self {
            Color::Black => format!("{}", termion::color::Bg(termion::color::Black)),
            Color::Red => format!("{}", termion::color::Bg(termion::color::Red)),
            Color::Green => format!("{}", termion::color::Bg(termion::color::Green)),
            Color::Yellow => format!("{}", termion::color::Bg(termion::color::Yellow)),
            Color::Magenta => format!("{}", termion::color::Bg(termion::color::Magenta)),
            Color::Cyan => format!("{}", termion::color::Bg(termion::color::Cyan)),
            Color::Gray => format!("{}", termion::color::Bg(termion::color::Rgb(146, 131, 116))),
            Color::DarkGray => format!("{}", termion::color::Bg(termion::color::Rgb(80, 73, 69))),
            Color::LightRed => format!("{}", termion::color::Bg(termion::color::LightRed)),
            Color::LightGreen => format!("{}", termion::color::Bg(termion::color::LightGreen)),
            Color::LightYellow => format!("{}", termion::color::Bg(termion::color::LightYellow)),
            Color::LightMagenta => format!("{}", termion::color::Bg(termion::color::LightMagenta)),
            Color::LightCyan => format!("{}", termion::color::Bg(termion::color::LightCyan)),
            Color::White => format!("{}", termion::color::Bg(termion::color::White)),
            Color::Rgb(r, g, b) => format!("{}", termion::color::Bg(termion::color::Rgb(r, g, b))),
        }
    }
}
