#[allow(dead_code)]
mod util;

use crate::util::event::{Event, Events};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Margin},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut i = 0;
    let mut lines = Vec::with_capacity(100);
    while i < 100 {
        lines.push((i, format!("{}: {}", i, LOREM_IPSUM)));
        i += 1;
    }
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let text: Vec<Spans> = lines
                .iter()
                .cloned()
                .map(|(j, l)| {
                    let span = if i == j + 1 {
                        Span::styled(l, Style::default().bg(Color::Yellow))
                    } else {
                        Span::raw(l)
                    };
                    Spans::from(span)
                })
                .collect();
            let mut wrap = Wrap::default();
            wrap.scroll_callback = Some(Box::new(|text_area, lines| {
                let len = lines.len() as u16;
                (len.saturating_sub(text_area.height), 0)
            }));
            let paragraph = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL))
                .wrap(wrap)
                .alignment(Alignment::Left);
            f.render_widget(
                paragraph,
                size.inner(&Margin {
                    vertical: 2,
                    horizontal: 2,
                }),
            );
        })?;

        match events.next()? {
            Event::Tick => {
                lines.push((i, format!("{}: {}", i, LOREM_IPSUM)));
                lines.remove(0);
                i += 1;
            }
            Event::Input(key) => {
                if key == Key::Char('q') {
                    break;
                }
            }
        }
    }
    Ok(())
}
