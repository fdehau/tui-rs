use std::cmp::min;

use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::style::Style;
use crate::symbols::bar;
use crate::widgets::{Block, Widget};

/// Widget to render a sparkline over one or more lines.
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, Sparkline};
/// # use tui::style::{Style, Color};
/// # fn main() {
/// Sparkline::default()
///     .block(Block::default().title("Sparkline").borders(Borders::ALL))
///     .data(&[0, 2, 3, 4, 1, 4, 10])
///     .max(5)
///     .style(Style::default().fg(Color::Red).bg(Color::White));
/// # }
/// ```
pub struct Sparkline<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Widget style
    style: Style,
    /// A slice of the data to display
    data: &'a [u64],
    /// The maximum value to take to compute the maximum bar height (if nothing is specified, the
    /// widget uses the max of the dataset)
    max: Option<u64>,
    /// If true, draws a baseline of `bar::ONE_EIGHTH` spanning the bottom of the sparkline graph
    show_baseline: bool,
}

impl<'a> Default for Sparkline<'a> {
    fn default() -> Sparkline<'a> {
        Sparkline {
            block: None,
            style: Default::default(),
            data: &[],
            max: None,
            show_baseline: false,
        }
    }
}

impl<'a> Sparkline<'a> {
    pub fn block(mut self, block: Block<'a>) -> Sparkline<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Sparkline<'a> {
        self.style = style;
        self
    }

    pub fn data(mut self, data: &'a [u64]) -> Sparkline<'a> {
        self.data = data;
        self
    }

    pub fn max(mut self, max: u64) -> Sparkline<'a> {
        self.max = Some(max);
        self
    }

    pub fn show_baseline(mut self, show_baseline: bool) -> Sparkline<'a> {
        self.show_baseline = show_baseline;
        self
    }
}

impl<'a> Widget for Sparkline<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let spark_area = match self.block {
            Some(ref mut b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => area,
        };

        if spark_area.height < 1 {
            return;
        }

        if self.show_baseline {
            for i in spark_area.left()..spark_area.right() {
                buf.get_mut(i, spark_area.bottom() - 1)
                    .set_symbol(bar::ONE_EIGHTH)
                    .set_fg(self.style.fg)
                    .set_bg(self.style.bg);
            }
        }

        let max = match self.max {
            Some(v) => v,
            None => *self.data.iter().max().unwrap_or(&1u64),
        };
        let max_index = min(spark_area.width as usize, self.data.len());
        let mut data = self
            .data
            .iter()
            .take(max_index)
            .map(|e| {
                if max != 0 {
                    e * u64::from(spark_area.height) * 8 / max
                } else {
                    0
                }
            })
            .collect::<Vec<u64>>();
        for j in (0..spark_area.height).rev() {
            for (i, d) in data.iter_mut().enumerate() {
                let symbol = match *d {
                    0 => {
                        if self.show_baseline && j == spark_area.height - 1 {
                            bar::ONE_EIGHTH
                        } else {
                            " "
                        }
                    }
                    1 => bar::ONE_EIGHTH,
                    2 => bar::ONE_QUARTER,
                    3 => bar::THREE_EIGHTHS,
                    4 => bar::HALF,
                    5 => bar::FIVE_EIGHTHS,
                    6 => bar::THREE_QUARTERS,
                    7 => bar::SEVEN_EIGHTHS,
                    _ => bar::FULL,
                };
                buf.get_mut(spark_area.left() + i as u16, spark_area.top() + j)
                    .set_symbol(symbol)
                    .set_fg(self.style.fg)
                    .set_bg(self.style.bg);

                if *d > 8 {
                    *d -= 8;
                } else {
                    *d = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_not_panic_if_max_is_zero() {
        let mut widget = Sparkline::default().data(&[0, 0, 0]);
        let area = Rect::new(0, 0, 3, 1);
        let mut buffer = Buffer::empty(area);
        widget.draw(area, &mut buffer);
    }

    #[test]
    fn it_does_not_panic_if_max_is_set_to_zero() {
        let mut widget = Sparkline::default().data(&[0, 1, 2]).max(0);
        let area = Rect::new(0, 0, 3, 1);
        let mut buffer = Buffer::empty(area);
        widget.draw(area, &mut buffer);
    }
}
