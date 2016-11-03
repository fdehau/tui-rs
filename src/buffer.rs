use std::cmp::min;
use std::usize;

use unicode_segmentation::UnicodeSegmentation;

use layout::Rect;
use style::Color;

/// A buffer cell
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

/// A buffer that maps to the desired content of the terminal after the draw call
///
/// No widget in the library interacts directly with the terminal. Instead each of them is required
/// to draw their state to an intermediate buffer. It is basically a grid where each cell contains
/// a grapheme, a foreground color and a background color. This grid will then be used to output
/// the appropriate escape sequences and characters to draw the UI as the user has defined it.
///
/// # Examples:
///
/// ```
/// # extern crate tui;
/// use tui::buffer::{Buffer, Cell};
/// use tui::layout::Rect;
/// use tui::style::Color;
///
/// # fn main() {
/// let mut buf = Buffer::empty(Rect{x: 0, y: 0, width: 10, height: 5});
/// buf.set_symbol(0, 2, "x");
/// assert_eq!(buf.at(0, 2).symbol, "x");
/// buf.set_string(3, 0, "string", Color::Red, Color::White);
/// assert_eq!(buf.at(5, 0), &Cell{symbol: String::from("r"), fg: Color::Red, bg: Color::White});
/// buf.update_cell(5, 0, |c| {
///     c.symbol.clear();
///     c.symbol.push('x');
/// });
/// assert_eq!(buf.at(5, 0).symbol, "x");
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Buffer {
    /// The area represented by this buffer
    pub area: Rect,
    /// The content of the buffer. The length of this Vec should always be equal to area.width *
    /// area.height
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
    /// Returns a Buffer with all cells set to the default one
    pub fn empty(area: Rect) -> Buffer {
        let cell: Cell = Default::default();
        Buffer::filled(area, cell)
    }

    /// Returns a Buffer with all cells initialized with the attributes of the given Cell
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

    /// Returns the content of the buffer as a slice
    pub fn content(&self) -> &[Cell] {
        &self.content
    }

    /// Returns the area covered by this buffer
    pub fn area(&self) -> &Rect {
        &self.area
    }

    /// Returns a reference to Cell at the given coordinates
    pub fn at(&self, x: u16, y: u16) -> &Cell {
        let i = self.index_of(x, y);
        &self.content[i]
    }

    /// Returns the index in the Vec<Cell> for the given (x, y)
    pub fn index_of(&self, x: u16, y: u16) -> usize {
        debug_assert!(x >= self.area.left() && x < self.area.right() && y >= self.area.top() &&
                      y < self.area.bottom(),
                      "Trying to access position outside the buffer: x={}, y={}, area={:?}",
                      x,
                      y,
                      self.area);
        let index = ((y - self.area.y) * self.area.width + (x - self.area.x)) as usize;
        index
    }

    /// Returns the coordinates of a cell given its index
    pub fn pos_of(&self, i: usize) -> (u16, u16) {
        debug_assert!(i >= self.content.len(),
                      "Trying to get the coords of a cell outside the buffer: i={} len={}",
                      i,
                      self.content.len());
        (self.area.x + i as u16 % self.area.width, self.area.y + i as u16 / self.area.width)
    }

    /// Update the symbol of a cell at (x, y)
    pub fn set_symbol(&mut self, x: u16, y: u16, symbol: &str) {
        let i = self.index_of(x, y);
        self.content[i].symbol.clear();
        self.content[i].symbol.push_str(symbol);
    }

    /// Update the foreground color of a cell at (x, y)
    pub fn set_fg(&mut self, x: u16, y: u16, color: Color) {
        let i = self.index_of(x, y);
        self.content[i].fg = color;
    }

    /// Update the background color of a cell at (x, y)
    pub fn set_bg(&mut self, x: u16, y: u16, color: Color) {
        let i = self.index_of(x, y);
        self.content[i].bg = color;
    }

    /// Print a string, starting at the position (x, y)
    pub fn set_string(&mut self, x: u16, y: u16, string: &str, fg: Color, bg: Color) {
        self.set_stringn(x, y, string, usize::MAX, fg, bg);
    }

    /// Print at most the first n characters of a string if enough space is available
    /// until the end of the line
    pub fn set_stringn(&mut self,
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


    /// Update both the foreground and the background colors in a single method call
    pub fn set_colors(&mut self, x: u16, y: u16, fg: Color, bg: Color) {
        let i = self.index_of(x, y);
        self.content[i].fg = fg;
        self.content[i].bg = bg;
    }

    /// Update all attributes of a cell at the given position
    pub fn set_cell(&mut self, x: u16, y: u16, symbol: &str, fg: Color, bg: Color) {
        let i = self.index_of(x, y);
        self.content[i].symbol.clear();
        self.content[i].symbol.push_str(symbol);
        self.content[i].fg = fg;
        self.content[i].bg = bg;
    }

    /// Update a cell using the closure passed as last argument
    pub fn update_cell<F>(&mut self, x: u16, y: u16, f: F)
        where F: Fn(&mut Cell)
    {
        let i = self.index_of(x, y);
        f(&mut self.content[i]);
    }

    /// Resize the buffer so that the mapped area matches the given area and that the buffer
    /// length is equal to area.width * area.height
    pub fn resize(&mut self, area: Rect) {
        let length = area.area() as usize;
        if self.content.len() > length {
            self.content.truncate(length);
        } else {
            self.content.resize(length, Default::default());
        }
        self.area = area;
    }

    /// Reset all cells in the buffer
    pub fn reset(&mut self) {
        for c in &mut self.content {
            c.reset();
        }
    }

    /// Merge an other buffer into this one
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
