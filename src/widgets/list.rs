use std::cmp::min;

use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use widgets::{Widget, Block};
use layout::Rect;
use style::Color;

pub struct List<'a> {
    block: Option<Block<'a>>,
    selected: usize,
    selection_symbol: Option<&'a str>,
    selection_color: Color,
    color: Color,
    background_color: Color,
    items: &'a [&'a str],
}

impl<'a> Default for List<'a> {
    fn default() -> List<'a> {
        List {
            block: None,
            selected: 0,
            selection_symbol: None,
            selection_color: Color::Reset,
            color: Color::Reset,
            background_color: Color::Reset,
            items: &[],
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

    pub fn selection_symbol(&'a mut self, selection_symbol: &'a str) -> &mut List<'a> {
        self.selection_symbol = Some(selection_symbol);
        self
    }

    pub fn selection_color(&'a mut self, selection_color: Color) -> &mut List<'a> {
        self.selection_color = selection_color;
        self
    }

    pub fn select(&'a mut self, index: usize) -> &'a mut List<'a> {
        self.selected = index;
        self
    }
}

impl<'a> Widget for List<'a> {
    fn buffer(&self, area: &Rect, buf: &mut Buffer) {

        let list_area = match self.block {
            Some(ref b) => {
                b.buffer(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        let list_length = self.items.len();
        let list_height = list_area.height as usize;
        let bound = min(list_height, list_length);
        let offset = if self.selected > list_height {
            min(self.selected - list_height, list_length - list_height)
        } else {
            0
        };
        let x = match self.selection_symbol {
            Some(s) => (s.width() + 1) as u16 + list_area.left(),
            None => list_area.left(),
        };
        for i in 0..bound {
            let index = i + offset;
            let item = self.items[index];
            let color = if index == self.selected {
                self.selection_color
            } else {
                self.color
            };
            buf.set_string(x,
                           list_area.top() + i as u16,
                           item,
                           color,
                           self.background_color);
        }
        if let Some(s) = self.selection_symbol {
            buf.set_string(list_area.left(),
                           list_area.top() + (self.selected - offset) as u16,
                           s,
                           self.selection_color,
                           self.background_color);
        }
    }
}
