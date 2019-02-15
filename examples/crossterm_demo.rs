#[allow(dead_code)]
mod demo;
#[allow(dead_code)]
mod util;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use structopt::StructOpt;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crossterm;

use crate::demo::{ui, App};

enum Event<I> {
    Input(I),
    Tick,
}

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

    let screen = crossterm::Screen::default();
    let alternate_screen = screen.enable_alternate_modes(true)?;
    let backend = CrosstermBackend::with_alternate_screen(alternate_screen)?;
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup input handling
    let (tx, rx) = mpsc::channel();
    {
        let tx = tx.clone();
        thread::spawn(move || {
            let input = crossterm::input();
            loop {
                match input.read_char() {
                    Ok(key) => {
                        if let Err(_) = tx.send(Event::Input(key)) {
                            return;
                        }
                        if key == 'q' {
                            return;
                        }
                    }
                    Err(_) => {}
                }
            }
        });
    }
    {
        let tx = tx.clone();
        thread::spawn(move || {
            let tx = tx.clone();
            loop {
                tx.send(Event::Tick).unwrap();
                thread::sleep(Duration::from_millis(cli.tick_rate));
            }
        });
    }

    let mut app = App::new("Crossterm Demo");

    terminal.clear()?;

    loop {
        ui::draw(&mut terminal, &app)?;
        match rx.recv()? {
            Event::Input(key) => {
                // TODO: handle key events once they are supported by crossterm
                app.on_key(key);
            }
            Event::Tick => {
                app.on_tick();
            }
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
