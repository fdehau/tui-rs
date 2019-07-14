mod demo;
#[allow(dead_code)]
mod util;

use std::{
    io,
    time::{Duration, Instant},
};

use easycurses;
use itui::{backend::CursesBackend, Terminal};
use structopt::StructOpt;

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

    let mut backend = CursesBackend::new().ok_or(io::Error::new(io::ErrorKind::Other, ""))?;
    let curses = backend.get_curses_mut();
    curses.set_echo(false);
    curses.set_input_timeout(easycurses::TimeoutMode::WaitUpTo(50));
    curses.set_input_mode(easycurses::InputMode::RawCharacter);
    curses.set_keypad_enabled(true);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new("Curses demo");

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    loop {
        ui::draw(&mut terminal, &app)?;
        match terminal.backend_mut().get_curses_mut().get_input() {
            Some(input) => {
                match input {
                    easycurses::Input::Character(c) => {
                        app.on_key(c);
                    }
                    easycurses::Input::KeyUp => {
                        app.on_up();
                    }
                    easycurses::Input::KeyDown => {
                        app.on_down();
                    }
                    easycurses::Input::KeyLeft => {
                        app.on_left();
                    }
                    easycurses::Input::KeyRight => {
                        app.on_right();
                    }
                    _ => {}
                };
            }
            _ => {}
        };
        terminal.backend_mut().get_curses_mut().flush_input();
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
