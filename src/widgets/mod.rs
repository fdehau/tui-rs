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
pub use self::chart::{Chart, Axis, Dataset};

use std::hash::Hash;

use util::hash;
use buffer::{Buffer, Cell};
use layout::Rect;
use style::Color;
use terminal::Terminal;

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

pub trait Widget<'a> {
    fn buffer(&'a self, area: &Rect) -> Buffer<'a>;
    fn render(&'a self, area: &Rect, t: &mut Terminal) {
        t.render_buffer(self.buffer(area));
    }
}
