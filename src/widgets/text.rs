use std::cmp::min;

use widgets::{Widget, WidgetType, Block};
use buffer::Buffer;
use layout::Rect;
use style::Color;

#[derive(Hash)]
pub struct Text<'a> {
    block: Option<Block<'a>>,
    fg: Color,
    bg: Color,
    text: &'a str,
}

impl<'a> Default for Text<'a> {
    fn default() -> Text<'a> {
        Text {
            block: None,
            fg: Color::White,
            bg: Color::Black,
            text: "",
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
}

impl<'a> Widget for Text<'a> {
    fn buffer(&self, area: &Rect) -> Buffer {
        let (mut buf, text_area) = match self.block {
            Some(b) => (b.buffer(area), b.inner(*area)),
            None => (Buffer::empty(*area), *area),
        };
        let mut lines = self.text.lines().map(String::from).collect::<Vec<String>>();
        let margin_x = text_area.x - area.x;
        let margin_y = text_area.y - area.y;
        let height = min(lines.len(), text_area.height as usize);
        let width = text_area.width as usize;
        for line in lines.iter_mut().take(height) {
            line.truncate(width);
            buf.set_string(margin_x, margin_y, line, self.fg, self.bg);
        }
        buf
    }
    fn widget_type(&self) -> WidgetType {
        WidgetType::Text
    }
}
