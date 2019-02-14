mod demo;
#[allow(dead_code)]
mod util;

use std::time::{Duration, Instant};

use structopt::StructOpt;
use tui::backend::CursesBackend;
use easycurses;
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

    let mut terminal = Terminal::new(CursesBackend::new().unwrap()).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    terminal
        .backend_mut()
        .get_curses_window_mut()
        .set_input_timeout(easycurses::TimeoutMode::WaitUpTo(50));

    let mut app = App::new("Curses demo");

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    loop {
        ui::draw(&mut terminal, &app)?;
        match terminal.backend_mut().get_curses_window_mut().get_input() {
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
        terminal.backend_mut().get_curses_window_mut().flush_input();
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
