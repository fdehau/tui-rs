use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use widgets::{Widget, Block};
use buffer::Buffer;
use layout::Rect;
use style::{Style, Color, Modifier};

/// A widget to display some text. You can specify colors using commands embedded in
/// the text such as "{[color] [text]}".
///
/// # Examples
///
/// ```
/// # extern crate tui;
/// # use tui::widgets::{Block, border, Paragraph};
/// # use tui::style::Color;
/// # fn main() {
/// Paragraph::default()
///     .block(Block::default().title("Paragraph").borders(border::ALL))
///     .color(Color::White)
///     .background_color(Color::Black)
///     .wrap(true)
///     .text("First line\nSecond line\n{red Colored text}.");
/// # }
/// ```
pub struct Paragraph<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Widget style
    style: Style,
    /// Wrap the text or not
    wrapping: bool,
    /// The text to display
    text: &'a str,
}

impl<'a> Default for Paragraph<'a> {
    fn default() -> Paragraph<'a> {
        Paragraph {
            block: None,
            style: Default::default(),
            wrapping: false,
            text: "",
        }
    }
}

impl<'a> Paragraph<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut Paragraph<'a> {
        self.block = Some(block);
        self
    }

    pub fn text(&mut self, text: &'a str) -> &mut Paragraph<'a> {
        self.text = text;
        self
    }

    pub fn style(&mut self, style: Style) -> &mut Paragraph<'a> {
        self.style = style;
        self
    }

    pub fn wrap(&mut self, flag: bool) -> &mut Paragraph<'a> {
        self.wrapping = flag;
        self
    }
}

struct Parser<'a> {
    text: Vec<&'a str>,
    mark: bool,
    cmd_string: String,
    style: Style,
    base_style: Style,
    escaping: bool,
    styling: bool,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str, base_style: Style) -> Parser<'a> {
        Parser {
            text: UnicodeSegmentation::graphemes(text, true).rev().collect::<Vec<&str>>(),
            mark: false,
            cmd_string: String::from(""),
            style: base_style,
            base_style: base_style,
            escaping: false,
            styling: false,
        }
    }

    fn update_style(&mut self) {
        for cmd in self.cmd_string.split(';') {
            let args = cmd.split('=').collect::<Vec<&str>>();
            if let Some(first) = args.get(0) {
                match *first {
                    "fg" => {
                        if let Some(snd) = args.get(1) {
                            self.style.fg = Parser::str_to_color(snd);
                        }
                    }
                    "bg" => {
                        if let Some(snd) = args.get(1) {
                            self.style.bg = Parser::str_to_color(snd);
                        }
                    }
                    "mod" => {
                        if let Some(snd) = args.get(1) {
                            self.style.modifier = Parser::str_to_modifier(snd);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn str_to_color(string: &str) -> Color {
        match string {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "gray" => Color::Gray,
            "dark_gray" => Color::DarkGray,
            "light_red" => Color::LightRed,
            "light_green" => Color::LightGreen,
            "light_yellow" => Color::LightYellow,
            "light_magenta" => Color::LightMagenta,
            "light_cyan" => Color::LightCyan,
            "white" => Color::White,
            _ => Color::Reset,
        }
    }

    fn str_to_modifier(string: &str) -> Modifier {
        match string {
            "bold" => Modifier::Bold,
            "italic" => Modifier::Italic,
            "underline" => Modifier::Underline,
            "invert" => Modifier::Invert,
            _ => Modifier::Reset,
        }
    }

    fn reset(&mut self) {
        self.styling = false;
        self.mark = false;
        self.style = self.base_style;
        self.cmd_string.clear();
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = (&'a str, Style);
    fn next(&mut self) -> Option<Self::Item> {
        match self.text.pop() {
            Some(s) => {
                if s == "\\" {
                    if self.escaping {
                        Some((s, self.style))
                    } else {
                        self.escaping = true;
                        self.next()
                    }
                } else if s == "{" {
                    if self.escaping {
                        self.escaping = false;
                        Some((s, self.style))
                    } else {
                        self.style = self.base_style;
                        self.mark = true;
                        self.next()
                    }
                } else if s == "}" && self.mark {
                    self.reset();
                    self.next()
                } else if s == " " && self.mark {
                    self.styling = true;
                    self.update_style();
                    self.next()
                } else if self.mark && !self.styling {
                    self.cmd_string.push_str(s);
                    self.next()
                } else {
                    Some((s, self.style))
                }
            }
            None => None,
        }
    }
}

impl<'a> Widget for Paragraph<'a> {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {
        let text_area = match self.block {
            Some(ref b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        if text_area.height < 1 {
            return;
        }

        self.background(&text_area, buf, self.style.bg);

        let mut x = 0;
        let mut y = 0;
        for (string, style) in Parser::new(self.text, self.style) {
            if string == "\n" {
                x = 0;
                y += 1;
                continue;
            }
            if x >= text_area.width {
                if self.wrapping {
                    x = 0;
                    y += 1;
                }
                continue;
            }

            if y > text_area.height - 1 {
                break;
            }

            buf.get_mut(text_area.left() + x, text_area.top() + y)
                .set_symbol(string)
                .set_style(style);
            x += string.width() as u16;
        }
    }
}
