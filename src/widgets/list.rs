use std::cmp::min;
use std::hash::{Hash, Hasher};

use buffer::Buffer;
use widgets::{Widget, WidgetType, Block};
use layout::Rect;
use style::Color;

pub struct List<'a, T> {
    block: Option<Block<'a>>,
    selected: usize,
    formatter: Box<Fn(&T, bool) -> (String, Color, Color)>,
    items: Vec<T>,
}

impl<'a, T> Hash for List<'a, T>
    where T: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.block.hash(state);
        self.selected.hash(state);
        self.items.hash(state);
    }
}

impl<'a, T> Default for List<'a, T> {
    fn default() -> List<'a, T> {
        List {
            block: None,
            selected: 0,
            formatter: Box::new(|_, _| (String::from(""), Color::White, Color::Black)),
            items: Vec::new(),
        }
    }
}

impl<'a, T> List<'a, T>
    where T: Clone
{
    pub fn block(&'a mut self, block: Block<'a>) -> &mut List<'a, T> {
        self.block = Some(block);
        self
    }

    pub fn formatter<F>(&'a mut self, f: F) -> &mut List<'a, T>
        where F: 'static + Fn(&T, bool) -> (String, Color, Color)
    {
        self.formatter = Box::new(f);
        self
    }

    pub fn items(&'a mut self, items: &'a [T]) -> &mut List<'a, T> {
        self.items = items.to_vec();
        self
    }

    pub fn select(&'a mut self, index: usize) -> &mut List<'a, T> {
        self.selected = index;
        self
    }
}

impl<'a, T> Widget for List<'a, T>
    where T: Hash
{
    fn buffer(&self, area: &Rect) -> Buffer {

        let (mut buf, list_area) = match self.block {
            Some(ref b) => (b.buffer(area), area.inner(1)),
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
            let item = &self.items[index];
            let formatter = &self.formatter;
            let (mut string, fg, bg) = formatter(item, self.selected == index);
            string.truncate(list_area.width as usize);
            buf.set_string(1, 1 + i as u16, &string, fg, bg);
        }
        buf
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::List
    }
}
