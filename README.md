# tui-rs

[![Build Status](https://travis-ci.org/fdehau/tui-rs.svg?branch=master)](https://travis-ci.org/fdehau/tui-rs)
[![Crate Status](https://img.shields.io/crates/v/tui.svg)](https://crates.io/crates/tui)
[![Docs Status](https://docs.rs/tui/badge.svg)](https://docs.rs/crate/tui/)

<img src="./assets/demo.gif" alt="Demo cast under Linux Termite with Inconsolata font 12pt">

`tui-rs` is a [Rust](https://www.rust-lang.org) library to build rich terminal
user interfaces and dashboards. It is heavily inspired by the `Javascript`
library [blessed-contrib](https://github.com/yaronn/blessed-contrib) and the
`Go` library [termui](https://github.com/gizak/termui).

The library itself supports three different backends to draw to the terminal. You
can either choose from:

  - [termion](https://github.com/ticki/termion)
  - [rustbox](https://github.com/gchp/rustbox)
  - [crossterm](https://github.com/TimonPost/crossterm)

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

### [Documentation](https://docs.rs/tui)

### Demo

The [source code](examples/demo.rs) of the demo gif.

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

Click on each item to see the source of the example. Run the examples with with 
cargo (e.g. to run the demo `cargo run --example demo`), and quit by pressing `q`.

### Third-party widgets

* [tui-logger](https://github.com/gin66/tui-logger)

### Alternatives

You might want to checkout [Cursive](https://github.com/gyscos/Cursive) for an
alternative solution to build text user interfaces in Rust.

## License

[MIT](LICENSE)
