extern crate tui;
extern crate termion;

use std::io;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, border, Table};
use tui::layout::{Group, Direction, Size, Rect};
use tui::style::{Style, Color, Modifier};

struct App<'a> {
    size: Rect,
    items: Vec<Vec<&'a str>>,
    selected: usize,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            size: Rect::default(),
            items: vec![vec!["Row12", "Row12", "Row13"],
                        vec!["Row21", "Row22", "Row23"],
                        vec!["Row31", "Row32", "Row33"],
                        vec!["Row41", "Row42", "Row43"],
                        vec!["Row41", "Row42", "Row43"],
                        vec!["Row41", "Row42", "Row43"]],
            selected: 0,
        }
    }
}

fn main() {
    // Terminal initialization
    let backend = TermionBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();


    // App
    let mut app = App::new();

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    draw(&mut terminal, &app);

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
            event::Key::Up => {
                if app.selected > 0 {
                    app.selected -= 1;
                } else {
                    app.selected = app.items.len() - 1;
                }
            }
            _ => {}
        };
        draw(&mut terminal, &app);
    }

    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<TermionBackend>, app: &App) {

    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Percent(100)])
        .margin(5)
        .render(t, &app.size, |t, chunks| {
            let selected_style = Style::default().fg(Color::Yellow).modifier(Modifier::Bold);
            let normal_style = Style::default().fg(Color::White);
            Table::default()
                .block(Block::default().borders(border::ALL).title("Table"))
                .header(&["Header1", "Header2", "Header3"])
                .widths(&[10, 10, 10])
                .rows(&app.items
                           .iter()
                           .enumerate()
                           .map(|(i, item)| {
                                    (item,
                                     if i == app.selected {
                                         &selected_style
                                     } else {
                                         &normal_style
                                     })
                                })
                           .collect::<Vec<(&Vec<&str>, &Style)>>())
                .render(t, &chunks[0]);
        });

    t.draw().unwrap();
}
