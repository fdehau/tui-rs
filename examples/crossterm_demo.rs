#[allow(dead_code)]
mod demo;
#[allow(dead_code)]
mod util;

use crate::demo::{ui, App};
use argh::FromArgs;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::Duration,
};
use tui::{backend::CrosstermBackend, Terminal};

enum Event<I> {
    Input(I),
    Resize,
    Tick,
}

/// Crossterm demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup input handling
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            if event::poll(Duration::from_millis(cli.tick_rate)).unwrap() {
                match event::read().unwrap() {
                    CEvent::Key(key) => {
                        tx.send(Event::Input(key)).unwrap();
                    }
                    CEvent::Resize(_, _) => {
                        // Force re-render on resize event.
                        tx.send(Event::Resize).unwrap();
                    }
                    _ => {}
                }
            }

            tx.send(Event::Tick).unwrap();
        }
    });

    let mut app = App::new("Crossterm Demo");

    terminal.clear()?;

    loop {
        terminal.draw(|mut f| ui::draw(&mut f, &mut app))?;
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Left => app.on_left(),
                KeyCode::Up => app.on_up(),
                KeyCode::Right => app.on_right(),
                KeyCode::Down => app.on_down(),
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
            _ => {}
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
