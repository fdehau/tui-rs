extern crate failure;
extern crate termion;
extern crate tui;

#[allow(dead_code)]
mod util;

use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Paragraph, Text, Widget};
use tui::Terminal;

use util::event::{Event, Events};

struct App {
    size: Rect,
}

impl Default for App {
    fn default() -> App {
        App {
            size: Rect::default(),
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
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut app = App::default();

    loop {
        let size = terminal.size()?;
        if size != app.size {
            terminal.resize(size)?;
            app.size = size;
        }

        terminal.draw(|mut f| {
            Block::default()
                .style(Style::default().bg(Color::White))
                .render(&mut f, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(30),
                        Constraint::Percentage(30),
                    ]
                        .as_ref(),
                ).split(size);

            let text = [
                Text::raw("This a line\n"),
                Text::styled("This a line\n", Style::default().fg(Color::Red)),
                Text::styled("This a line\n", Style::default().bg(Color::Blue)),
                Text::styled(
                    "This a longer line\n",
                    Style::default().modifier(Modifier::CrossedOut),
                ),
                Text::styled(
                    "This a line\n",
                    Style::default().fg(Color::Green).modifier(Modifier::Italic),
                ),
            ];

            Paragraph::new(text.iter())
                .alignment(Alignment::Left)
                .render(&mut f, chunks[0]);
            Paragraph::new(text.iter())
                .alignment(Alignment::Center)
                .wrap(true)
                .render(&mut f, chunks[1]);
            Paragraph::new(text.iter())
                .alignment(Alignment::Right)
                .wrap(true)
                .render(&mut f, chunks[2]);
        })?;

        match events.next()? {
            Event::Input(key) => if key == Key::Char('q') {
                break;
            },
            _ => {}
        }
    }
    terminal.show_cursor()?;
    Ok(())
}
