use std::cmp::min;
use std::usize;

use unicode_segmentation::UnicodeSegmentation;

use layout::Rect;
use style::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub fg: Color,
    pub bg: Color,
    pub symbol: String,
}

impl Cell {
    pub fn reset(&mut self) {
        self.symbol.clear();
        self.symbol.push(' ');
        self.fg = Color::Reset;
        self.bg = Color::Reset;
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            symbol: " ".into(),
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub area: Rect,
    pub content: Vec<Cell>,
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer {
            area: Default::default(),
            content: Vec::new(),
        }
    }
}

impl Buffer {
    pub fn empty(area: Rect) -> Buffer {
        let cell: Cell = Default::default();
        Buffer::filled(area, cell)
    }

    pub fn filled(area: Rect, cell: Cell) -> Buffer {
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

    pub fn content(&self) -> &[Cell] {
        &self.content
    }

    pub fn area(&self) -> &Rect {
        &self.area
    }

    pub fn at(&self, x: u16, y: u16) -> &Cell {
        let i = self.index_of(x, y);
        &self.content[i]
    }

    pub fn index_of(&self, x: u16, y: u16) -> usize {
        let index = (y * self.area.width + x) as usize;
        debug_assert!(index < self.content.len());
        index
    }

    pub fn pos_of(&self, i: usize) -> (u16, u16) {
        debug_assert!(self.area.width > 0);
        (i as u16 % self.area.width, i as u16 / self.area.width)
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        let i = self.index_of(x, y);
        self.content[i] = cell;
    }

    pub fn set_symbol(&mut self, x: u16, y: u16, symbol: &str) {
        let i = self.index_of(x, y);
        self.content[i].symbol.clear();
        self.content[i].symbol.push_str(symbol);
    }

    pub fn set_fg(&mut self, x: u16, y: u16, color: Color) {
        let i = self.index_of(x, y);
        self.content[i].fg = color;
    }
    pub fn set_bg(&mut self, x: u16, y: u16, color: Color) {
        let i = self.index_of(x, y);
        self.content[i].bg = color;
    }

    pub fn set_string(&mut self, x: u16, y: u16, string: &str, fg: Color, bg: Color) {
        self.set_characters(x, y, string, usize::MAX, fg, bg);
    }

    pub fn set_characters(&mut self,
                          x: u16,
                          y: u16,
                          string: &str,
                          limit: usize,
                          fg: Color,
                          bg: Color) {
        let mut index = self.index_of(x, y);
        let graphemes = UnicodeSegmentation::graphemes(string, true).collect::<Vec<&str>>();
        let max_index = min((self.area.width - x) as usize, limit);
        for s in graphemes.into_iter().take(max_index) {
            self.content[index].symbol.clear();
            self.content[index].symbol.push_str(s);
            self.content[index].fg = fg;
            self.content[index].bg = bg;
            index += 1;
        }
    }


    pub fn update_colors(&mut self, x: u16, y: u16, fg: Color, bg: Color) {
        let i = self.index_of(x, y);
        self.content[i].fg = fg;
        self.content[i].bg = bg;
    }

    pub fn update_cell(&mut self, x: u16, y: u16, symbol: &str, fg: Color, bg: Color) {
        let i = self.index_of(x, y);
        self.content[i].symbol.clear();
        self.content[i].symbol.push_str(symbol);
        self.content[i].fg = fg;
        self.content[i].bg = bg;
    }

    pub fn resize(&mut self, area: Rect) {
        let length = area.area() as usize;
        if self.content.len() > length {
            self.content.truncate(length);
        } else {
            self.content.resize(length, Default::default());
        }
        self.area = area;
    }

    pub fn reset(&mut self) {
        for c in &mut self.content {
            c.reset();
        }
    }

    pub fn merge(&mut self, other: Buffer) {
        let area = self.area.union(&other.area);
        let cell: Cell = Default::default();
        self.content.resize(area.area() as usize, cell.clone());

        // Move original content to the appropriate space
        let offset_x = self.area.x - area.x;
        let offset_y = self.area.y - area.y;
        let size = self.area.area() as usize;
        for i in (0..size).rev() {
            let (x, y) = self.pos_of(i);
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
            let (x, y) = other.pos_of(i);
            // New index in content
            let k = ((y + offset_y) * area.width + (x + offset_x)) as usize;
            self.content[k] = other.content[i].clone();
        }
        self.area = area;
    }
}
