#[allow(dead_code)]
mod util;

use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Row, Table, Widget};
use tui::Terminal;

use crate::util::event::{Event, Events};

struct App {
    items: Vec<Vec<String>>,
    selected: usize,
}

impl App {
    fn new() -> App {
        App {
            items: (1..100)
                .map(|i| {
                    let e1 = format!("Row{}Col1", i);
                    let e2 = format!("Row{}Col2", i);
                    let e3 = format!("Row{}Col3", i);
                    vec![e1, e2, e3]
                })
                .collect::<Vec<_>>(),
            selected: 0,
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

    // App
    let mut app = App::new();

    // Input
    loop {
        terminal.draw(|mut f| {
            let selected_style = Style::default().fg(Color::Yellow).modifier(Modifier::BOLD);
            let header = ["Header1", "Header2", "Header3"];
            let rows = app.items.iter().map(|item| Row::Data(item.into_iter()));

            let rects = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .margin(5)
                .split(f.size());
            Table::new(header.into_iter(), rows)
                .block(Block::default().borders(Borders::ALL).title("Table"))
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Max(10),
                ])
                .highlight_style(selected_style)
                .highlight_symbol("|=>|")
                .select(Some(app.selected))
                .render(&mut f, rects[0]);
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                }
                Key::Down => {
                    app.selected += 1;
                    if app.selected > app.items.len() - 1 {
                        app.selected = 0;
                    }
                }
                Key::Up => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    } else {
                        app.selected = app.items.len() - 1;
                    }
                }
                _ => {}
            },
            _ => {}
        };
    }

    Ok(())
}
