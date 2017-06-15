# tui-rs

[![Build Status](https://travis-ci.org/fdehau/tui-rs.svg?branch=master)](https://travis-ci.org/fdehau/tui-rs)
[![Crate Status](https://img.shields.io/crates/v/tui.svg)](https://crates.io/crates/tui)
[![Docs Status](https://docs.rs/tui/badge.svg)](https://docs.rs/crate/tui/)

<img src="./docs/demo.gif" alt="Demo cast under Linux Termite with Inconsolata font 12pt">

`tui-rs` is a [Rust](https://www.rust-lang.org) library to build rich terminal
user interfaces and dashboards. It is heavily inspired by the `Javascript`
library [blessed-contrib](https://github.com/yaronn/blessed-contrib) and the
`Go` library [termui](https://github.com/gizak/termui).

The library itself supports two different backends to draw to the terminal. You
can either choose from:

  - [termion](https://github.com/ticki/termion)
  - [rustbox](https://github.com/gchp/rustbox)

However, some features may only be available in one of the two.

The library is based on the principle of immediate rendering with intermediate
buffers. This means that at each new frame you should build all widgets that are
supposed to be part of the UI. While providing a great flexibility for rich and
interactive UI, this may introduce overhead for highly dynamic content. So, the
implementation try to minimize the number of ansi escapes sequences generated to
draw the updated UI. In practice, given the speed of `Rust` the overhead rather
comes from the terminal emulator than the library itself.

Moreover, the library does not provide any input handling nor any event system and
you may rely on the previously cited libraries to achieve such features.

## Get Started

### Create the terminal interface

Every application using `tui` should start by instantiating a `Terminal`. It is
a light abstraction over available backends that provides basic functionalities
such as clearing the screen, hiding the cursor, etc. By default only the `termion`
backend is available.

```rust
use tui::Terminal;
use tui::backend::TermionBackend;

fn main() {
  let backend = TermionBackend::new().unwrap();
  let mut terminal = Terminal::new(backend);
}
```

If for some reason, you might want to use the `rustbox` backend instead, you
need the to replace your `tui` dependency specification by:

```toml
[dependencies.tui]
version = "0.1.3"
default-features = false
features = ['rustbox']
```

and then create the terminal in a similar way:

```rust
use tui::Terminal;
use tui::backend::RustboxBackend;

fn main() {
  let backend = RustboxBackend::new().unwrap();
  let mut terminal = Terminal::new(backend);
}
```

### Layout

The library comes with a basic yet useful layout management object called
`Group`. As you may see below and in the examples, the library makes heavy use
of the builder pattern to provide full customization. And the `Group` object is
no exception:

```rust
use tui::widgets::{Block, border};
use tui::layout::{Group, Rect, Direction};

fn draw(t: &mut Terminal<TermionBackend>) {

  let size = t.size().unwrap();

  Group::default()
    /// You first choose a main direction for the group
    .direction(Direction::Vertical)
    /// An optional margin
    .margin(1)
    /// The preferred sizes (heights in this case)
    .sizes(&[Size::Fixed(10), Size::Max(20), Size::Min(10)])
    /// The computed (or cached) layout is then available as the second argument
    /// of the closure
    .render(t, &size, |t, chunks| {
        /// Continue to describe your UI there.
        Block::default()
          .title("Block")
          .borders(border::ALL)
          .render(t, &chunks[0]);
    })
```

This let you describe responsive terminal UI by nesting groups. You should note
that by default the computed layout tries to fill the available space
completely. So if for any reason you might need a blank space somewhere, try to
pass an additional size to the group and don't use the corresponding area inside
the render method.

Once you have finished to describe the UI, you just need to call:

```rust
t.draw().unwrap()
```

to actually draw to the terminal.

### Widgets

The library comes with the following list of widgets:

  * [Block](examples/block.rs)
  * [Gauge](examples/gauge.rs)
  * [Sparkline](examples/sparkline.rs)
  * [Chart](examples/chart.rs)
  * [BarChart](examples/bar_chart.rs)
  * [List](examples/list.rs)
  * [Table](examples/table.rs)
  * [Paragraph](examples/paragraph.rs)
  * [Canvas (with line, point cloud, map)](examples/canvas.rs)
  * [Tabs](examples/tabs.rs)

Click on each item to get an example.

### Demo

The [source code](examples/demo.rs) of the demo gif.

## License

[MIT](LICENSE)

## Author

Florian Dehau
