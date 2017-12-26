mod block;
mod paragraph;
mod list;
mod gauge;
mod sparkline;
mod chart;
mod barchart;
mod tabs;
mod table;
pub mod canvas;

pub use self::block::Block;
pub use self::paragraph::Paragraph;
pub use self::list::{Item, List, SelectableList};
pub use self::gauge::Gauge;
pub use self::sparkline::Sparkline;
pub use self::chart::{Axis, Chart, Dataset, Marker};
pub use self::barchart::BarChart;
pub use self::tabs::Tabs;
pub use self::table::{Row, Table};

use buffer::Buffer;
use layout::Rect;
use terminal::Terminal;
use backend::Backend;
use style::Color;

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

/// Base requirements for a Widget
pub trait Widget {
    /// Draws the current state of the widget in the given buffer. That the only method required to
    /// implement a custom widget.
    fn draw(&mut self, area: &Rect, buf: &mut Buffer);
    /// Helper method to quickly set the background of all cells inside the specified area.
    fn background(&self, area: &Rect, buf: &mut Buffer, color: Color) {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                buf.get_mut(x, y).set_bg(color);
            }
        }
    }
    /// Helper method that can be chained with a widget's builder methods to render it.
    fn render<B>(&mut self, t: &mut Terminal<B>, area: &Rect)
    where
        Self: Sized,
        B: Backend,
    {
        t.render(self, area);
    }
}
