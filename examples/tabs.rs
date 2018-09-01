extern crate termion;
extern crate tui;

mod util;
use util::*;

use std::io;
use termion::event;
use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Tabs, Widget};
use tui::Terminal;

struct App<'a> {
    size: Rect,
    tabs: MyTabs<'a>,
}

fn main() {
    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // App
    let mut app = App {
        size: Rect::default(),
        tabs: MyTabs {
            titles: vec!["Tab0", "Tab1", "Tab2", "Tab3"],
            selection: 0,
        },
    };

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    draw(&mut terminal, &mut app).unwrap();

    // Main loop
    let stdin = io::stdin();
    for c in stdin.keys() {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }

        let evt = c.unwrap();
        match evt {
            event::Key::Char('q') => {
                break;
            }
            event::Key::Right => app.tabs.next(),
            event::Key::Left => app.tabs.previous(),
            _ => {}
        }
        draw(&mut terminal, &mut app).unwrap();
    }

    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<MouseBackend>, app: &App) -> Result<(), io::Error> {
    t.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(app.size);

        Block::default()
            .style(Style::default().bg(Color::White))
            .render(&mut f, app.size);
        Tabs::default()
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .titles(&app.tabs.titles)
            .select(app.tabs.selection)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::Yellow))
            .render(&mut f, chunks[0]);
        match app.tabs.selection {
            0 => {
                Block::default()
                    .title("Inner 0")
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]);
            }
            1 => {
                Block::default()
                    .title("Inner 1")
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]);
            }
            2 => {
                Block::default()
                    .title("Inner 2")
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]);
            }
            3 => {
                Block::default()
                    .title("Inner 3")
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]);
            }
            _ => {}
        }
    })
}
