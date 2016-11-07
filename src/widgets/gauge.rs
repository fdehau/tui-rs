use unicode_width::UnicodeWidthStr;

use widgets::{Widget, Block};
use buffer::Buffer;
use style::{Style, Color};
use layout::Rect;

/// A widget to display a task progress.
///
/// # Examples:
///
/// ```
/// # extern crate tui;
/// # use tui::widgets::{Widget, Gauge, Block, border};
/// # use tui::style::{Style, Color, Modifier};
/// # fn main() {
/// Gauge::default()
///     .block(Block::default().borders(border::ALL).title("Progress"))
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
    pub fn block(&mut self, block: Block<'a>) -> &mut Gauge<'a> {
        self.block = Some(block);
        self
    }

    pub fn percent(&mut self, percent: u16) -> &mut Gauge<'a> {
        self.percent = percent;
        self
    }

    pub fn label(&mut self, string: &'a str) -> &mut Gauge<'a> {
        self.label = Some(string);
        self
    }

    pub fn style(&mut self, style: Style) -> &mut Gauge<'a> {
        self.style = style;
        self
    }
}

impl<'a> Widget for Gauge<'a> {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {
        let gauge_area = match self.block {
            Some(ref b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };
        if gauge_area.height < 1 {
            return;
        }

        if self.style.bg != Color::Reset {
            self.background(&gauge_area, buf, self.style.bg);
        }

        // Gauge
        let width = (gauge_area.width * self.percent) / 100;
        let end = gauge_area.left() + width;

        for x in gauge_area.left()..end {
            buf.get_mut(x, gauge_area.top())
                .set_symbol(" ");
        }

        // Label
        let precent_label = format!("{}%", self.percent);
        let label = self.label.unwrap_or(&precent_label);
        let label_width = label.width() as u16;
        let middle = (gauge_area.width - label_width) / 2 + gauge_area.left();
        buf.set_string(middle, gauge_area.top(), &label, &self.style);

        for x in gauge_area.left()..end {
            buf.get_mut(x, gauge_area.top()).set_fg(self.style.bg).set_bg(self.style.fg);
        }
    }
}
