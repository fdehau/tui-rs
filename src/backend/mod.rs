use std::io;

use buffer::Cell;
use layout::Rect;

#[cfg(feature = "rustbox")]
mod rustbox;
#[cfg(feature = "rustbox")]
pub use self::rustbox::RustboxBackend;

#[cfg(feature = "termion")]
mod termion;
#[cfg(feature = "termion")]
pub use self::termion::{TermionBackend, MouseBackend, RawBackend};

pub trait Backend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>;
    fn hide_cursor(&mut self) -> Result<(), io::Error>;
    fn show_cursor(&mut self) -> Result<(), io::Error>;
    fn clear(&mut self) -> Result<(), io::Error>;
    fn size(&self) -> Result<Rect, io::Error>;
    fn flush(&mut self) -> Result<(), io::Error>;
}
