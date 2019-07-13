#[allow(dead_code)]
mod util;

use std::io;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

use crate::util::event::{Event, Events};

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup event handlers
    let events = Events::new();

    loop {
        terminal.draw(|mut f| {
            // Wrapping block for a group
            // Just draw the block and the group on the same area and build the group
            // with at least a margin of 1
            let size = f.size();
            Block::default().borders(Borders::ALL).area(size).render(&mut f);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(4)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[0]);
                Block::default()
                    .title("With background")
                    .title_style(Style::default().fg(Color::Yellow))
                    .style(Style::default().bg(Color::Green))
                    .area(chunks[0])
                    .render(&mut f);
                Block::default()
                    .title("Styled title")
                    .title_style(
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::Red)
                            .modifier(Modifier::BOLD),
                    )
                    .area(chunks[1])
                    .render(&mut f);
            }
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[1]);
                Block::default()
                    .title("With borders")
                    .borders(Borders::ALL)
                    .area(chunks[0])
                    .render(&mut f);
                Block::default()
                    .title("With styled borders")
                    .border_style(Style::default().fg(Color::Cyan))
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .area(chunks[1])
                    .render(&mut f);
            }
        })?;

        match events.next()? {
            Event::Input(key) => {
                if key == Key::Char('q') {
                    break;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
