use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use layout::Rect;
use style::{Color, Modifier, Style};
use widgets::{Block, Widget};

/// A widget to display some text. You can specify colors using commands embedded in
/// the text such as "{[color] [text]}".
///
/// # Examples
///
/// ```
/// # extern crate tui;
/// # use tui::widgets::{Block, Borders, Paragraph};
/// # use tui::style::{Style, Color};
/// # fn main() {
/// Paragraph::default()
///     .block(Block::default().title("Paragraph").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White).bg(Color::Black))
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
    /// Should we parse the text for embedded commands
    raw: bool,
    /// Scroll
    scroll: u16,
}

impl<'a> Default for Paragraph<'a> {
    fn default() -> Paragraph<'a> {
        Paragraph {
            block: None,
            style: Default::default(),
            wrapping: false,
            raw: false,
            text: "",
            scroll: 0,
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

    pub fn raw(&mut self, flag: bool) -> &mut Paragraph<'a> {
        self.raw = flag;
        self
    }

    pub fn scroll(&mut self, offset: u16) -> &mut Paragraph<'a> {
        self.scroll = offset;
        self
    }
}

struct Parser<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    text: T,
    mark: bool,
    cmd_string: String,
    style: Style,
    base_style: Style,
    escaping: bool,
    styling: bool,
}

impl<'a, T> Parser<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    fn new(text: T, base_style: Style) -> Parser<'a, T> {
        Parser {
            text: text,
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
                    "fg" => if let Some(snd) = args.get(1) {
                        self.style.fg = Parser::<T>::str_to_color(snd);
                    },
                    "bg" => if let Some(snd) = args.get(1) {
                        self.style.bg = Parser::<T>::str_to_color(snd);
                    },
                    "mod" => if let Some(snd) = args.get(1) {
                        self.style.modifier = Parser::<T>::str_to_modifier(snd);
                    },
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
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "gray" => Color::Gray,
            "dark_gray" => Color::DarkGray,
            "light_red" => Color::LightRed,
            "light_green" => Color::LightGreen,
            "light_blue" => Color::LightBlue,
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
            "crossed_out" => Modifier::CrossedOut,
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

impl<'a, T> Iterator for Parser<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    type Item = (&'a str, Style);
    fn next(&mut self) -> Option<Self::Item> {
        match self.text.next() {
            Some(s) => if s == "\\" {
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
                } else if self.mark {
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
                if self.styling {
                    Some((s, self.style))
                } else {
                    self.styling = true;
                    self.update_style();
                    self.next()
                }
            } else if self.mark && !self.styling {
                self.cmd_string.push_str(s);
                self.next()
            } else {
                Some((s, self.style))
            },
            None => None,
        }
    }
}

impl<'a> Widget for Paragraph<'a> {
    fn draw(&mut self, area: &Rect, buf: &mut Buffer) {
        let text_area = match self.block {
            Some(ref mut b) => {
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
        let graphemes = UnicodeSegmentation::graphemes(self.text, true);
        let styled: Box<Iterator<Item = (&str, Style)>> = if self.raw {
            Box::new(graphemes.map(|g| (g, self.style)))
        } else {
            Box::new(Parser::new(graphemes, self.style))
        };

        let mut remove_leading_whitespaces = false;
        for (string, style) in styled {
            if string == "\n" {
                x = 0;
                y += 1;
                continue;
            }
            if x >= text_area.width {
                if self.wrapping {
                    x = 0;
                    y += 1;
                    remove_leading_whitespaces = true
                }
            }

            if remove_leading_whitespaces && string == " " {
                continue;
            }
            remove_leading_whitespaces = false;

            if y > text_area.height + self.scroll - 1 {
                break;
            }

            if y < self.scroll {
                continue;
            }

            buf.get_mut(text_area.left() + x, text_area.top() + y - self.scroll)
                .set_symbol(string)
                .set_style(style);
            x += string.width() as u16;
        }
    }
}
