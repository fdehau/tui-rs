#[allow(dead_code)]
mod util;

use crate::util::event::{Event, Events};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::*,
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let input = "OK? [\u{1b}[36my\u{1b}[0mes/\u{1b}[36mn\u{1b}[0mo]\r\n";
    let mut buffer = AnsiBuffer::new(&input);

    let events = Events::new();

    loop {
        terminal.draw(|mut f| {
            let rect = f.size();
            let area = Rect {
                y: 5,
                height: rect.height / 2,
                ..rect
            };

            let text = buffer.as_text();
            let w = Paragraph::new(text.iter()).raw(true).block(
                Block::default()
                    .border_type(BorderType::Plain)
                    .borders(Borders::ALL),
            );

            f.render_widget(w, area);
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
