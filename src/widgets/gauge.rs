use unicode_width::UnicodeWidthStr;

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
    color: Color,
    background_color: Color,
}

impl<'a> Default for Gauge<'a> {
    fn default() -> Gauge<'a> {
        Gauge {
            block: None,
            percent: 0,
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

impl<'a> Widget for Gauge<'a> {
    fn buffer(&self, area: &Rect, buf: &mut Buffer) {
        let gauge_area = match self.block {
            Some(ref b) => {
                b.buffer(area, buf);
                b.inner(area)
            }
            None => *area,
        };
        if gauge_area.height < 1 {
            return;
        } else {
            // Gauge
            let width = (gauge_area.width * self.percent) / 100;
            let end = gauge_area.left() + width;

            for x in gauge_area.left()..end {
                buf.set_symbol(x, gauge_area.top(), " ");
            }

            // Label
            let label = format!("{}%", self.percent);
            let label_width = label.width() as u16;
            let middle = (gauge_area.width - label_width) / 2 + gauge_area.left();
            buf.set_string(middle,
                           gauge_area.top(),
                           &label,
                           self.color,
                           self.background_color);

            for x in gauge_area.left()..end {
                buf.update_colors(x, gauge_area.top(), self.background_color, self.color);
            }
        }
    }
}
