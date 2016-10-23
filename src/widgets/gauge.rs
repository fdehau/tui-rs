use std::cmp::{max, min};

use widgets::{Widget, Block};
use buffer::Buffer;
use style::Color;
use layout::Rect;

/// Progress bar widget
///
/// # Examples:
///
/// ```
/// extern crate tui;
/// use tui::widgets::{Widget, Gauge, Block, border};
///
/// fn main() {
///     Gauge::new()
///         .block(*Block::default().borders(border::ALL).title("Progress"))
///         .percent(20);
/// }
/// ```
pub struct Gauge<'a> {
    block: Option<Block<'a>>,
    percent: u16,
    percent_string: String,
    color: Color,
    background_color: Color,
}

impl<'a> Default for Gauge<'a> {
    fn default() -> Gauge<'a> {
        Gauge {
            block: None,
            percent: 0,
            percent_string: String::from("0%"),
            color: Color::Reset,
            background_color: Color::Reset,
        }
    }
}

impl<'a> Gauge<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut Gauge<'a> {
        self.block = Some(block);
        self
    }

    pub fn percent(&mut self, percent: u16) -> &mut Gauge<'a> {
        self.percent = percent;
        self.percent_string = format!("{}%", percent);
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Gauge<'a> {
        self.color = color;
        self
    }

    pub fn background_color(&mut self, color: Color) -> &mut Gauge<'a> {
        self.background_color = color;
        self
    }
}

impl<'a> Widget<'a> for Gauge<'a> {
    fn buffer(&'a self, area: &Rect) -> Buffer<'a> {
        let (mut buf, gauge_area) = match self.block {
            Some(ref b) => (b.buffer(area), b.inner(*area)),
            None => (Buffer::empty(*area), *area),
        };
        if gauge_area.height < 1 {
            return buf;
        } else {
            let margin_x = gauge_area.x - area.x;
            let margin_y = gauge_area.y - area.y;
            // Gauge
            let width = (gauge_area.width * self.percent) / 100;
            for i in 0..width {
                buf.update_cell(margin_x + i,
                                margin_y,
                                " ",
                                self.color,
                                self.background_color);
            }
            // Label
            let len = self.percent_string.len() as u16;
            let middle = gauge_area.width / 2 - len / 2;
            buf.set_string(middle,
                           margin_y,
                           &self.percent_string,
                           self.background_color,
                           self.color);
            let bound = max(middle, min(middle + len, width));
            for i in middle..bound {
                buf.update_colors(margin_x + i, margin_y, self.color, self.background_color);
            }
        }
        buf
    }
}
