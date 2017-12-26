use std::fmt::Display;
use std::iter::Iterator;

use buffer::Buffer;
use widgets::{Block, Widget};
use layout::Rect;
use style::Style;

/// Holds data to be displayed in a Table widget
pub enum Row<'i, D, I>
where
    D: Iterator<Item = I>,
    I: Display,
{
    Data(D),
    StyledData(D, &'i Style),
}

/// A widget to display data in formatted columns
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, Table, Row};
/// # use tui::style::{Style, Color};
/// # fn main() {
/// let row_style = Style::default().fg(Color::White);
/// Table::new(
///         ["Col1", "Col2", "Col3"].into_iter(),
///         vec![
///             Row::StyledData(["Row11", "Row12", "Row13"].into_iter(), &row_style),
///             Row::StyledData(["Row21", "Row22", "Row23"].into_iter(), &row_style),
///             Row::StyledData(["Row31", "Row32", "Row33"].into_iter(), &row_style),
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
pub struct Table<'a, 'i, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T>,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<'i, D, I>>,
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
}

impl<'a, 'i, T, H, I, D, R> Default for Table<'a, 'i, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T> + Default,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<'i, D, I>> + Default,
{
    fn default() -> Table<'a, 'i, T, H, I, D, R> {
        Table {
            block: None,
            style: Style::default(),
            header: H::default(),
            header_style: Style::default(),
            widths: &[],
            rows: R::default(),
            column_spacing: 1,
        }
    }
}

impl<'a, 'i, T, H, I, D, R> Table<'a, 'i, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T>,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<'i, D, I>>,
{
    pub fn new(header: H, rows: R) -> Table<'a, 'i, T, H, I, D, R> {
        Table {
            block: None,
            style: Style::default(),
            header: header,
            header_style: Style::default(),
            widths: &[],
            rows: rows,
            column_spacing: 1,
        }
    }
    pub fn block(&'a mut self, block: Block<'a>) -> &mut Table<'a, 'i, T, H, I, D, R> {
        self.block = Some(block);
        self
    }

    pub fn header<II>(&mut self, header: II) -> &mut Table<'a, 'i, T, H, I, D, R>
    where
        II: IntoIterator<Item = T, IntoIter = H>,
    {
        self.header = header.into_iter();
        self
    }

    pub fn header_style(&mut self, style: Style) -> &mut Table<'a, 'i, T, H, I, D, R> {
        self.header_style = style;
        self
    }

    pub fn widths(&mut self, widths: &'a [u16]) -> &mut Table<'a, 'i, T, H, I, D, R> {
        self.widths = widths;
        self
    }

    pub fn rows<II>(&mut self, rows: II) -> &mut Table<'a, 'i, T, H, I, D, R>
    where
        II: IntoIterator<Item = Row<'i, D, I>, IntoIter = R>,
    {
        self.rows = rows.into_iter();
        self
    }

    pub fn style(&mut self, style: Style) -> &mut Table<'a, 'i, T, H, I, D, R> {
        self.style = style;
        self
    }

    pub fn column_spacing(&mut self, spacing: u16) -> &mut Table<'a, 'i, T, H, I, D, R> {
        self.column_spacing = spacing;
        self
    }
}

impl<'a, 'i, T, H, I, D, R> Widget for Table<'a, 'i, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T>,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<'i, D, I>>,
{
    fn draw(&mut self, area: &Rect, buf: &mut Buffer) {
        // Render block if necessary and get the drawing area
        let table_area = match self.block {
            Some(ref mut b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        // Set the background
        self.background(&table_area, buf, self.style.bg);

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
                buf.set_string(x, y, &format!("{}", t), &self.header_style);
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
                    Row::Data(d) => (d, &default_style),
                    Row::StyledData(d, s) => (d, s),
                };
                x = table_area.left();
                for (w, elt) in widths.iter().zip(data) {
                    buf.set_stringn(x, y + i as u16, &format!("{}", elt), *w as usize, style);
                    x += *w + self.column_spacing;
                }
            }
        }
    }
}
