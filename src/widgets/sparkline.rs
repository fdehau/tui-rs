use std::cmp::min;

use layout::Rect;
use buffer::Buffer;
use widgets::{Widget, Block};
use style::Color;
use symbols::bar;

pub struct Sparkline<'a> {
    block: Option<Block<'a>>,
    color: Color,
    background_color: Color,
    data: &'a [u64],
    max: Option<u64>,
}

impl<'a> Default for Sparkline<'a> {
    fn default() -> Sparkline<'a> {
        Sparkline {
            block: None,
            color: Color::Reset,
            background_color: Color::Reset,
            data: &[],
            max: None,
        }
    }
}

impl<'a> Sparkline<'a> {
    pub fn block(&mut self, block: Block<'a>) -> &mut Sparkline<'a> {
        self.block = Some(block);
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Sparkline<'a> {
        self.color = color;
        self
    }

    pub fn background_color(&mut self, color: Color) -> &mut Sparkline<'a> {
        self.background_color = color;
        self
    }


    pub fn data(&mut self, data: &'a [u64]) -> &mut Sparkline<'a> {
        self.data = data;
        self
    }

    pub fn max(&mut self, max: u64) -> &mut Sparkline<'a> {
        self.max = Some(max);
        self
    }
}

impl<'a> Widget for Sparkline<'a> {
    fn buffer(&self, area: &Rect, buf: &mut Buffer) {
        let spark_area = match self.block {
            Some(ref b) => {
                b.buffer(area, buf);
                b.inner(area)
            }
            None => *area,
        };
        if spark_area.height < 1 {
            return;
        } else {
            let max = match self.max {
                Some(v) => v,
                None => *self.data.iter().max().unwrap_or(&1u64),
            };
            let max_index = min(spark_area.width as usize, self.data.len());
            let mut data = self.data
                .iter()
                .take(max_index)
                .map(|e| e * spark_area.height as u64 * 8 / max)
                .collect::<Vec<u64>>();
            for j in (0..spark_area.height).rev() {
                for (i, d) in data.iter_mut().enumerate() {
                    let symbol = match *d {
                        0 => " ",
                        1 => bar::ONE_EIGHTH,
                        2 => bar::ONE_QUATER,
                        3 => bar::THREE_EIGHTHS,
                        4 => bar::HALF,
                        5 => bar::FIVE_EIGHTHS,
                        6 => bar::THREE_QUATERS,
                        7 => bar::SEVEN_EIGHTHS,
                        _ => bar::FULL,
                    };
                    buf.update_cell(spark_area.left() + i as u16,
                                    spark_area.top() + j,
                                    symbol,
                                    self.color,
                                    self.background_color);

                    if *d > 8 {
                        *d -= 8;
                    } else {
                        *d = 0;
                    }
                }
            }
        }
    }
}
