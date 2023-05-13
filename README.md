# tui-rs

[![Build Status](https://github.com/fdehau/tui-rs/workflows/CI/badge.svg)](https://github.com/fdehau/tui-rs/actions?query=workflow%3ACI+)
[![Crate Status](https://img.shields.io/crates/v/tui.svg)](https://crates.io/crates/tui)
[![Docs Status](https://docs.rs/tui/badge.svg)](https://docs.rs/crate/tui/)

<img src="./assets/demo.gif" alt="Demo cast under Linux Termite with Inconsolata font 12pt">

`tui-rs` is a [Rust](https://www.rust-lang.org) library to build rich terminal
user interfaces and dashboards. It is heavily inspired by the `Javascript`
library [blessed-contrib](https://github.com/yaronn/blessed-contrib) and the
`Go` library [termui](https://github.com/gizak/termui).

The library supports multiple backends:
  - [crossterm](https://github.com/crossterm-rs/crossterm) [default]
  - [termion](https://github.com/ticki/termion)

The library is based on the principle of immediate rendering with intermediate
buffers. This means that at each new frame you should build all widgets that are
supposed to be part of the UI. While providing a great flexibility for rich and
interactive UI, this may introduce overhead for highly dynamic content. So, the
implementation try to minimize the number of ansi escapes sequences generated to
draw the updated UI. In practice, given the speed of `Rust` the overhead rather
comes from the terminal emulator than the library itself.

Moreover, the library does not provide any input handling nor any event system and
you may rely on the previously cited libraries to achieve such features.

**I'm actively looking for help maintaining this crate. See [this issue](https://github.com/fdehau/tui-rs/issues/654)**

### Rust version requirements

Since version 0.17.0, `tui` requires **rustc version 1.56.1 or greater**.

### [Documentation](https://docs.rs/tui)

### Demo

The demo shown in the gif can be run with all available backends.

```
# crossterm
cargo run --example demo --release -- --tick-rate 200
# termion
cargo run --example demo --no-default-features --features=termion --release -- --tick-rate 200
```

where `tick-rate` is the UI refresh rate in ms.

The UI code is in [examples/demo/ui.rs](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/demo/ui.rs) while the
application state is in [examples/demo/app.rs](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/demo/app.rs).

If the user interface contains glyphs that are not displayed correctly by your terminal, you may want to run
the demo without those symbols:

```
cargo run --example demo --release -- --tick-rate 200 --enhanced-graphics false
```

### Widgets

The library comes with the following list of widgets:

  * [Block](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/block.rs)
  * [Gauge](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/gauge.rs)
  * [Sparkline](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/sparkline.rs)
  * [Chart](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/chart.rs)
  * [BarChart](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/barchart.rs)
  * [List](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/list.rs)
  * [Table](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/table.rs)
  * [Paragraph](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/paragraph.rs)
  * [Canvas (with line, point cloud, map)](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/canvas.rs)
  * [Tabs](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/tabs.rs)

Click on each item to see the source of the example. Run the examples with with 
cargo (e.g. to run the gauge example `cargo run --example gauge`), and quit by pressing `q`.

You can run all examples by running `cargo make run-examples` (require
`cargo-make` that can be installed with `cargo install cargo-make`).

### Third-party widgets

* [tui-logger](https://github.com/gin66/tui-logger)
* [tui-textarea](https://github.com/rhysd/tui-textarea): simple yet powerful multi-line text editor widget supporting several key shortcuts, undo/redo, text search, etc.
* [tui-rs-tree-widgets](https://github.com/EdJoPaTo/tui-rs-tree-widget): widget for tree data structures.

### Apps using tui
#### 💻 Development Tools:
- [desed](https://github.com/SoptikHa2/desed): Debugging tool for sed scripts.
- [gitui](https://github.com/extrawurst/gitui): Terminal UI for Git.
- [gobang](https://github.com/TaKO8Ki/gobang): Cross-platform TUI database management tool.
- [joshuto](https://github.com/kamiyaa/joshuto): Ranger-like terminal file manager written in Rust.

#### 🕹️ Games and Entertainment:
- [Battleship.rs](https://github.com/deepu105/battleship-rs): Terminal-based Battleship game.
- [minesweep](https://github.com/cpcloud/minesweep-rs): Terminal-based Minesweeper game.

#### 🚀 Productivity and Utilities:
- [diskonaut](https://github.com/imsnif/diskonaut): Terminal-based disk space navigator.
- [taskwarrior-tui](https://github.com/kdheepak/taskwarrior-tui): TUI for the Taskwarrior command-line task manager.
- [meteo-tui](https://github.com/16arpi/meteo-tui): French weather app in the command line.
- [tickrs](https://github.com/tarkah/tickrs): Stock market ticker in the terminal.

#### 🎼 Music and Media:
- [spotify-tui](https://github.com/Rigellute/spotify-tui): Spotify client for the terminal.
- [ytui-music](https://github.com/sudipghimire533/ytui-music): Listen to music from YouTube in the terminal.

#### 🌐 Networking and Internet:
- [adsb_deku/radar](https://github.com/wcampbell0x2a/adsb_deku#radar-tui): TUI for displaying ADS-B data from aircraft.
- [bandwhich](https://github.com/imsnif/bandwhich): Displays network utilization by process.
- [gping](https://github.com/orf/gping/): Ping tool with a graph.
- [mqttui](https://github.com/EdJoPaTo/mqttui): MQTT client for subscribing or publishing to topics.
- [oha](https://github.com/hatoo/oha): Top-like monitoring tool for HTTP(S) traffic.
- [trippy](https://github.com/fujiapple852/trippy): Network diagnostic tool.

#### 👨‍💻 System Administration:
- [bottom](https://github.com/ClementTsang/bottom): Cross-platform graphical process/system monitor.
- [kmon](https://github.com/orhun/kmon): Linux Kernel Manager and Activity Monitor.
- [oxker](https://github.com/mrjackwills/oxker): Simple TUI to view & control docker containers.
- [xplr](https://github.com/sayanarijit/xplr): Hackable, minimal, and fast TUI file explorer.
- [ytop](https://github.com/cjbassi/ytop): TUI system monitor for Linux.
- [zenith](https://github.com/bvaisvil/zenith): Cross-platform monitoring tool for system stats.

#### 🌌 Other Categories:
- [cotp](https://github.com/replydev/cotp): Command-line TOTP/HOTP authenticator app.
- [hg-tui](https://github.com/kaixinbaba/hg-tui): TUI for viewing the hellogithub.com website.
- [hwatch](https://github.com/blacknon/hwatch): Alternative watch command with command history and diffs.


### Alternatives

You might want to checkout [Cursive](https://github.com/gyscos/Cursive) for an
alternative solution to build text user interfaces in Rust.

## License

[MIT](LICENSE)
