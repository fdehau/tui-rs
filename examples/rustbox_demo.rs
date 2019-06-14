mod demo;
#[allow(dead_code)]
mod util;

use std::time::{Duration, Instant};

use rustbox::keyboard::Key;
use structopt::StructOpt;
use tui::backend::RustboxBackend;
use tui::Terminal;

use crate::demo::{ui, App};

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "tick-rate", default_value = "250")]
    tick_rate: u64,
    #[structopt(long = "log")]
    log: bool,
}

fn main() -> Result<(), failure::Error> {
    let cli = Cli::from_args();
    stderrlog::new().quiet(!cli.log).verbosity(4).init()?;

    let backend = RustboxBackend::new()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new("Rustbox demo");

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    loop {
        ui::draw(&mut terminal, &app)?;
        match terminal.backend().rustbox().peek_event(tick_rate, false) {
            Ok(rustbox::Event::KeyEvent(key)) => match key {
                Key::Char(c) => {
                    app.on_key(c);
                }
                Key::Up => {
                    app.on_up();
                }
                Key::Down => {
                    app.on_down();
                }
                Key::Left => {
                    app.on_left();
                }
                Key::Right => {
                    app.on_right();
                }
                _ => {}
            },
            _ => {}
        }
        if last_tick.elapsed() > tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            break;
        }
    }
    Ok(())
}
