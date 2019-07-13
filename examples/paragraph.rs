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

    let mut scroll: u16 = 0;
    loop {
        terminal.draw(|mut f| {
            let size = f.size();

            // Words made "loooong" to demonstrate line breaking.
            let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
            let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
            long_line.push('\n');

            Block::default()
                .style(Style::default().bg(Color::White))
                .area(size)
                .render(&mut f);

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
                Text::raw("This is a line \n"),
                Text::styled("This is a line   \n", Style::default().fg(Color::Red)),
                Text::styled("This is a line\n", Style::default().bg(Color::Blue)),
                Text::styled(
                    "This is a longer line\n",
                    Style::default().modifier(Modifier::CROSSED_OUT),
                ),
                Text::styled(&long_line, Style::default().bg(Color::Green)),
                Text::styled(
                    "This is a line\n",
                    Style::default().fg(Color::Green).modifier(Modifier::ITALIC),
                ),
            ];

            let block = Block::default()
                .borders(Borders::ALL)
                .title_style(Style::default().modifier(Modifier::BOLD));
            Paragraph::new(text.iter())
                .block(block.clone().title("Left, no wrap").area(chunks[0]))
                .alignment(Alignment::Left)
                .area(chunks[0])
                .render(&mut f);
            Paragraph::new(text.iter())
                .block(block.clone().title("Left, wrap").area(chunks[1]))
                .alignment(Alignment::Left)
                .wrap(true)
                .area(chunks[1])
                .render(&mut f);
            Paragraph::new(text.iter())
                .block(block.clone().title("Center, wrap").area(chunks[2]))
                .alignment(Alignment::Center)
                .wrap(true)
                .scroll(scroll)
                .area(chunks[2])
                .render(&mut f);
            Paragraph::new(text.iter())
                .block(block.clone().title("Right, wrap").area(chunks[3]))
                .alignment(Alignment::Right)
                .wrap(true)
                .area(chunks[3])
                .render(&mut f);
        })?;

        scroll += 1;
        scroll %= 10;

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
