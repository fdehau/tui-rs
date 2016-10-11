use std::cmp::{min, max};
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use buffer::Buffer;
use widgets::{Widget, WidgetType, Block};
use style::Color;
use layout::Rect;

pub struct List<'a, T> {
    block: Block<'a>,
    selected: usize,
    formatter: Box<Fn(&T, bool) -> String>,
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
            block: Block::default(),
            selected: 0,
            formatter: Box::new(|e: &T, selected: bool| String::from("")),
            items: Vec::new(),
        }
    }
}

impl<'a, T> List<'a, T>
    where T: Clone
{
    pub fn block<F>(&'a mut self, f: F) -> &mut List<'a, T>
        where F: Fn(&mut Block)
    {
        f(&mut self.block);
        self
    }

    pub fn formatter<F>(&'a mut self, f: F) -> &mut List<'a, T>
        where F: 'static + Fn(&T, bool) -> String
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
    where T: Display + Hash
{
    fn buffer(&self, area: &Rect) -> Buffer {
        let mut buf = self.block.buffer(area);
        if area.area() == 0 {
            return buf;
        }

        let list_length = self.items.len();
        let list_area = area.inner(1);
        let list_height = list_area.height as usize;
        let bound = min(list_height, list_length);
        let offset = if self.selected > list_height {
            min(self.selected - list_height, list_length - list_height)
        } else {
            0
        };
        for i in 0..bound {
            let index = i + offset;
            let ref item = self.items[index];
            let ref formatter = self.formatter;
            let mut string = formatter(item, self.selected == index);
            string.truncate(list_area.width as usize);
            buf.set_string(1, 1 + i as u16, &string);
        }
        buf
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::List
    }
}
