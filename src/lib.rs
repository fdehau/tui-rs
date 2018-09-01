//! [tui](https://github.com/fdehau/tui-rs) is a library used to build rich
//! terminal users interfaces and dashboards.
//!
//! ![](https://raw.githubusercontent.com/fdehau/tui-rs/master/docs/demo.gif)
//!
//! # Get started
//!
//! ## Creating a `Terminal`
//!
//! Every application using `tui` should start by instantiating a `Terminal`. It is
//! a light abstraction over available backends that provides basic functionalities
//! such as clearing the screen, hiding the cursor, etc. By default only the `termion`
//! backend is available.
//!
//! ```rust,no_run
//! extern crate tui;
//!
//! use tui::Terminal;
//! use tui::backend::RawBackend;
//!
//! fn main() {
//!     let backend = RawBackend::new().unwrap();
//!     let mut terminal = Terminal::new(backend).unwrap();
//! }
//! ```
//!
//! If for some reason, you might want to use the `rustbox` backend instead, you
//! need the to replace your `tui` dependency specification by:
//!
//! ```toml
//! [dependencies.tui]
//! version = "0.2.0"
//! default-features = false
//! features = ['rustbox']
//! ```
//!
//! and then create the terminal in a similar way:
//!
//! ```rust,ignore
//! extern crate tui;
//!
//! use tui::Terminal;
//! use tui::backend::RustboxBackend;
//!
//! fn main() {
//!     let backend = RustboxBackend::new().unwrap();
//!     let mut terminal = Terminal::new(backend).unwrap();
//! }
//! ```
//!
//! ## Building a User Interface (UI)
//!
//! Every component of your interface will be implementing the `Widget` trait.
//! The library comes with a predefined set of widgets that should met most of
//! your use cases. You are also free to implement your owns.
//!
//! Each widget follows a builder pattern API providing a default configuration
//! along with methods to customize them. The widget is then registered using
//! its `render` method that take a `Frame` instance and an area to draw
//! to.
//!
//! The following example renders a block of the size of the terminal:
//!
//! ```rust,no_run
//! extern crate tui;
//!
//! use std::io;
//!
//! use tui::Terminal;
//! use tui::backend::RawBackend;
//! use tui::widgets::{Widget, Block, Borders};
//! use tui::layout::{Layout, Constraint, Direction};
//!
//! fn main() {
//!     let mut terminal = init().expect("Failed initialization");
//!     draw(&mut terminal).expect("Failed to draw");
//! }
//!
//! fn init() -> Result<Terminal<RawBackend>, io::Error> {
//!     let backend = RawBackend::new()?;
//!     Terminal::new(backend)
//! }
//!
//! fn draw(t: &mut Terminal<RawBackend>) -> Result<(), io::Error> {
//!
//!     let size = t.size()?;
//!     t.draw(|mut f| {
//!         Block::default()
//!             .title("Block")
//!             .borders(Borders::ALL)
//!             .render(&mut f, size);
//!     })
//! }
//! ```
//!
//! ## Layout
//!
//! The library comes with a basic yet useful layout management object called `Layout`. As you may
//! see below and in the examples, the library makes heavy use of the builder pattern to provide
//! full customization. And `Layout` is no exception:
//!
//! ```rust,no_run
//! extern crate tui;
//!
//! use std::io;
//!
//! use tui::Terminal;
//! use tui::backend::RawBackend;
//! use tui::widgets::{Widget, Block, Borders};
//! use tui::layout::{Layout, Constraint, Direction};
//!
//! fn main() {
//!     let mut terminal = init().expect("Failed initialization");
//!     draw(&mut terminal).expect("Failed to draw");
//! }
//!
//! fn init() -> Result<Terminal<RawBackend>, io::Error> {
//!     let backend = RawBackend::new()?;
//!     Terminal::new(backend)
//! }
//!
//! fn draw(t: &mut Terminal<RawBackend>) -> Result<(), io::Error> {
//!
//!     let size = t.size()?;
//!     t.draw(|mut f| {
//!         let chunks = Layout::default()
//!             .direction(Direction::Vertical)
//!             .margin(1)
//!             .constraints(
//!                 [
//!                     Constraint::Percentage(10),
//!                     Constraint::Percentage(80),
//!                     Constraint::Percentage(10)
//!                 ].as_ref()
//!             )
//!             .split(size);
//!         Block::default()
//!              .title("Block")
//!              .borders(Borders::ALL)
//!              .render(&mut f, chunks[0]);
//!         Block::default()
//!              .title("Block 2")
//!              .borders(Borders::ALL)
//!              .render(&mut f, chunks[2]);
//!     })
//! }
//! ```
//!
//! This let you describe responsive terminal UI by nesting layouts. You should note
//! that by default the computed layout tries to fill the available space
//! completely. So if for any reason you might need a blank space somewhere, try to
//! pass an additional constraint and don't use the corresponding area.

#[macro_use]
extern crate bitflags;
extern crate cassowary;
extern crate either;
extern crate itertools;
#[macro_use]
extern crate log;
extern crate unicode_segmentation;
extern crate unicode_width;

pub mod backend;
pub mod buffer;
pub mod layout;
pub mod style;
pub mod symbols;
pub mod terminal;
pub mod widgets;

pub use self::terminal::{Frame, Terminal};
