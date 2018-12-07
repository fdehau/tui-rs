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
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;

use util::event::{Event, Events};

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
            let size = f.size();

            // Words made "loooong" to demonstrate line breaking.
            let s = "Veeeeeeeeeeeeeeeery loooooooooooooooooong striiiiiiiiiiiiiiiiiiiiiiiiiing. ";
            let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
            long_line.push('\n');

            Block::default()
                .style(Style::default().bg(Color::White))
                .render(&mut f, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(size);

            let text = [
                Text::raw("This a line\n"),
                Text::styled("This a line\n", Style::default().fg(Color::Red)),
                Text::styled("This a line\n", Style::default().bg(Color::Blue)),
                Text::styled(
                    "This a longer line\n",
                    Style::default().modifier(Modifier::CrossedOut),
                ),
                Text::raw(&long_line),
                Text::styled(
                    "This a line\n",
                    Style::default().fg(Color::Green).modifier(Modifier::Italic),
                ),
            ];

            let block = Block::default()
                .borders(Borders::ALL)
                .title_style(Style::default().modifier(Modifier::Bold));
            Paragraph::new(text.iter())
                .block(block.clone().title("Left, no wrap"))
                .alignment(Alignment::Left)
                .render(&mut f, chunks[0]);
            Paragraph::new(text.iter())
                .block(block.clone().title("Left, wrap"))
                .alignment(Alignment::Left)
                .wrap(true)
                .render(&mut f, chunks[1]);
            Paragraph::new(text.iter())
                .block(block.clone().title("Center, wrap"))
                .alignment(Alignment::Center)
                .wrap(true)
                .render(&mut f, chunks[2]);
            Paragraph::new(text.iter())
                .block(block.clone().title("Right, wrap"))
                .alignment(Alignment::Right)
                .wrap(true)
                .render(&mut f, chunks[3]);
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
