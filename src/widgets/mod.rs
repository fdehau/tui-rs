mod block;
mod text;
mod list;
mod gauge;
mod sparkline;
mod chart;

pub use self::block::Block;
pub use self::text::Text;
pub use self::list::List;
pub use self::gauge::Gauge;
pub use self::sparkline::Sparkline;
pub use self::chart::Chart;

use std::hash::Hash;

use util::hash;
use buffer::{Buffer, Cell};
use layout::{Rect, Tree, Leaf};
use style::Color;

#[allow(dead_code)]
enum Line {
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    VerticalLeft,
    VerticalRight,
    HorizontalDown,
    HorizontalUp,
}

pub mod border {
    bitflags! {
        pub flags Flags: u32 {
            const NONE  = 0b00000001,
            const TOP   = 0b00000010,
            const RIGHT = 0b00000100,
            const BOTTOM = 0b0001000,
            const LEFT = 0b00010000,
            const ALL = TOP.bits | RIGHT.bits | BOTTOM.bits | LEFT.bits,
        }
    }
}

impl Line {
    fn get(&self) -> char {
        match *self {
            Line::TopRight => '┐',
            Line::Vertical => '│',
            Line::Horizontal => '─',
            Line::TopLeft => '┌',
            Line::BottomRight => '┘',
            Line::BottomLeft => '└',
            Line::VerticalLeft => '┤',
            Line::VerticalRight => '├',
            Line::HorizontalDown => '┬',
            Line::HorizontalUp => '┴',
        }
    }
}

fn hline(x: u16, y: u16, len: u16, fg: Color, bg: Color) -> Buffer {
    Buffer::filled(Rect {
                       x: x,
                       y: y,
                       width: len,
                       height: 1,
                   },
                   Cell {
                       symbol: Line::Horizontal.get(),
                       fg: fg,
                       bg: bg,
                   })
}
fn vline(x: u16, y: u16, len: u16, fg: Color, bg: Color) -> Buffer {
    Buffer::filled(Rect {
                       x: x,
                       y: y,
                       width: 1,
                       height: len,
                   },
                   Cell {
                       symbol: Line::Vertical.get(),
                       fg: fg,
                       bg: bg,
                   })
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum WidgetType {
    Block,
    Text,
    List,
    Gauge,
    Sparkline,
    Chart,
}

pub trait Widget: Hash {
    fn buffer(&self, area: &Rect) -> Buffer;
    fn widget_type(&self) -> WidgetType;
    fn render(&self, area: &Rect) -> Tree {
        let widget_type = self.widget_type();
        let hash = hash(&self, area);
        let buffer = self.buffer(area);
        Tree::Leaf(Leaf {
            widget_type: widget_type,
            hash: hash,
            buffer: buffer,
        })
    }
}
