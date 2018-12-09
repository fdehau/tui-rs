use bitflags::bitflags;
use either::Either;
use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;

mod barchart;
mod block;
pub mod canvas;
mod chart;
mod gauge;
mod list;
mod paragraph;
mod reflow;
mod sparkline;
mod table;
mod tabs;

pub use self::barchart::BarChart;
pub use self::block::Block;
pub use self::chart::{Axis, Chart, Dataset, Marker};
pub use self::gauge::Gauge;
pub use self::list::List;
pub use self::paragraph::Paragraph;
pub use self::sparkline::Sparkline;
pub use self::table::{Row, Table};
pub use self::tabs::Tabs;

use crate::backend::Backend;
use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::style::{Color, Style};
use crate::terminal::Frame;

/// Bitflags that can be composed to set the visible borders essentially on the block widget.
bitflags! {
    pub struct Borders: u32 {
        /// Show no border (default)
        const NONE  = 0b0000_0001;
        /// Show the top border
        const TOP   = 0b0000_0010;
        /// Show the right border
        const RIGHT = 0b0000_0100;
        /// Show the bottom border
        const BOTTOM = 0b000_1000;
        /// Show the left border
        const LEFT = 0b0001_0000;
        /// Show all borders
        const ALL = Self::TOP.bits | Self::RIGHT.bits | Self::BOTTOM.bits | Self::LEFT.bits;
    }
}

pub enum Text<'b> {
    Raw(Cow<'b, str>),
    Styled(Vec<(Cow<'b, str>, Style)>),
}

impl<'b> Text<'b> {
    pub fn raw<D>(data: D) -> Text<'b>
    where
        D: Into<Cow<'b, str>>,
    {
        Text::Raw(data.into())
    }

    pub fn styled<D>(data: D, style: Style) -> Text<'b>
    where
        D: Into<Cow<'b, str>>,
    {
        Text::Styled(vec![(data.into(), style)])
    }

    pub fn with_styles<D>(items: Vec<(D, Style)>) -> Text<'b>
    where
        D: Into<Cow<'b, str>>,
    {
        Text::Styled(items.into_iter().map(|i| (i.0.into(), i.1)).collect())
    }

    pub fn height(&self) -> u16 {
        match self {
            Text::Raw(ref d) => d.lines().count() as u16,
            Text::Styled(items) => {
                items
                    .iter()
                    .flat_map(|i| i.0.chars())
                    .filter(|i| i == &'\n')
                    .count() as u16
                    + 1
            }
        }
    }

    pub fn styled_graphemes(
        &self,
        default_style: Style,
    ) -> Either<impl Iterator<Item = (&str, Style)>, impl Iterator<Item = (&str, Style)>> {
        match self {
            Text::Raw(d) => Either::Left(
                UnicodeSegmentation::graphemes(&**d, true).map(move |g| (g, default_style)),
            ),
            Text::Styled(items) => Either::Right(items.iter().flat_map(|item| {
                UnicodeSegmentation::graphemes(&*item.0, true).map(move |g| (g, item.1))
            })),
        }
    }
}

/// Base requirements for a Widget
pub trait Widget {
    /// Draws the current state of the widget in the given buffer. That the only method required to
    /// implement a custom widget.
    fn draw(&mut self, area: Rect, buf: &mut Buffer);
    /// Helper method to quickly set the background of all cells inside the specified area.
    fn background(&self, area: Rect, buf: &mut Buffer, color: Color) {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                buf.get_mut(x, y).set_bg(color);
            }
        }
    }
    /// Helper method that can be chained with a widget's builder methods to render it.
    fn render<B>(&mut self, f: &mut Frame<B>, area: Rect)
    where
        Self: Sized,
        B: Backend,
    {
        f.render(self, area);
    }
}
