mod demo;
#[allow(dead_code)]
mod util;

use std::{io, time::Duration};

use itui::{backend::TermionBackend, Terminal};
use structopt::StructOpt;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};

use crate::{
    demo::{ui, App},
    util::event::{Config, Event, Events},
};

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

    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(cli.tick_rate),
        ..Config::default()
    });

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new("Termion demo");
    loop {
        ui::draw(&mut terminal, &app)?;
        match events.next()? {
            Event::Input(key) => match key {
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
            Event::Tick => {
                app.on_tick();
            }
            Event::MouseEvent(me) => {
                app.on_mouse(me);
            }
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
