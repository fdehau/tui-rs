use widgets::{Widget, WidgetType, Block};
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
#[derive(Hash)]
pub struct Gauge<'a> {
    block: Option<Block<'a>>,
    percent: u16,
    fg: Color,
    bg: Color,
}

impl<'a> Gauge<'a> {
    pub fn new() -> Gauge<'a> {
        Gauge {
            block: None,
            percent: 0,
            bg: Color::White,
            fg: Color::Black,
        }
    }

    pub fn block(&'a mut self, block: Block<'a>) -> &mut Gauge<'a> {
        self.block = Some(block);
        self
    }

    pub fn percent(&mut self, percent: u16) -> &mut Gauge<'a> {
        self.percent = percent;
        self
    }

    pub fn bg(&mut self, bg: Color) -> &mut Gauge<'a> {
        self.bg = bg;
        self
    }

    pub fn fg(&mut self, fg: Color) -> &mut Gauge<'a> {
        self.fg = fg;
        self
    }
}

impl<'a> Widget for Gauge<'a> {
    fn buffer(&self, area: &Rect) -> Buffer {
        let (mut buf, gauge_area) = match self.block {
            Some(ref b) => (b.buffer(area), b.inner(*area)),
            None => (Buffer::empty(*area), *area),
        };
        if gauge_area.height < 1 {
            return buf;
        } else {
            let margin_x = gauge_area.x - area.x;
            let margin_y = gauge_area.y - area.y;
            // Label
            let percent_string = format!("{}%", self.percent);
            let len = percent_string.len() as u16;
            let middle = gauge_area.width / 2 - len / 2;
            buf.set_string(middle, margin_y, &percent_string, self.bg, self.fg);
            // Gauge
            let width = (gauge_area.width * self.percent) / 100;
            for i in 0..width {
                buf.set_bg(margin_x + i, margin_y, self.bg);
                buf.set_fg(margin_x + i, margin_y, self.fg);
            }
        }
        buf
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Gauge
    }
}
