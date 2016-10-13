use std::cmp::min;

use layout::Rect;
use buffer::Buffer;
use widgets::{Widget, WidgetType, Block};
use style::Color;
use symbols::bar;

#[derive(Hash)]
pub struct Sparkline<'a> {
    block: Option<Block<'a>>,
    fg: Color,
    bg: Color,
    data: Vec<u64>,
    max: Option<u64>,
}

impl<'a> Sparkline<'a> {
    pub fn new() -> Sparkline<'a> {
        Sparkline {
            block: None,
            fg: Color::White,
            bg: Color::Black,
            data: Vec::new(),
            max: None,
        }
    }

    pub fn block(&mut self, block: Block<'a>) -> &mut Sparkline<'a> {
        self.block = Some(block);
        self
    }

    pub fn fg(&mut self, fg: Color) -> &mut Sparkline<'a> {
        self.fg = fg;
        self
    }

    pub fn bg(&mut self, bg: Color) -> &mut Sparkline<'a> {
        self.bg = bg;
        self
    }


    pub fn data(&mut self, data: &[u64]) -> &mut Sparkline<'a> {
        self.data = data.to_vec();
        self
    }

    pub fn max(&mut self, max: u64) -> &mut Sparkline<'a> {
        self.max = Some(max);
        self
    }
}

impl<'a> Widget for Sparkline<'a> {
    fn buffer(&self, area: &Rect) -> Buffer {
        let (mut buf, spark_area) = match self.block {
            Some(ref b) => (b.buffer(area), b.inner(*area)),
            None => (Buffer::empty(*area), *area),
        };
        if spark_area.height < 1 {
            return buf;
        } else {
            let margin_x = spark_area.x - area.x;
            let margin_y = spark_area.y - area.y;
            let max = match self.max {
                Some(v) => v,
                None => *self.data.iter().max().unwrap_or(&1u64),
            };
            let max_index = min(spark_area.width as usize, self.data.len());
            let mut data = self.data
                .iter()
                .map(|e| e * spark_area.height as u64 * 8 / max)
                .collect::<Vec<u64>>();
            for j in (0..spark_area.height).rev() {
                let mut line = String::with_capacity(max_index);
                for i in 0..max_index {
                    line.push(match data[i] {
                        0 => ' ',
                        1 => bar::ONE_EIGHTH,
                        2 => bar::ONE_QUATER,
                        3 => bar::THREE_EIGHTHS,
                        4 => bar::HALF,
                        5 => bar::FIVE_EIGHTHS,
                        6 => bar::THREE_EIGHTHS,
                        7 => bar::THREE_QUATERS,
                        _ => bar::FULL,
                    });
                    if data[i] > 8 {
                        data[i] -= 8;
                    } else {
                        data[i] = 0;
                    }
                }
                buf.set_string(margin_x, margin_y + j, &line, self.fg, self.bg);
            }
        }
        buf
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Sparkline
    }
}
