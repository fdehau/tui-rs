use std::{fmt::Display, iter::Iterator};

use crate::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Widget},
};

/// Holds data to be displayed in a Table widget
pub enum Row<D, I>
where
    D: Iterator<Item = I>,
    I: Display,
{
    Data(D),
    StyledData(D, Style),
}

/// A widget to display data in formatted columns
///
/// # Examples
///
/// ```
/// # use itui::widgets::{Block, Borders, Table, Row};
/// # use itui::style::{Style, Color};
/// # fn main() {
/// let row_style = Style::default().fg(Color::White);
/// Table::new(
///         ["Col1", "Col2", "Col3"].into_iter(),
///         vec![
///             Row::StyledData(["Row11", "Row12", "Row13"].into_iter(), row_style),
///             Row::StyledData(["Row21", "Row22", "Row23"].into_iter(), row_style),
///             Row::StyledData(["Row31", "Row32", "Row33"].into_iter(), row_style),
///             Row::Data(["Row41", "Row42", "Row43"].into_iter())
///         ].into_iter()
///     )
///     .block(Block::default().title("Table"))
///     .header_style(Style::default().fg(Color::Yellow))
///     .widths(&[5, 5, 10])
///     .style(Style::default().fg(Color::White))
///     .column_spacing(1);
/// # }
/// ```
pub struct Table<'a, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T>,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<D, I>>,
{
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Base style for the widget
    style: Style,
    /// Header row for all columns
    header: H,
    /// Style for the header
    header_style: Style,
    /// Width of each column (if the total width is greater than the widget width some columns may
    /// not be displayed)
    widths: &'a [u16],
    /// Space between each column
    column_spacing: u16,
    /// Data to display in each row
    rows: R,
    /// area occupied by this table
    area: Rect,
}

impl<'a, T, H, I, D, R> Default for Table<'a, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T> + Default,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<D, I>> + Default,
{
    fn default() -> Table<'a, T, H, I, D, R> {
        Table {
            block: None,
            style: Style::default(),
            header: H::default(),
            header_style: Style::default(),
            widths: &[],
            rows: R::default(),
            column_spacing: 1,
            area: Default::default(),
        }
    }
}

impl<'a, T, H, I, D, R> Table<'a, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T>,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<D, I>>,
{
    pub fn new(header: H, rows: R) -> Table<'a, T, H, I, D, R> {
        Table {
            block: None,
            style: Style::default(),
            header,
            header_style: Style::default(),
            widths: &[],
            rows,
            column_spacing: 1,
            area: Default::default(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Table<'a, T, H, I, D, R> {
        self.block = Some(block);
        self
    }

    pub fn header<II>(mut self, header: II) -> Table<'a, T, H, I, D, R>
    where
        II: IntoIterator<Item = T, IntoIter = H>,
    {
        self.header = header.into_iter();
        self
    }

    pub fn header_style(mut self, style: Style) -> Table<'a, T, H, I, D, R> {
        self.header_style = style;
        self
    }

    pub fn widths(mut self, widths: &'a [u16]) -> Table<'a, T, H, I, D, R> {
        self.widths = widths;
        self
    }

    pub fn rows<II>(mut self, rows: II) -> Table<'a, T, H, I, D, R>
    where
        II: IntoIterator<Item = Row<D, I>, IntoIter = R>,
    {
        self.rows = rows.into_iter();
        self
    }

    pub fn style(mut self, style: Style) -> Table<'a, T, H, I, D, R> {
        self.style = style;
        self
    }

    pub fn column_spacing(mut self, spacing: u16) -> Table<'a, T, H, I, D, R> {
        self.column_spacing = spacing;
        self
    }

    pub fn area(mut self, area: Rect) -> Self {
        self.area = area;
        self
    }
}

impl<'a, T, H, I, D, R> Widget for Table<'a, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T>,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<D, I>>,
{
    fn get_area(&self) -> Rect {
        self.area
    }
    fn draw(&mut self, buf: &mut Buffer) {
        // Render block if necessary and get the drawing self.area
        let table_area = match self.block {
            Some(ref mut b) => {
                b.draw(buf);
                b.inner()
            }
            None => self.area,
        };

        // Set the background
        self.background(buf, self.style.bg);

        // Save widths of the columns that will fit in the given area
        let mut x = 0;
        let mut widths = Vec::with_capacity(self.widths.len());
        for width in self.widths.iter() {
            if x + width < table_area.width {
                widths.push(*width);
            }
            x += *width;
        }

        let mut y = table_area.top();

        // Draw header
        if y < table_area.bottom() {
            x = table_area.left();
            for (w, t) in widths.iter().zip(self.header.by_ref()) {
                buf.set_string(x, y, format!("{}", t), self.header_style);
                x += *w + self.column_spacing;
            }
        }
        y += 2;

        // Draw rows
        let default_style = Style::default();
        if y < table_area.bottom() {
            let remaining = (table_area.bottom() - y) as usize;
            for (i, row) in self.rows.by_ref().take(remaining).enumerate() {
                let (data, style) = match row {
                    Row::Data(d) => (d, default_style),
                    Row::StyledData(d, s) => (d, s),
                };
                x = table_area.left();
                for (w, elt) in widths.iter().zip(data) {
                    buf.set_stringn(x, y + i as u16, format!("{}", elt), *w as usize, style);
                    x += *w + self.column_spacing;
                }
            }
        }
    }
}
