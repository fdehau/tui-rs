extern crate termion;
extern crate tui;

use std::io;

use termion::event;
use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Row, Table, Widget};
use tui::Terminal;

struct App<'a> {
    size: Rect,
    items: Vec<Vec<&'a str>>,
    selected: usize,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            size: Rect::default(),
            items: vec![
                vec!["Row12", "Row12", "Row13"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row31", "Row32", "Row33"],
                vec!["Row41", "Row42", "Row43"],
                vec!["Row51", "Row52", "Row53"],
                vec!["Row61", "Row62", "Row63"],
            ],
            selected: 0,
        }
    }
}

fn main() {
    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // App
    let mut app = App::new();

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    draw(&mut terminal, &app).unwrap();

    // Input
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
            event::Key::Down => {
                app.selected += 1;
                if app.selected > app.items.len() - 1 {
                    app.selected = 0;
                }
            }
            event::Key::Up => if app.selected > 0 {
                app.selected -= 1;
            } else {
                app.selected = app.items.len() - 1;
            },
            _ => {}
        };
        draw(&mut terminal, &app).unwrap();
    }

    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
}

fn draw(t: &mut Terminal<MouseBackend>, app: &App) -> Result<(), io::Error> {
    {
        let mut frame = t.get_frame();
        let selected_style = Style::default().fg(Color::Yellow).modifier(Modifier::Bold);
        let normal_style = Style::default().fg(Color::White);
        let header = ["Header1", "Header2", "Header3"];
        let rows = app.items.iter().enumerate().map(|(i, item)| {
            if i == app.selected {
                Row::StyledData(item.into_iter(), &selected_style)
            } else {
                Row::StyledData(item.into_iter(), &normal_style)
            }
        });

        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(5)
            .split(&app.size);
        Table::new(header.into_iter(), rows)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .widths(&[10, 10, 10])
            .render(&mut frame, &rects[0]);
    }
    t.draw()
}
