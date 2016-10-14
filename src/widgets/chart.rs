use std::cmp::min;

use widgets::{Widget, WidgetType, Block};
use buffer::Buffer;
use layout::Rect;
use style::Color;
use symbols;

#[derive(Hash)]
pub struct Chart<'a> {
    block: Option<Block<'a>>,
    fg: Color,
    bg: Color,
    axis: [u64; 2],
    data: &'a [u64],
}

impl<'a> Default for Chart<'a> {
    fn default() -> Chart<'a> {
        Chart {
            block: None,
            fg: Color::White,
            bg: Color::Black,
            axis: [0, 1],
            data: &[],
        }
    }
}

impl<'a> Chart<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut Chart<'a> {
        self.block = Some(block);
        self
    }

    pub fn bg(&mut self, bg: Color) -> &mut Chart<'a> {
        self.bg = bg;
        self
    }

    pub fn fg(&mut self, fg: Color) -> &mut Chart<'a> {
        self.fg = fg;
        self
    }

    pub fn axis(&mut self, axis: [u64; 2]) -> &mut Chart<'a> {
        debug_assert!(self.axis[0] <= self.axis[1]);
        self.axis = axis;
        self
    }

    pub fn data(&mut self, data: &'a [u64]) -> &mut Chart<'a> {
        self.data = data;
        self
    }
}

impl<'a> Widget for Chart<'a> {
    fn buffer(&self, area: &Rect) -> Buffer {
        let (mut buf, chart_area) = match self.block {
            Some(ref b) => (b.buffer(area), b.inner(*area)),
            None => (Buffer::empty(*area), *area),
        };

        if self.axis[1] == 0 {
            return buf;
        }

        let margin_x = chart_area.x - area.x;
        let margin_y = chart_area.y - area.y;
        let max_index = min(chart_area.width as usize, self.data.len());
        for (i, &y) in self.data.iter().take(max_index).enumerate() {
            if y < self.axis[1] {
                let dy = (self.axis[1] - y) * (chart_area.height - 1) as u64 /
                         (self.axis[1] - self.axis[0]);
                buf.update_cell(i as u16 + margin_x, dy as u16 + margin_y, |c| {
                    c.symbol = symbols::DOT;
                    c.fg = self.fg;
                    c.bg = self.bg;
                })
            }
        }
        buf
    }
    fn widget_type(&self) -> WidgetType {
        WidgetType::Chart
    }
}
