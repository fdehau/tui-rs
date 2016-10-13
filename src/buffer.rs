use layout::Rect;
use style::Color;

#[derive(Debug, Clone)]
pub struct Cell {
    pub symbol: char,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            symbol: ' ',
            fg: Color::White,
            bg: Color::Black,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    area: Rect,
    content: Vec<Cell>,
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

    pub fn index_of(&self, x: u16, y: u16) -> usize {
        let index = (y * self.area.width + x) as usize;
        debug_assert!(index < self.content.len());
        index
    }

    pub fn pos_of(&self, i: usize) -> (u16, u16) {
        debug_assert!(self.area.width != 0);
        (i as u16 % self.area.width, i as u16 / self.area.width)
    }

    pub fn next_pos(&self, x: u16, y: u16) -> Option<(u16, u16)> {
        let mut nx = x + 1;
        let mut ny = y;
        if nx >= self.area.width {
            nx = 0;
            ny = y + 1;
        }
        if ny >= self.area.height {
            None
        } else {
            Some((nx, ny))
        }
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        let i = self.index_of(x, y);
        self.content[i] = cell;
    }

    pub fn set_symbol(&mut self, x: u16, y: u16, symbol: char) {
        let i = self.index_of(x, y);
        self.content[i].symbol = symbol;
    }

    pub fn set_fg(&mut self, x: u16, y: u16, color: Color) {
        let i = self.index_of(x, y);
        self.content[i].fg = color;
    }
    pub fn set_bg(&mut self, x: u16, y: u16, color: Color) {
        let i = self.index_of(x, y);
        self.content[i].bg = color;
    }

    pub fn set_string(&mut self, x: u16, y: u16, string: &str) {
        let mut cursor = (x, y);
        for c in string.chars() {
            let index = self.index_of(cursor.0, cursor.1);
            self.content[index].symbol = c;
            match self.next_pos(cursor.0, cursor.1) {
                Some(c) => {
                    cursor = c;
                }
                None => {
                    warn!("Failed to set all string");
                }
            }
        }
    }

    pub fn get(&self, x: u16, y: u16) -> &Cell {
        let i = self.index_of(x, y);
        &self.content[i]
    }

    pub fn merge(&mut self, other: &Buffer) {
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
