use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use layout::Rect;
use style::{Color, Style};
use widgets::{Block, Widget};

/// A widget to display a task progress.
///
/// # Examples:
///
/// ```
/// # extern crate tui;
/// # use tui::widgets::{Widget, Gauge, Block, Borders};
/// # use tui::style::{Style, Color, Modifier};
/// # fn main() {
/// Gauge::default()
///     .block(Block::default().borders(Borders::ALL).title("Progress"))
///     .style(Style::default().fg(Color::White).bg(Color::Black).modifier(Modifier::Italic))
///     .percent(20);
/// # }
/// ```
pub struct Gauge<'a> {
    block: Option<Block<'a>>,
    percent: u16,
    label: Option<&'a str>,
    style: Style,
}

impl<'a> Default for Gauge<'a> {
    fn default() -> Gauge<'a> {
        Gauge {
            block: None,
            percent: 0,
            label: None,
            style: Default::default(),
        }
    }
}

impl<'a> Gauge<'a> {
    pub fn block(mut self, block: Block<'a>) -> Gauge<'a> {
        self.block = Some(block);
        self
    }

    pub fn percent(mut self, percent: u16) -> Gauge<'a> {
        self.percent = percent;
        self
    }

    pub fn label(mut self, string: &'a str) -> Gauge<'a> {
        self.label = Some(string);
        self
    }

    pub fn style(mut self, style: Style) -> Gauge<'a> {
        self.style = style;
        self
    }
}

impl<'a> Widget for Gauge<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let gauge_area = match self.block {
            Some(ref mut b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => area,
        };
        if gauge_area.height < 1 {
            return;
        }

        if self.style.bg != Color::Reset {
            self.background(&gauge_area, buf, self.style.bg);
        }

        let center = gauge_area.height / 2 + gauge_area.top();
        let width = (gauge_area.width * self.percent) / 100;
        let end = gauge_area.left() + width;
        for y in gauge_area.top()..gauge_area.bottom() {
            // Gauge
            for x in gauge_area.left()..end {
                buf.get_mut(x, y).set_symbol(" ");
            }

            if y == center {
                // Label
                let precent_label = format!("{}%", self.percent);
                let label = self.label.unwrap_or(&precent_label);
                let label_width = label.width() as u16;
                let middle = (gauge_area.width - label_width) / 2 + gauge_area.left();
                buf.set_string(middle, y, label, self.style);
            }

            // Fix colors
            for x in gauge_area.left()..end {
                buf.get_mut(x, y)
                    .set_fg(self.style.bg)
                    .set_bg(self.style.fg);
            }
        }
    }
}
