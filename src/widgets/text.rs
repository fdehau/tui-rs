use std::cmp::min;

use widgets::{Widget, Block};
use buffer::Buffer;
use layout::Rect;
use style::Color;

pub struct Text<'a> {
    block: Option<Block<'a>>,
    fg: Color,
    bg: Color,
    text: &'a str,
    colors: &'a [(u16, u16, u16, Color, Color)],
}

impl<'a> Default for Text<'a> {
    fn default() -> Text<'a> {
        Text {
            block: None,
            fg: Color::Reset,
            bg: Color::Reset,
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
}

impl<'a> Widget<'a> for Text<'a> {
    fn buffer(&'a self, area: &Rect) -> Buffer<'a> {
        let (mut buf, text_area) = match self.block {
            Some(ref b) => (b.buffer(area), b.inner(*area)),
            None => (Buffer::empty(*area), *area),
        };
        let mut lines = self.text.lines().collect::<Vec<&str>>();
        let margin_x = text_area.x - area.x;
        let margin_y = text_area.y - area.y;
        let height = min(lines.len(), text_area.height as usize);
        for line in lines.iter_mut().take(height) {
            buf.set_string(margin_x, margin_y, line, self.fg, self.bg);
        }
        for &(x, y, width, fg, bg) in self.colors {
            for i in 0..width {
                buf.set_fg(x + i, y + margin_y, fg);
                buf.set_bg(x + i, y + margin_y, fg);
            }
        }
        buf
    }
}
