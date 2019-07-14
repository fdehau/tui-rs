#[allow(dead_code)]
mod util;

use std::io;

use itui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};

use crate::util::event::{Event, Events};

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            Block::default()
                .title("Block")
                .borders(Borders::ALL)
                .area(chunks[0])
                .render(&mut f);
            Block::default()
                .title("Block 2")
                .borders(Borders::ALL)
                .area(chunks[2])
                .render(&mut f);
        })?;

        match events.next()? {
            Event::Input(input) => {
                if let Key::Char('q') = input {
                    break;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
