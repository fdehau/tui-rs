use unicode_segmentation::UnicodeSegmentation;

use widgets::{Widget, Block};
use buffer::Buffer;
use layout::Rect;
use style::Color;

pub struct Text<'a> {
    block: Option<Block<'a>>,
    fg: Color,
    bg: Color,
    wrapping: bool,
    text: &'a str,
    colors: &'a [(u16, u16, u16, Color, Color)],
}

impl<'a> Default for Text<'a> {
    fn default() -> Text<'a> {
        Text {
            block: None,
            fg: Color::Reset,
            bg: Color::Reset,
            wrapping: false,
            text: "",
            colors: &[],
        }
    }
}

impl<'a> Text<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut Text<'a> {
        self.block = Some(block);
        self
    }

    pub fn text(&mut self, text: &'a str) -> &mut Text<'a> {
        self.text = text;
        self
    }

    pub fn bg(&mut self, bg: Color) -> &mut Text<'a> {
        self.bg = bg;
        self
    }

    pub fn fg(&mut self, fg: Color) -> &mut Text<'a> {
        self.fg = fg;
        self
    }

    pub fn colors(&mut self, colors: &'a [(u16, u16, u16, Color, Color)]) -> &mut Text<'a> {
        self.colors = colors;
        self
    }

    pub fn wrap(&mut self, flag: bool) -> &mut Text<'a> {
        self.wrapping = flag;
        self
    }
}

struct Parser<'a> {
    text: Vec<&'a str>,
    mark: bool,
    color_string: String,
    color: Color,
    escaping: bool,
    coloring: bool,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Parser<'a> {
        Parser {
            text: UnicodeSegmentation::graphemes(text, true).rev().collect::<Vec<&str>>(),
            mark: false,
            color_string: String::from(""),
            color: Color::Reset,
            escaping: false,
            coloring: false,
        }
    }

    fn update_color(&mut self) {
        self.color = match &*self.color_string {
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

    fn reset(&mut self) {
        self.coloring = false;
        self.mark = false;
        self.color = Color::Reset;
        self.color_string = String::from("");
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = (&'a str, Color);
    fn next(&mut self) -> Option<(&'a str, Color)> {
        match self.text.pop() {
            Some(s) => {
                if s == "\\" {
                    if self.escaping {
                        Some((s, self.color))
                    } else {
                        self.escaping = true;
                        self.next()
                    }
                } else if s == "{" {
                    if self.escaping {
                        self.escaping = false;
                        Some((s, self.color))
                    } else {
                        self.color = Color::Reset;
                        self.mark = true;
                        self.next()
                    }
                } else if s == "}" && self.mark {
                    self.reset();
                    self.next()
                } else if s == " " && self.mark {
                    self.coloring = true;
                    self.update_color();
                    self.next()
                } else if self.mark && !self.coloring {
                    self.color_string.push_str(s);
                    self.next()
                } else {
                    Some((s, self.color))
                }
            }
            None => None,
        }
    }
}

impl<'a> Widget<'a> for Text<'a> {
    fn buffer(&'a self, area: &Rect) -> Buffer<'a> {
        let (mut buf, text_area) = match self.block {
            Some(ref b) => (b.buffer(area), b.inner(*area)),
            None => (Buffer::empty(*area), *area),
        };

        if text_area.height < 1 {
            return buf;
        }

        let margin_x = text_area.x - area.x;
        let margin_y = text_area.y - area.y;
        let mut x = 0;
        let mut y = 0;
        for (s, c) in Parser::new(self.text) {
            if s == "\n" {
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

            buf.update_cell(margin_x + x, margin_y + y, s, c, self.bg);
            x += 1;
        }

        for &(x, y, width, fg, bg) in self.colors {
            for i in 0..width {
                buf.set_fg(x + i, y + margin_y, fg);
                buf.set_bg(x + i, y + margin_y, bg);
            }
        }
        buf
    }
}
