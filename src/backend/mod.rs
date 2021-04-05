//! Stuff related to the different backends like crossterm, curses, rustbox, termion and a backend for testing purposes.

use std::io;

use crate::buffer::Cell;
use crate::layout::Rect;

#[cfg(feature = "rustbox")]
mod rustbox;
#[cfg(feature = "rustbox")]
pub use self::rustbox::RustboxBackend;

#[cfg(feature = "termion")]
mod termion;
#[cfg(feature = "termion")]
pub use self::termion::TermionBackend;

#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "crossterm")]
pub use self::crossterm::CrosstermBackend;

#[cfg(feature = "curses")]
mod curses;
#[cfg(feature = "curses")]
pub use self::curses::CursesBackend;

mod test;
pub use self::test::TestBackend;

/// Shared interface of all backends.
pub trait Backend {
    /// Draw something on the terminal-screen.
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>;

    /// Hide the cursors.
    fn hide_cursor(&mut self) -> Result<(), io::Error>;

    /// Show the cursors.
    fn show_cursor(&mut self) -> Result<(), io::Error>;

    /// Get current row and column of the cursor.
    fn get_cursor(&mut self) -> Result<(u16, u16), io::Error>;

    /// Set the row and column of the cursor.
    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), io::Error>;

    /// Clear the terminal-screen.
    fn clear(&mut self) -> Result<(), io::Error>;

    /// Get a rectangle of the same size as the terminal-screen.
    fn size(&self) -> Result<Rect, io::Error>;

    /// Execute any remaining read- and write-operations.
    fn flush(&mut self) -> Result<(), io::Error>;
}
