use bitflags::bitflags;
use std::borrow::Cow;

mod block;
mod list;
mod paragraph;
mod reflow;
mod table;
mod tabs;

pub use self::block::Block;
pub use self::list::{List, SelectableList};
pub use self::paragraph::Paragraph;
pub use self::table::{Row, Table};
pub use self::tabs::Tabs;

use crate::backend::Backend;
use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::style::{Color, Style};
use crate::terminal::Frame;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Text<'b> {
    Raw(Cow<'b, str>),
    Styled(Cow<'b, str>, Style),
}

impl<'b> Text<'b> {
    pub fn raw<D: Into<Cow<'b, str>>>(data: D) -> Text<'b> {
        Text::Raw(data.into())
    }

    pub fn styled<D: Into<Cow<'b, str>>>(data: D, style: Style) -> Text<'b> {
        Text::Styled(data.into(), style)
    }
}

/// Base requirements for a Widget
pub trait Widget {
    /// Draws the current state of the widget in the given buffer. That the only method required to
    /// implement a custom widget.
    fn draw(&mut self, buf: &mut Buffer);
    /// Helper method to quickly set the background of all cells inside the specified area.
    fn background(&self, buf: &mut Buffer, color: Color) {
        for y in self.top()..self.bottom() {
            for x in self.left()..self.right() {
                buf.get_mut(x, y).set_bg(color);
            }
        }
    }
    /// Helper method that can be chained with a widget's builder methods to render it.
    fn render<B>(&mut self, f: &mut Frame<B>)
    where
        Self: Sized,
        B: Backend,
    {
        f.render(self);
    }

    fn get_area(&self) -> Rect;

    fn top(&self) -> u16 {
        self.get_area().top()
    }
    fn bottom(&self) -> u16 {
        self.get_area().bottom()
    }
    fn left(&self) -> u16 {
        self.get_area().left()
    }
    fn right(&self) -> u16{
        self.get_area().right()
    }
}
