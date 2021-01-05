//! `widgets` is a collection of types that implement [`Widget`].
//!
//! All widgets are implemented using the builder pattern and are consumable objects. They are not
//! meant to be stored but used as *commands* to draw common figures in the UI.
//!
//! The available widgets are:
//! - [`Block`]
//! - [`Tabs`]
//! - [`List`]
//! - [`Table`]
//! - [`Paragraph`]
//! - [`Chart`]
//! - [`BarChart`]
//! - [`Gauge`]
//! - [`Sparkline`]
//! - [`Clear`]

mod barchart;
mod block;
pub mod canvas;
mod chart;
mod clear;
mod gauge;
mod list;
mod paragraph;
mod reflow;
mod sparkline;
mod table;
mod tabs;

pub use self::barchart::BarChart;
pub use self::block::{Block, BorderType};
pub use self::chart::{Axis, Chart, Dataset, GraphType};
pub use self::clear::Clear;
pub use self::gauge::{Gauge, LineGauge};
pub use self::list::{List, ListItem, ListState};
pub use self::paragraph::{Paragraph, Wrap};
pub use self::sparkline::Sparkline;
pub use self::table::{Cell, Row, Table, TableState};
pub use self::tabs::Tabs;

use crate::{buffer::Buffer, layout::Rect};
use bitflags::bitflags;

bitflags! {
    /// Bitflags that can be composed to set the visible borders essentially on the block widget.
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
    /// State stores everything that need to be saved between draw calls in order for the widget to
    /// implement certain UI patterns.
    ///
    /// For example, the [`List`] widget can highlight the item currently selected. This can be
    /// translated in an offset, which is the number of elements to skip in order to have the
    /// selected item within the viewport currently allocated to this widget. If the widget had
    /// only access to the index of the selected item, it could only implement the following
    /// behavior: whenever the selected item is out of the viewport scroll to a predefined position
    /// (making the selected item the last viewable item or the one in the middle for example).
    /// Nonetheless, if the widget has access to the last computed offset then it can implement a
    /// natural scrolling experience where the last offset is reused until the selected item is out
    /// of the viewport.
    type State;
    /// Render the widget in the internal buffer. That the only method required to implement a
    /// custom widget.
    fn render(self, ctx: &mut RenderContext<Self::State>);
}

/// RenderContext is a set of dependencies that may be used when a widget is rendered.
pub struct RenderContext<'a, S> {
    /// Area where the widget is rendered.
    pub area: Rect,
    /// Buffer where the drawing operations will be temporarily registered.
    pub buffer: &'a mut Buffer,
    /// Internal state associated with the widget.
    pub state: &'a mut S,
}
