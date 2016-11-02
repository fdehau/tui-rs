use std::cmp::min;

use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use widgets::{Widget, Block};
use layout::Rect;
use style::Color;

pub struct List<'a> {
    block: Option<Block<'a>>,
    items: &'a [&'a str],
    selected: usize,
    color: Color,
    background_color: Color,
    highlight_color: Color,
    highlight_symbol: Option<&'a str>,
}

impl<'a> Default for List<'a> {
    fn default() -> List<'a> {
        List {
            block: None,
            items: &[],
            selected: 0,
            color: Color::Reset,
            background_color: Color::Reset,
            highlight_color: Color::Reset,
            highlight_symbol: None,
        }
    }
}

impl<'a> List<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut List<'a> {
        self.block = Some(block);
        self
    }

    pub fn items(&'a mut self, items: &'a [&'a str]) -> &mut List<'a> {
        self.items = items;
        self
    }

    pub fn color(&'a mut self, color: Color) -> &mut List<'a> {
        self.color = color;
        self
    }

    pub fn background_color(&'a mut self, color: Color) -> &mut List<'a> {
        self.background_color = color;
        self
    }

    pub fn highlight_symbol(&'a mut self, highlight_symbol: &'a str) -> &mut List<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_color(&'a mut self, highlight_color: Color) -> &mut List<'a> {
        self.highlight_color = highlight_color;
        self
    }

    pub fn select(&'a mut self, index: usize) -> &'a mut List<'a> {
        self.selected = index;
        self
    }
}

impl<'a> Widget for List<'a> {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {

        let list_area = match self.block {
            Some(ref b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        if self.background_color != Color::Reset {
            self.background(&list_area, buf, self.background_color);
        }

        let list_length = self.items.len();
        let list_height = list_area.height as usize;
        let bound = min(list_height, list_length);
        let offset = if self.selected >= list_height {
            self.selected - list_height + 1
        } else {
            0
        };

        let x = match self.highlight_symbol {
            Some(s) => (s.width() + 1) as u16 + list_area.left(),
            None => list_area.left(),
        };

        if x < list_area.right() {
            let width = (list_area.right() - x) as usize;
            for i in 0..bound {
                let index = i + offset;
                let item = self.items[index];
                let color = if index == self.selected {
                    self.highlight_color
                } else {
                    self.color
                };
                buf.set_stringn(x,
                                list_area.top() + i as u16,
                                item,
                                width,
                                color,
                                self.background_color);
            }

            if let Some(s) = self.highlight_symbol {
                buf.set_string(list_area.left(),
                               list_area.top() + (self.selected - offset) as u16,
                               s,
                               self.highlight_color,
                               self.background_color);
            }
        }
    }
}
