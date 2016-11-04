use std::cmp::{min, max};

use unicode_width::UnicodeWidthStr;

use widgets::{Widget, Block};
use buffer::Buffer;
use layout::Rect;
use style::Color;
use symbols::bar;

/// Display multiple bars in a single widgets
///
/// # Examples
///
/// ```
/// # extern crate tui;
/// # use tui::widgets::{Block, border, BarChart};
/// # use tui::style::Color;
/// # fn main() {
/// BarChart::default()
///     .block(Block::default().title("BarChart").borders(border::ALL))
///     .bar_width(3)
///     .bar_gap(1)
///     .bar_color(Color::Yellow)
///     .value_color(Color::Red)
///     .label_color(Color::White)
///     .background_color(Color::Black)
///     .data(&[("B0", 0), ("B1", 2), ("B2", 4), ("B3", 3)])
///     .max(4);
/// # }
/// ```
pub struct BarChart<'a> {
    /// Block to wrap the widget in
    block: Option<Block<'a>>,
    /// The width of each bar
    bar_width: u16,
    /// The gap between each bar
    bar_gap: u16,
    /// Color of the bars
    bar_color: Color,
    /// Color of the values printed at the bottom of each bar
    value_color: Color,
    /// Color of the labels printed under each bar
    label_color: Color,
    /// Background color for the widget
    background_color: Color,
    /// Slice of (label, value) pair to plot on the chart
    data: &'a [(&'a str, u64)],
    /// Value necessary for a bar to reach the maximum height (if no value is specified,
    /// the maximum value in the data is taken as reference)
    max: Option<u64>,
    /// Values to display on the bar (computed when the data is passed to the widget)
    values: Vec<String>,
}

impl<'a> Default for BarChart<'a> {
    fn default() -> BarChart<'a> {
        BarChart {
            block: None,
            max: None,
            data: &[],
            values: Vec::new(),
            bar_width: 1,
            bar_gap: 1,
            bar_color: Color::Reset,
            value_color: Color::Reset,
            label_color: Color::Reset,
            background_color: Color::Reset,
        }
    }
}

impl<'a> BarChart<'a> {
    pub fn data(&'a mut self, data: &'a [(&'a str, u64)]) -> &mut BarChart<'a> {
        self.data = data;
        self.values = Vec::with_capacity(self.data.len());
        for &(_, v) in self.data {
            self.values.push(format!("{}", v));
        }
        self
    }

    pub fn block(&'a mut self, block: Block<'a>) -> &mut BarChart<'a> {
        self.block = Some(block);
        self
    }
    pub fn max(&'a mut self, max: u64) -> &mut BarChart<'a> {
        self.max = Some(max);
        self
    }

    pub fn bar_width(&'a mut self, width: u16) -> &mut BarChart<'a> {
        self.bar_width = width;
        self
    }
    pub fn bar_gap(&'a mut self, gap: u16) -> &mut BarChart<'a> {
        self.bar_gap = gap;
        self
    }
    pub fn bar_color(&'a mut self, color: Color) -> &mut BarChart<'a> {
        self.bar_color = color;
        self
    }
    pub fn value_color(&'a mut self, color: Color) -> &mut BarChart<'a> {
        self.value_color = color;
        self
    }
    pub fn label_color(&'a mut self, color: Color) -> &mut BarChart<'a> {
        self.label_color = color;
        self
    }
    pub fn background_color(&'a mut self, color: Color) -> &mut BarChart<'a> {
        self.background_color = color;
        self
    }
}

impl<'a> Widget for BarChart<'a> {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {
        let chart_area = match self.block {
            Some(ref b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        if chart_area.height < 2 {
            return;
        }

        if self.background_color != Color::Reset {
            self.background(&chart_area, buf, self.background_color);
        }

        let max = self.max.unwrap_or(self.data.iter().fold(0, |acc, &(_, v)| max(v, acc)));
        let max_index = min((chart_area.width / (self.bar_width + self.bar_gap)) as usize,
                            self.data.len());
        let mut data = self.data
            .iter()
            .take(max_index)
            .map(|&(l, v)| (l, v * chart_area.height as u64 * 8 / max))
            .collect::<Vec<(&str, u64)>>();
        for j in (0..chart_area.height - 1).rev() {
            for (i, d) in data.iter_mut().enumerate() {
                let symbol = match d.1 {
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

                for x in 0..self.bar_width {
                    buf.set_cell(chart_area.left() + i as u16 * (self.bar_width + self.bar_gap) +
                                 x,
                                 chart_area.top() + j,
                                 symbol,
                                 self.bar_color,
                                 self.background_color);
                }

                if d.1 > 8 {
                    d.1 -= 8;
                } else {
                    d.1 = 0;
                }

            }
        }

        for (i, &(label, value)) in self.data.iter().take(max_index).enumerate() {
            if value != 0 {
                let value_label = &self.values[i];
                let width = value_label.width() as u16;
                if width < self.bar_width {
                    buf.set_string(chart_area.left() + i as u16 * (self.bar_width + self.bar_gap) +
                                   (self.bar_width - width) / 2,
                                   chart_area.bottom() - 2,
                                   value_label,
                                   self.value_color,
                                   self.bar_color);
                }
            }
            buf.set_stringn(chart_area.left() + i as u16 * (self.bar_width + self.bar_gap),
                            chart_area.bottom() - 1,
                            label,
                            self.bar_width as usize,
                            self.label_color,
                            self.background_color);
        }
    }
}
