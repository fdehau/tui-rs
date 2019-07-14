/// A simple example demonstrating how to handle user input. This is
/// a bit out of the scope of the library as it does not provide any
/// input handling out of the box. However, it may helps some to get
/// started.
///
/// This is a very simple example:
///   * A input box always focused. Every character you type is registered
///   here
///   * Pressing Backspace erases a character
///   * Pressing Enter pushes the current input in the history of previous
///   messages

#[allow(dead_code)]
mod util;

use std::io::{self, Write};

use itui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, Paragraph, Text, Widget},
    Terminal,
};
use termion::{
    cursor::Goto, event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen,
};
use unicode_width::UnicodeWidthStr;

use crate::util::event::{Event, Events};

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// History of recorded messages
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let events = Events::new();

    // Create default app state
    let mut app = App::default();

    loop {
        // Draw UI
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                .split(f.size());
            Paragraph::new([Text::raw(&app.input)].iter())
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Input")
                        .area(chunks[0]),
                )
                .render(&mut f);
            let messages = app
                .messages
                .iter()
                .enumerate()
                .map(|(i, m)| Text::raw(format!("{}: {}", i, m)));
            List::new(messages)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Messages")
                        .area(chunks[1]),
                )
                .render(&mut f);
        })?;

        // Put the cursor back inside the input box
        write!(
            terminal.backend_mut(),
            "{}",
            Goto(4 + app.input.width() as u16, 4)
        )?;
        // stdout is buffered, flush it to see the effect immediately when hitting backspace
        io::stdout().flush().ok();

        // Handle input
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('\n') => {
                    app.messages.push(app.input.drain(..).collect());
                }
                Key::Char(c) => {
                    app.input.push(c);
                }
                Key::Backspace => {
                    app.input.pop();
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
