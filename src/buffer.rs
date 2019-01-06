use std::cmp::min;
use std::fmt;
use std::usize;

use unicode_segmentation::UnicodeSegmentation;

use crate::layout::Rect;
use crate::style::{Color, Modifier, Style};

/// A buffer cell
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub symbol: String,
    pub style: Style,
}

impl Cell {
    pub fn set_symbol(&mut self, symbol: &str) -> &mut Cell {
        self.symbol.clear();
        self.symbol.push_str(symbol);
        self
    }

    pub fn set_char(&mut self, ch: char) -> &mut Cell {
        self.symbol.clear();
        self.symbol.push(ch);
        self
    }

    pub fn set_fg(&mut self, color: Color) -> &mut Cell {
        self.style.fg = color;
        self
    }

    pub fn set_bg(&mut self, color: Color) -> &mut Cell {
        self.style.bg = color;
        self
    }

    pub fn set_modifier(&mut self, modifier: Modifier) -> &mut Cell {
        self.style.modifier = modifier;
        self
    }

    pub fn set_style(&mut self, style: Style) -> &mut Cell {
        self.style = style;
        self
    }

    pub fn reset(&mut self) {
        self.symbol.clear();
        self.symbol.push(' ');
        self.style.reset();
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            symbol: " ".into(),
            style: Default::default(),
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
/// use tui::buffer::{Buffer, Cell};
/// use tui::layout::Rect;
/// use tui::style::{Color, Style, Modifier};
///
/// # fn main() {
/// let mut buf = Buffer::empty(Rect{x: 0, y: 0, width: 10, height: 5});
/// buf.get_mut(0, 2).set_symbol("x");
/// assert_eq!(buf.get(0, 2).symbol, "x");
/// buf.set_string(3, 0, "string", Style::default().fg(Color::Red).bg(Color::White));
/// assert_eq!(buf.get(5, 0), &Cell{
///     symbol: String::from("r"),
///     style: Style {
///         fg: Color::Red,
///         bg: Color::White,
///         modifier: Modifier::Reset
///     }});
/// buf.get_mut(5, 0).set_char('x');
/// assert_eq!(buf.get(5, 0).symbol, "x");
/// # }
/// ```
#[derive(Clone, PartialEq)]
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

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Buffer: {:?}", self.area)?;
        f.write_str("Content (quoted lines):\n")?;
        for cells in self.content.chunks(self.area.width as usize) {
            let line: String = cells.iter().map(|cell| &cell.symbol[..]).collect();
            f.write_fmt(format_args!("{:?},\n", line))?;
        }
        f.write_str("Style:\n")?;
        for cells in self.content.chunks(self.area.width as usize) {
            f.write_str("|")?;
            for cell in cells {
                write!(
                    f,
                    "{} {} {}|",
                    cell.style.fg.code(),
                    cell.style.bg.code(),
                    cell.style.modifier.code()
                )?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Buffer {
    /// Returns a Buffer with all cells set to the default one
    pub fn empty(area: Rect) -> Buffer {
        let cell: Cell = Default::default();
        Buffer::filled(area, &cell)
    }

    /// Returns a Buffer with all cells initialized with the attributes of the given Cell
    pub fn filled(area: Rect, cell: &Cell) -> Buffer {
        let size = area.area() as usize;
        let mut content = Vec::with_capacity(size);
        for _ in 0..size {
            content.push(cell.clone());
        }
        Buffer { area, content }
    }

    /// Returns a Buffer containing the given lines
    pub fn with_lines<S>(lines: Vec<S>) -> Buffer
    where
        S: AsRef<str>,
    {
        let height = lines.len() as u16;
        let width = lines.iter().fold(0, |acc, item| {
            std::cmp::max(
                acc,
                UnicodeSegmentation::graphemes(item.as_ref(), true).count() as u16,
            )
        });
        let mut buffer = Buffer::empty(Rect {
            x: 0,
            y: 0,
            width,
            height,
        });
        let mut y = 0;
        for line in &lines {
            buffer.set_string(0, y, line, Style::default());
            y += 1;
        }
        buffer
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
    pub fn get(&self, x: u16, y: u16) -> &Cell {
        let i = self.index_of(x, y);
        &self.content[i]
    }

    /// Returns a mutable reference to Cell at the given coordinates
    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        let i = self.index_of(x, y);
        &mut self.content[i]
    }

    /// Returns the index in the Vec<Cell> for the given global (x, y) coordinates.
    ///
    /// Global coordinates are offset by the Buffer's area offset (`x`/`y`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use tui::buffer::Buffer;
    /// # use tui::layout::Rect;
    /// let rect = Rect::new(200, 100, 10, 10);
    /// let buffer = Buffer::empty(rect);
    /// // Global coordinates to the top corner of this buffer's area
    /// assert_eq!(buffer.index_of(200, 100), 0);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics when given an coordinate that is outside of this Buffer's area.
    ///
    /// ```should_panic
    /// # use tui::buffer::Buffer;
    /// # use tui::layout::Rect;
    /// let rect = Rect::new(200, 100, 10, 10);
    /// let buffer = Buffer::empty(rect);
    /// // Top coordinate is outside of the buffer in global coordinate space, as the Buffer's area
    /// // starts at (200, 100).
    /// buffer.index_of(0, 0); // Panics
    /// ```
    pub fn index_of(&self, x: u16, y: u16) -> usize {
        debug_assert!(
            x >= self.area.left()
                && x < self.area.right()
                && y >= self.area.top()
                && y < self.area.bottom(),
            "Trying to access position outside the buffer: x={}, y={}, area={:?}",
            x,
            y,
            self.area
        );
        ((y - self.area.y) * self.area.width + (x - self.area.x)) as usize
    }

    /// Returns the (global) coordinates of a cell given its index
    ///
    /// Global coordinates are offset by the Buffer's area offset (`x`/`y`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use tui::buffer::Buffer;
    /// # use tui::layout::Rect;
    /// let rect = Rect::new(200, 100, 10, 10);
    /// let buffer = Buffer::empty(rect);
    /// assert_eq!(buffer.pos_of(0), (200, 100));
    /// assert_eq!(buffer.pos_of(14), (204, 101));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics when given an index that is outside the Buffer's content.
    ///
    /// ```should_panic
    /// # use tui::buffer::Buffer;
    /// # use tui::layout::Rect;
    /// let rect = Rect::new(0, 0, 10, 10); // 100 cells in total
    /// let buffer = Buffer::empty(rect);
    /// // Index 100 is the 101th cell, which lies outside of the area of this Buffer.
    /// buffer.pos_of(100); // Panics
    /// ```
    pub fn pos_of(&self, i: usize) -> (u16, u16) {
        debug_assert!(
            i < self.content.len(),
            "Trying to get the coords of a cell outside the buffer: i={} len={}",
            i,
            self.content.len()
        );
        (
            self.area.x + i as u16 % self.area.width,
            self.area.y + i as u16 / self.area.width,
        )
    }

    /// Print a string, starting at the position (x, y)
    pub fn set_string<S>(&mut self, x: u16, y: u16, string: S, style: Style)
    where
        S: AsRef<str>,
    {
        self.set_stringn(x, y, string, usize::MAX, style);
    }

    /// Print at most the first n characters of a string if enough space is available
    /// until the end of the line
    pub fn set_stringn<S>(&mut self, x: u16, y: u16, string: S, limit: usize, style: Style)
    where
        S: AsRef<str>,
    {
        let mut index = self.index_of(x, y);
        let graphemes = UnicodeSegmentation::graphemes(string.as_ref(), true);
        let max_index = min((self.area.right() - x) as usize, limit);
        for s in graphemes.take(max_index) {
            self.content[index].symbol.clear();
            self.content[index].symbol.push_str(s);
            self.content[index].style = style;
            index += 1;
        }
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
    pub fn merge(&mut self, other: &Buffer) {
        let area = self.area.union(other.area);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_translates_to_and_from_coordinates() {
        let rect = Rect::new(200, 100, 50, 80);
        let buf = Buffer::empty(rect);

        // First cell is at the upper left corner.
        assert_eq!(buf.pos_of(0), (200, 100));
        assert_eq!(buf.index_of(200, 100), 0);

        // Last cell is in the lower right.
        assert_eq!(buf.pos_of(buf.content.len() - 1), (249, 179));
        assert_eq!(buf.index_of(249, 179), buf.content.len() - 1);
    }

    #[test]
    #[should_panic(expected = "outside the buffer")]
    fn pos_of_panics_on_out_of_bounds() {
        let rect = Rect::new(0, 0, 10, 10);
        let buf = Buffer::empty(rect);

        // There are a total of 100 cells; zero-indexed means that 100 would be the 101st cell.
        buf.pos_of(100);
    }

    #[test]
    #[should_panic(expected = "outside the buffer")]
    fn index_of_panics_on_out_of_bounds() {
        let rect = Rect::new(0, 0, 10, 10);
        let buf = Buffer::empty(rect);

        // width is 10; zero-indexed means that 10 would be the 11th cell.
        buf.index_of(10, 0);
    }
}
