#[allow(dead_code)]
mod util;

use crate::util::event::{Event, Events};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, TableState},
    Terminal,
};

pub struct TableStateContainer {
    state: TableState,
    items: Vec<Vec<String>>,
}

impl TableStateContainer {
    fn new() -> TableStateContainer {
        let mut items = vec![];
        let max_row = 100;
        items.resize(max_row, vec![]);
        for row in 0..100 {
            for column in 0..3 {
                items[row].push(format!("{}.{}", row, column))
            }
        }

        TableStateContainer {
            state: TableState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn next_page(&mut self) {
        let page_size = self.state.page_size.unwrap_or(1);
        let i = match self.state.selected() {
            Some(i) => {
                if (i + page_size) > self.items.len() - 1 {
                    i + page_size - self.items.len()
                } else {
                    i + page_size
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous_page(&mut self) {
        let page_size = self.state.page_size.unwrap_or(1);
        let i = match self.state.selected() {
            Some(i) => {
                if i >= page_size {
                    i - page_size
                } else {
                    let remainder = page_size - i;
                    self.items.len() - remainder - i
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut table = TableStateContainer::new();

    // Input
    loop {
        terminal.draw(|mut f| {
            let rects = Layout::default()
                .constraints([Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let selected_style = Style::default().fg(Color::Yellow).modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::White);
            let header = ["Header1", "Header2", "Header3"];
            let rows = table
                .items
                .iter()
                .map(|i| Row::StyledData(i.into_iter(), normal_style));
            let t = Table::new(header.iter(), rows)
                .block(Block::default().borders(Borders::ALL).title("Table"))
                .highlight_style(selected_style)
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]);
            f.render_stateful_widget(t, rects[0], &mut table.state);
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                }
                Key::Down | Key::Char('j') => {
                    table.next();
                }
                Key::Up | Key::Char('k') => {
                    table.previous();
                }
                Key::PageDown | Key::Char('n') => {
                    table.next_page();
                }
                Key::PageUp | Key::Char('p') => {
                    table.previous_page();
                }
                _ => {}
            },
            _ => {}
        };
    }

    Ok(())
}
