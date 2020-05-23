mod demo;
#[allow(dead_code)]
mod util;

use crate::demo::{ui, App};
use argh::FromArgs;
use std::{
    error::Error,
    time::{Duration, Instant},
};
use termwiz::{input::*, terminal::Terminal as TermwizTerminal};
use tui::{backend::TermwizBackend, Terminal};

/// Termwiz demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();

    let backend = TermwizBackend::new()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new("Termwiz demo", cli.enhanced_graphics);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(cli.tick_rate);

    loop {
        terminal.draw(|mut f| ui::draw(&mut f, &mut app))?;
        match terminal
            .backend_mut()
            .buffered_terminal_mut()
            .terminal()
            .poll_input(Some(tick_rate))
        {
            Ok(Some(input)) => match input {
                InputEvent::Key(KeyEvent {
                    key: KeyCode::Char(c),
                    ..
                }) => app.on_key(c),
                InputEvent::Key(KeyEvent {
                    key: KeyCode::UpArrow,
                    ..
                }) => app.on_up(),
                InputEvent::Key(KeyEvent {
                    key: KeyCode::DownArrow,
                    ..
                }) => app.on_down(),
                InputEvent::Key(KeyEvent {
                    key: KeyCode::LeftArrow,
                    ..
                }) => app.on_left(),
                InputEvent::Key(KeyEvent {
                    key: KeyCode::RightArrow,
                    ..
                }) => app.on_right(),
                InputEvent::Resized { cols, rows } => {
                    terminal
                        .backend_mut()
                        .buffered_terminal_mut()
                        .resize(cols, rows);
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
