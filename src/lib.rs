//! [itui](https://github.com/fdehau/itui-rs) is a library used to build rich
//! terminal users interfaces and dashboards.
//!
//! ![](https://raw.githubusercontent.com/fdehau/itui-rs/master/assets/demo.gif)
//!
//! # Get started
//!
//! ## Adding `itui` as a dependency
//!
//! ```toml
//! [dependencies]
//! itui = "0.5"
//! termion = "1.5"
//! ```
//!
//! The crate is using the `termion` backend by default but if for some reason you might want to use
//! the `rustbox` backend instead, you need the to replace your dependency specification by:
//!
//! ```toml
//! [dependencies]
//! rustbox = "0.11"
//!
//! [dependencies.itui]
//! version = "0.5"
//! default-features = false
//! features = ['rustbox']
//! ```
//!
//! The same logic applies for all other available backends.
//!
//! ## Creating a `Terminal`
//!
//! Every application using `itui` should start by instantiating a `Terminal`. It is a light
//! abstraction over available backends that provides basic functionalities such as clearing the
//! screen, hiding the cursor, etc.
//!
//! ```rust,no_run
//! use std::io;
//! use itui::Terminal;
//! use itui::backend::TermionBackend;
//! use termion::raw::IntoRawMode;
//!
//! fn main() -> Result<(), io::Error> {
//!     let stdout = io::stdout().into_raw_mode()?;
//!     let backend = TermionBackend::new(stdout);
//!     let mut terminal = Terminal::new(backend)?;
//!     Ok(())
//! }
//! ```
//!
//! If you had previously chosen `rustbox` as a backend, the terminal can be created in a similar
//! way:
//!
//! ```rust,ignore
//! use itui::Terminal;
//! use itui::backend::RustboxBackend;
//!
//! fn main() -> Result<(), io::Error> {
//!     let backend = RustboxBackend::new()?;
//!     let mut terminal = Terminal::new(backend);
//!     Ok(())
//! }
//! ```
//!
//! You may also refer to the examples to find out how to create a `Terminal` for each available
//! backend.
//!
//! ## Building a User Interface (UI)
//!
//! Every component of your interface will be implementing the `Widget` trait.  The library comes
//! with a predefined set of widgets that should met most of your use cases. You are also free to
//! implement your owns.
//!
//! Each widget follows a builder pattern API providing a default configuration along with methods
//! to customize them. The widget is then registered using its `render` method that take a `Frame`
//! instance and an area to draw to.
//!
//! The following example renders a block of the size of the terminal:
//!
//! ```rust,no_run
//! use std::io;
//! use termion::raw::IntoRawMode;
//! use itui::Terminal;
//! use itui::backend::TermionBackend;
//! use itui::widgets::{Widget, Block, Borders};
//! use itui::layout::{Layout, Constraint, Direction};
//!
//! fn main() -> Result<(), io::Error> {
//!     let stdout = io::stdout().into_raw_mode()?;
//!     let backend = TermionBackend::new(stdout);
//!     let mut terminal = Terminal::new(backend)?;
//!     terminal.draw(|mut f| {
//!         let size = f.size();
//!         Block::default()
//!             .title("Block")
//!             .borders(Borders::ALL)
//!             .area(size)
//!             .render(&mut f);
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
//! use std::io;
//! use termion::raw::IntoRawMode;
//! use itui::Terminal;
//! use itui::backend::TermionBackend;
//! use itui::widgets::{Widget, Block, Borders};
//! use itui::layout::{Layout, Constraint, Direction};
//!
//! fn main() -> Result<(), io::Error> {
//!     let stdout = io::stdout().into_raw_mode()?;
//!     let backend = TermionBackend::new(stdout);
//!     let mut terminal = Terminal::new(backend)?;
//!     terminal.draw(|mut f| {
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
//!             .split(f.size());
//!         Block::default()
//!              .title("Block")
//!              .borders(Borders::ALL)
//!              .area(chunks[0])
//!              .render(&mut f);
//!         Block::default()
//!              .title("Block 2")
//!              .borders(Borders::ALL)
//!              .area(chunks[2])
//!              .render(&mut f);
//!     })
//! }
//! ```
//!
//! This let you describe responsive terminal UI by nesting layouts. You should note that by
//! default the computed layout tries to fill the available space completely. So if for any reason
//! you might need a blank space somewhere, try to pass an additional constraint and don't use the
//! corresponding area.

pub mod backend;
pub mod buffer;
pub mod layout;
pub mod style;
pub mod symbols;
pub mod terminal;
pub mod widgets;

pub use self::terminal::{Frame, Terminal};
