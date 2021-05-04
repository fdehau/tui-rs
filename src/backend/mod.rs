use crate::buffer::Cell;
use crate::layout::Rect;

#[cfg(feature = "rustbox")]
pub(super) mod rustbox;
#[cfg(feature = "rustbox")]
pub use self::rustbox::RustboxBackend;

#[cfg(feature = "termion")]
pub(super) mod termion;
#[cfg(feature = "termion")]
pub use self::termion::TermionBackend;

#[cfg(feature = "crossterm")]
pub(super) mod crossterm;
#[cfg(feature = "crossterm")]
pub use self::crossterm::CrosstermBackend;

#[cfg(feature = "curses")]
pub(super) mod curses;
#[cfg(feature = "curses")]
pub use self::curses::CursesBackend;

mod test;
pub use self::test::TestBackend;

pub trait Backend {
    type Error: std::error::Error + Send + Sync + 'static;

    fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>;
    fn hide_cursor(&mut self) -> Result<(), Self::Error>;
    fn show_cursor(&mut self) -> Result<(), Self::Error>;
    fn get_cursor(&mut self) -> Result<(u16, u16), Self::Error>;
    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), Self::Error>;
    fn clear(&mut self) -> Result<(), Self::Error>;
    fn size(&self) -> Result<Rect, Self::Error>;
    fn flush(&mut self) -> Result<(), Self::Error>;
}
