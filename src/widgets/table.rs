use std::cmp::max;

use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use widgets::{Widget, Block};
use layout::Rect;
use style::Color;

pub struct Table<'a> {
    block: Option<Block<'a>>,
    header: &'a [&'a str],
    header_color: Color,
    widths: &'a [u16],
    rows: &'a [&'a [&'a str]],
    color: Color,
    column_spacing: u16,
    background_color: Color,
}

impl<'a> Default for Table<'a> {
    fn default() -> Table<'a> {
        Table {
            block: None,
            header: &[],
            header_color: Color::Reset,
            widths: &[],
            rows: &[],
            color: Color::Reset,
            column_spacing: 1,
            background_color: Color::Reset,
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

    pub fn header_color(&mut self, color: Color) -> &mut Table<'a> {
        self.header_color = color;
        self
    }

    pub fn widths(&mut self, widths: &'a [u16]) -> &mut Table<'a> {
        self.widths = widths;
        self
    }

    pub fn rows(&mut self, rows: &'a [&'a [&'a str]]) -> &mut Table<'a> {
        self.rows = rows;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Table<'a> {
        self.color = color;
        self
    }

    pub fn column_spacing(&mut self, spacing: u16) -> &mut Table<'a> {
        self.column_spacing = spacing;
        self
    }

    pub fn background_color(&mut self, color: Color) -> &mut Table<'a> {
        self.background_color = color;
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
        if self.background_color != Color::Reset {
            self.background(&table_area, buf, self.background_color);
        }

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

        if y < table_area.bottom() {
            x = table_area.left();
            for (w, t) in widths.iter().zip(self.header.iter()) {
                buf.set_string(x, y, t, self.header_color, self.background_color);
                x += *w + self.column_spacing;
            }
        }
        y += 2;

        if y < table_area.bottom() {
            let remaining = (table_area.bottom() - y) as usize;
            for (i, row) in self.rows.iter().take(remaining).enumerate() {
                x = table_area.left();
                for (w, elt) in widths.iter().zip(row.iter()) {
                    buf.set_stringn(x,
                                    y + i as u16,
                                    elt,
                                    *w as usize,
                                    self.color,
                                    self.background_color);
                    x += *w + self.column_spacing;
                }
            }
        }
    }
}
