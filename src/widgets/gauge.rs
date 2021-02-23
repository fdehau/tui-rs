use crate::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Block, Widget},
};

/// A widget to display a task progress.
///
/// # Examples:
///
/// ```
/// # use tui::widgets::{Widget, Gauge, Block, Borders};
/// # use tui::style::{Style, Color, Modifier};
/// Gauge::default()
///     .block(Block::default().borders(Borders::ALL).title("Progress"))
///     .gauge_style(Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::ITALIC))
///     .percent(20);
/// ```
#[derive(Debug, Default, Clone)]
pub struct Gauge<'a> {
    pub block:        Option<Block<'a>>,
    ratio:            f64,
    /// Cannot be modified directly, only with `relabel()` and `unlabel()`.
    label:            Option<Span<'a>>,
    pub use_unicode:  bool,
    pub style:        Style,
    pub gauge_style:  Style,
}

impl<'a> Gauge<'a> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn percent(mut self, percent: u16) -> Self {
        self.set_percent(percent);
        self
    }

    pub fn set_percent(&mut self, percent: u16) {
        assert!(
            percent <= 100,
            "Percentage should be between 0 and 100 inclusively."
        );
        self.ratio = f64::from(percent) / 100.0;
    }

    /// Sets ratio ([0.0, 1.0]) directly.
    pub fn ratio(mut self, ratio: f64) -> Self {
        self.set_ratio(ratio);
        self
    }

    /// Sets ratio ([0.0, 1.0]) directly.
    pub fn set_ratio(&mut self, ratio: f64) {
        assert!(
            ratio <= 1.0 && ratio >= 0.0,
            "Ratio should be between 0 and 1 inclusively."
        );
        self.ratio = ratio;
    }

    /// Gets the ratio.
    pub fn get_ratio(&self) -> f64 {
        self.ratio
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Span<'a>>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn relabel<T>(&mut self, label: T)
    where
        T: Into<Span<'a>>,
    {
        self.label = Some(label.into());
    }

    pub fn unlabel(&mut self) {
        self.label = None;
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn gauge_style(mut self, style: Style) -> Self {
        self.gauge_style = style;
        self
    }

    pub fn use_unicode(mut self, unicode: bool) -> Self {
        self.use_unicode = unicode;
        self
    }
}

impl<'a> Widget for Gauge<'a> {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let gauge_area = match self.block.take() {
            Some(mut b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };
        buf.set_style(gauge_area, self.gauge_style);
        if gauge_area.height < 1 {
            return;
        }

        let center = gauge_area.height / 2 + gauge_area.top();
        let width = f64::from(gauge_area.width) * self.ratio;
        //go to regular rounding behavior if we're not using unicode blocks
        let end = gauge_area.left()
            + if self.use_unicode {
                width.floor() as u16
            } else {
                width.round() as u16
            };
        // Label
        let ratio = self.ratio;
        //  If label is Some(Span{content: Cow::Owned(…), style: …}),
        //    this allocates memory,
        //  otherwise this clone is only copy by value.
        let label = self
            .label.clone  ( )
            .unwrap_or_else(|| Span::from(format!("{}%", (ratio * 100.0).round())));
        for y in gauge_area.top()..gauge_area.bottom() {
            // Gauge
            for x in gauge_area.left()..end {
                buf.get_mut(x, y).set_symbol(" ");
            }

            //set unicode block
            if self.use_unicode && self.ratio < 1.0 {
                buf.get_mut(end, y)
                    .set_symbol(get_unicode_block(width % 1.0));
            }

            let mut color_end = end;

            if y == center {
                let label_width = label.width() as u16;
                let middle = (gauge_area.width - label_width) / 2 + gauge_area.left();
                buf.set_span(middle, y, &label, gauge_area.right() - middle);
                if self.use_unicode && end >= middle && end < middle + label_width {
                    color_end = gauge_area.left() + (width.round() as u16); //set color on the label to the rounded gauge level
                }
            }

            // Fix colors
            for x in gauge_area.left()..color_end {
                buf.get_mut(x, y)
                    .set_fg(self.gauge_style.bg.unwrap_or(Color::Reset))
                    .set_bg(self.gauge_style.fg.unwrap_or(Color::Reset));
            }
        }
    }
}

fn get_unicode_block<'a>(frac: f64) -> &'a str {
    match (frac * 8.0).round() as u16 {
        //get how many eighths the fraction is closest to
        1 => symbols::block::ONE_EIGHTH,
        2 => symbols::block::ONE_QUARTER,
        3 => symbols::block::THREE_EIGHTHS,
        4 => symbols::block::HALF,
        5 => symbols::block::FIVE_EIGHTHS,
        6 => symbols::block::THREE_QUARTERS,
        7 => symbols::block::SEVEN_EIGHTHS,
        8 => symbols::block::FULL,
        _ => " ",
    }
}

/// A compact widget to display a task progress over a single line.
///
/// # Examples:
///
/// ```
/// # use tui::widgets::{Widget, LineGauge, Block, Borders};
/// # use tui::style::{Style, Color, Modifier};
/// # use tui::symbols;
/// LineGauge::default()
///     .block(Block::default().borders(Borders::ALL).title("Progress"))
///     .gauge_style(Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::BOLD))
///     .line_set(symbols::line::THICK)
///     .ratio(0.4);
/// ```
#[derive(Clone, Debug)]
pub struct LineGauge<'a> {
    pub block:        Option<Block<'a>>,
    /// Cannot be modified directly, only with `set_percent()` and `set_ration()`.
    ratio:            f64,
    pub label:        Option<Spans<'a>>,
    pub line_set:     symbols::line::Set,
    pub style:        Style,
    pub gauge_style:  Style,
}

impl<'a> Default for LineGauge<'a> {
    fn default() -> Self {
        Self {
            block: None,
            ratio: 0.0,
            label: None,
            style: Style::default(),
            line_set: symbols::line::NORMAL,
            gauge_style: Style::default(),
        }
    }
}

impl<'a> LineGauge<'a> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn percent(mut self, percent: u16) -> Self {
        self.set_percent(percent);
        self
    }

    pub fn set_percent(&mut self, percent: u16) {
        assert!(
            percent <= 100,
            "Percentage should be between 0 and 100 inclusively."
        );
        self.ratio = f64::from(percent) / 100.0;
    }

    /// Sets ratio ([0.0, 1.0]) directly.
    pub fn ratio(mut self, ratio: f64) -> Self {
        self.set_ratio(ratio);
        self
    }

    /// Sets ratio ([0.0, 1.0]) directly.
    pub fn set_ratio(&mut self, ratio: f64) {
        assert!(
            ratio <= 1.0 && ratio >= 0.0,
            "Ratio should be between 0 and 1 inclusively."
        );
        self.ratio = ratio;
    }

    pub fn line_set(mut self, set: symbols::line::Set) -> Self {
        self.line_set = set;
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Spans<'a>>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn gauge_style(mut self, style: Style) -> Self {
        self.gauge_style = style;
        self
    }
}

impl<'a> Widget for LineGauge<'a> {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let gauge_area = match self.block.take() {
            Some(mut b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if gauge_area.height < 1 {
            return;
        }

        let ratio = self.ratio;
        //  If label is Some(Span{content: Cow::Owned(…), style: …}),
        //    this allocates memory,
        //  otherwise this clone is only copy by value.
        let label = self
            .label.clone()
            .unwrap_or_else(|| Spans::from(format!("{:.0}%", ratio * 100.0)));
        let (col, row) = buf.set_spans(
            gauge_area.left(),
            gauge_area.top(),
            &label,
            gauge_area.width,
        );
        let start = col + 1;
        if start >= gauge_area.right() {
            return;
        }

        let end = start
            + (f64::from(gauge_area.right().saturating_sub(start)) * self.ratio).floor() as u16;
        for col in start..end {
            buf.get_mut(col, row)
                .set_symbol(self.line_set.horizontal)
                .set_style(Style {
                    fg: self.gauge_style.fg,
                    bg: None,
                    add_modifier: self.gauge_style.add_modifier,
                    sub_modifier: self.gauge_style.sub_modifier,
                });
        }
        for col in end..gauge_area.right() {
            buf.get_mut(col, row)
                .set_symbol(self.line_set.horizontal)
                .set_style(Style {
                    fg: self.gauge_style.bg,
                    bg: None,
                    add_modifier: self.gauge_style.add_modifier,
                    sub_modifier: self.gauge_style.sub_modifier,
                });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn gauge_invalid_percentage() {
        Gauge::default().percent(110);
    }

    #[test]
    #[should_panic]
    fn gauge_invalid_ratio_upper_bound() {
        Gauge::default().ratio(1.1);
    }

    #[test]
    #[should_panic]
    fn gauge_invalid_ratio_lower_bound() {
        Gauge::default().ratio(-0.5);
    }
}
