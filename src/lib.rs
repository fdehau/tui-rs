extern crate termion;
#[macro_use]
extern crate bitflags;
extern crate cassowary;

mod buffer;
pub mod terminal;
pub mod widgets;
pub mod style;
pub mod layout;

pub use self::terminal::Terminal;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
