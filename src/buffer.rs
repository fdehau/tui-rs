use unicode_segmentation::UnicodeSegmentation;

use layout::Rect;
use style::Color;

#[derive(Debug, Clone)]
pub struct Cell<'a> {
    pub symbol: &'a str,
    pub fg: Color,
    pub bg: Color,
}

impl<'a> Default for Cell<'a> {
    fn default() -> Cell<'a> {
        Cell {
            symbol: "",
            fg: Color::White,
            bg: Color::Black,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Buffer<'a> {
    area: Rect,
    content: Vec<Cell<'a>>,
}

impl<'a> Default for Buffer<'a> {
    fn default() -> Buffer<'a> {
        Buffer {
            area: Default::default(),
            content: Vec::new(),
        }
    }
}

impl<'a> Buffer<'a> {
    pub fn empty(area: Rect) -> Buffer<'a> {
        let cell: Cell = Default::default();
        Buffer::filled(area, cell)
    }

    pub fn filled(area: Rect, cell: Cell<'a>) -> Buffer<'a> {
        let size = area.area() as usize;
        let mut content = Vec::with_capacity(size);
        for _ in 0..size {
            content.push(cell.clone());
        }
        Buffer {
            area: area,
            content: content,
        }
    }

    pub fn content(&'a self) -> &'a [Cell<'a>] {
        &self.content
    }

    pub fn area(&self) -> &Rect {
        &self.area
    }

    pub fn index_of(&self, x: u16, y: u16) -> Option<usize> {
        let index = (y * self.area.width + x) as usize;
        if index < self.content.len() {
            Some(index)
        } else {
            None
        }
    }

    pub fn pos_of(&self, i: usize) -> Option<(u16, u16)> {
        if self.area.width > 0 {
            Some((i as u16 % self.area.width, i as u16 / self.area.width))
        } else {
            None
        }
    }

    pub fn next_pos(&self, x: u16, y: u16) -> Option<(u16, u16)> {
        let (nx, ny) = if x + 1 > self.area.width {
            (0, y + 1)
        } else {
            (x + 1, y)
        };
        if ny >= self.area.height {
            None
        } else {
            Some((nx, ny))
        }
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell<'a>) {
        if let Some(i) = self.index_of(x, y) {
            self.content[i] = cell;
        }
    }

    pub fn set_symbol(&mut self, x: u16, y: u16, symbol: &'a str) {
        if let Some(i) = self.index_of(x, y) {
            self.content[i].symbol = symbol;
        }
    }

    pub fn set_fg(&mut self, x: u16, y: u16, color: Color) {
        if let Some(i) = self.index_of(x, y) {
            self.content[i].fg = color;
        }
    }
    pub fn set_bg(&mut self, x: u16, y: u16, color: Color) {
        if let Some(i) = self.index_of(x, y) {
            self.content[i].bg = color;
        }
    }

    pub fn set_string(&mut self, x: u16, y: u16, string: &'a str, fg: Color, bg: Color) {
        let index = self.index_of(x, y);
        if index.is_none() {
            return;
        }
        let mut index = index.unwrap();
        let graphemes = UnicodeSegmentation::graphemes(string, true).collect::<Vec<&str>>();
        let max_index = (self.area.width - x) as usize;
        for s in graphemes.iter().take(max_index) {
            self.content[index].symbol = s;
            self.content[index].fg = fg;
            self.content[index].bg = bg;
            index += 1;
        }
    }

    pub fn update_cell<F>(&mut self, x: u16, y: u16, f: F)
        where F: Fn(&mut Cell)
    {
        if let Some(i) = self.index_of(x, y) {
            f(&mut self.content[i]);
        }
    }

    pub fn merge(&'a mut self, other: Buffer<'a>) {
        let area = self.area.union(&other.area);
        let cell: Cell = Default::default();
        self.content.resize(area.area() as usize, cell.clone());

        // Move original content to the appropriate space
        let offset_x = self.area.x - area.x;
        let offset_y = self.area.y - area.y;
        let size = self.area.area() as usize;
        for i in (0..size).rev() {
            let (x, y) = self.pos_of(i).unwrap();
            // New index in content
            let k = ((y + offset_y) * area.width + (x + offset_x)) as usize;
            self.content[k] = self.content[i].clone();
            if i != k {
                self.content[i] = cell.clone();
            }
        }

        // Push content of the other buffer into this one (may erase previous
        // data)
        let offset_x = other.area.x - area.x;
        let offset_y = other.area.y - area.y;
        let size = other.area.area() as usize;
        for i in 0..size {
            let (x, y) = other.pos_of(i).unwrap();
            // New index in content
            let k = ((y + offset_y) * area.width + (x + offset_x)) as usize;
            self.content[k] = other.content[i].clone();
        }
        self.area = area;
    }
}
