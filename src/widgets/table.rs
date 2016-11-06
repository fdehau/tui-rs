use std::cmp::max;
use std::borrow::Cow;

use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use widgets::{Widget, Block};
use layout::Rect;
use style::Style;

/// A widget to display data in formatted column
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, border, Table};
/// # use tui::style::Color;
/// # fn main() {
/// Table::default()
///     .block(Block::default().title("Table"))
///     .header(&["Col1", "Col2", "Col3"])
///     .header_color(Color::Yellow)
///     .widths(&[5, 5, 10])
///     .column_spacing(1)
///     .rows(vec![["Row11", "Row12", "Row13"].as_ref(),
///                ["Row21", "Row22", "Row23"].as_ref(),
///                ["Row31", "Row32", "Row33"].as_ref()])
///     .color(Color::White)
///     .background_color(Color::Black);
/// # }
/// ```
pub struct Table<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Base style for the widget
    style: Style,
    /// Header row for all columns
    header: &'a [&'a str],
    /// Style for the header
    header_style: Style,
    /// Width of each column (if the total width is greater than the widget width some columns may
    /// not be displayed)
    widths: &'a [u16],
    /// Space between each column
    column_spacing: u16,
    /// Data to display in each row
    rows: Vec<Cow<'a, [&'a str]>>,
    /// Style for each row
    row_style: Style,
}

impl<'a> Default for Table<'a> {
    fn default() -> Table<'a> {
        Table {
            block: None,
            style: Style::default(),
            header: &[],
            header_style: Style::default(),
            widths: &[],
            rows: Vec::new(),
            row_style: Style::default(),
            column_spacing: 1,
        }
    }
}

impl<'a> Table<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut Table<'a> {
        self.block = Some(block);
        self
    }

    pub fn header(&mut self, header: &'a [&'a str]) -> &mut Table<'a> {
        self.header = header;
        self
    }

    pub fn header_style(&mut self, style: Style) -> &mut Table<'a> {
        self.header_style = style;
        self
    }

    pub fn widths(&mut self, widths: &'a [u16]) -> &mut Table<'a> {
        self.widths = widths;
        self
    }

    pub fn rows<R>(&mut self, rows: Vec<R>) -> &mut Table<'a>
        where R: Into<Cow<'a, [&'a str]>>
    {
        self.rows = rows.into_iter().map(|r| r.into()).collect::<Vec<Cow<'a, [&'a str]>>>();
        self
    }

    pub fn row_style(&mut self, style: Style) -> &mut Table<'a> {
        self.row_style = style;
        self
    }

    pub fn column_spacing(&mut self, spacing: u16) -> &mut Table<'a> {
        self.column_spacing = spacing;
        self
    }
}

impl<'a> Widget for Table<'a> {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {

        // Render block if necessary and get the drawing area
        let table_area = match self.block {
            Some(ref b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        // Set the background
        self.background(&table_area, buf, self.style.bg);

        let mut x = 0;
        let mut widths = Vec::with_capacity(self.widths.len());
        for (width, title) in self.widths.iter().zip(self.header.iter()) {
            let w = max(title.width() as u16, *width);
            if x + w < table_area.width {
                widths.push(w);
            }
            x += w;
        }

        let mut y = table_area.top();

        // Header
        if y < table_area.bottom() {
            x = table_area.left();
            for (w, t) in widths.iter().zip(self.header.iter()) {
                buf.set_string(x, y, t, &self.header_style);
                x += *w + self.column_spacing;
            }
        }
        y += 2;

        if y < table_area.bottom() {
            let remaining = (table_area.bottom() - y) as usize;
            for (i, row) in self.rows.iter().take(remaining).enumerate() {
                x = table_area.left();
                for (w, elt) in widths.iter().zip(row.iter()) {
                    buf.set_stringn(x, y + i as u16, elt, *w as usize, &self.row_style);
                    x += *w + self.column_spacing;
                }
            }
        }
    }
}
