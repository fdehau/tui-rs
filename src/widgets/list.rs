use std::cmp::min;

use buffer::Buffer;
use widgets::{Widget, Block};
use layout::Rect;
use style::Color;

pub struct List<'a> {
    block: Option<Block<'a>>,
    selected: usize,
    items: Vec<(&'a str, Color, Color)>,
}

impl<'a> Default for List<'a> {
    fn default() -> List<'a> {
        List {
            block: None,
            selected: 0,
            items: Vec::new(),
        }
    }
}

impl<'a> List<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut List<'a> {
        self.block = Some(block);
        self
    }

    pub fn items(&'a mut self, items: &[(&'a str, Color, Color)]) -> &mut List<'a> {
        self.items = items.to_vec();
        self
    }
}

impl<'a> Widget<'a> for List<'a> {
    fn buffer(&'a self, area: &Rect) -> Buffer<'a> {

        let (mut buf, list_area) = match self.block {
            Some(ref b) => (b.buffer(area), b.inner(*area)),
            None => (Buffer::empty(*area), *area),
        };

        let list_length = self.items.len();
        let list_height = list_area.height as usize;
        let bound = min(list_height, list_length);
        let offset = if self.selected > list_height {
            min(self.selected - list_height, list_length - list_height)
        } else {
            0
        };
        for i in 0..bound {
            let index = i + offset;
            let (item, fg, bg) = self.items[index];
            buf.set_string(1, 1 + i as u16, item, fg, bg);
        }
        buf
    }
}
