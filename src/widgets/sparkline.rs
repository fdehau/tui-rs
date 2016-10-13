use std::cmp::min;

use layout::Rect;
use buffer::Buffer;
use widgets::{Widget, WidgetType, Block};
use style::Color;
use symbols::bar;

#[derive(Hash)]
pub struct Sparkline<'a> {
    block: Option<Block<'a>>,
    color: Color,
    data: Vec<u64>,
}

impl<'a> Sparkline<'a> {
    pub fn new() -> Sparkline<'a> {
        Sparkline {
            block: None,
            color: Color::White,
            data: Vec::new(),
        }
    }

    pub fn block(&mut self, block: Block<'a>) -> &mut Sparkline<'a> {
        self.block = Some(block);
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Sparkline<'a> {
        self.color = color;
        self
    }

    pub fn data(&mut self, data: &[u64]) -> &mut Sparkline<'a> {
        self.data = data.to_vec();
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
            match self.data.iter().max() {
                Some(max_value) => {
                    let max_index = min(spark_area.width as usize, self.data.len());
                    let line = self.data
                        .iter()
                        .take(max_index)
                        .filter_map(|e| {
                            let value = e * 8 / max_value;
                            match value {
                                0 => Some(' '),
                                1 => Some(bar::ONE_EIGHTH),
                                2 => Some(bar::ONE_QUATER),
                                3 => Some(bar::THREE_EIGHTHS),
                                4 => Some(bar::HALF),
                                5 => Some(bar::FIVE_EIGHTHS),
                                6 => Some(bar::THREE_EIGHTHS),
                                7 => Some(bar::THREE_QUATERS),
                                8 => Some(bar::FULL),
                                _ => None,
                            }
                        })
                        .collect::<String>();
                    buf.set_string(margin_x, margin_y, &line);
                }
                None => {}
            }
        }
        buf
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Sparkline
    }
}
