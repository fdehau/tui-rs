use std::cmp::max;

use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use widgets::{Widget, Block};
use layout::Rect;
use style::Color;

pub struct Table<'a> {
    block: Option<Block<'a>>,
    titles: &'a [&'a str],
    widths: &'a [u16],
    rows: &'a [&'a [&'a str]],
    column_spacing: u16,
}

impl<'a> Default for Table<'a> {
    fn default() -> Table<'a> {
        Table {
            block: None,
            titles: &[],
            widths: &[],
            rows: &[],
            column_spacing: 1,
        }
    }
}

impl<'a> Table<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut Table<'a> {
        self.block = Some(block);
        self
    }

    pub fn titles(&mut self, titles: &'a [&'a str]) -> &mut Table<'a> {
        self.titles = titles;
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

    pub fn column_spacing(&mut self, spacing: u16) -> &mut Table<'a> {
        self.column_spacing = spacing;
        self
    }
}

impl<'a> Widget for Table<'a> {
    fn buffer(&self, area: &Rect, buf: &mut Buffer) {
        let table_area = match self.block {
            Some(ref b) => {
                b.buffer(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        let mut x = 0;
        let mut widths = Vec::with_capacity(self.widths.len());
        for (width, title) in self.widths.iter().zip(self.titles.iter()) {
            let w = max(title.width() as u16, *width);
            if x + w < table_area.width {
                widths.push(w);
            }
            x += w;
        }

        let mut y = table_area.top();

        if y < table_area.bottom() {
            x = table_area.left();
            for (w, t) in widths.iter().zip(self.titles.iter()) {
                buf.set_string(x, y, t, Color::Reset, Color::Reset);
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
                                    Color::Reset,
                                    Color::Reset);
                    x += *w + self.column_spacing;
                }
            }
        }
    }
}
