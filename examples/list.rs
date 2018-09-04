extern crate termion;
extern crate tui;

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time;

use termion::event;
use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::layout::{Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Item, List, SelectableList, Widget};
use tui::Terminal;

struct App<'a> {
    size: Rect,
    items: Vec<&'a str>,
    selected: Option<usize>,
    events: Vec<(&'a str, &'a str)>,
    info_style: Style,
    warning_style: Style,
    error_style: Style,
    critical_style: Style,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            size: Rect::default(),
            items: vec![
                "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9",
                "Item10", "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17",
                "Item18", "Item19", "Item20", "Item21", "Item22", "Item23", "Item24",
            ],
            selected: None,
            events: vec![
                ("Event1", "INFO"),
                ("Event2", "INFO"),
                ("Event3", "CRITICAL"),
                ("Event4", "ERROR"),
                ("Event5", "INFO"),
                ("Event6", "INFO"),
                ("Event7", "WARNING"),
                ("Event8", "INFO"),
                ("Event9", "INFO"),
                ("Event10", "INFO"),
                ("Event11", "CRITICAL"),
                ("Event12", "INFO"),
                ("Event13", "INFO"),
                ("Event14", "INFO"),
                ("Event15", "INFO"),
                ("Event16", "INFO"),
                ("Event17", "ERROR"),
                ("Event18", "ERROR"),
                ("Event19", "INFO"),
                ("Event20", "INFO"),
                ("Event21", "WARNING"),
                ("Event22", "INFO"),
                ("Event23", "INFO"),
                ("Event24", "WARNING"),
                ("Event25", "INFO"),
                ("Event26", "INFO"),
            ],
            info_style: Style::default().fg(Color::White),
            warning_style: Style::default().fg(Color::Yellow),
            error_style: Style::default().fg(Color::Magenta),
            critical_style: Style::default().fg(Color::Red),
        }
    }

    fn advance(&mut self) {
        let event = self.events.pop().unwrap();
        self.events.insert(0, event);
    }
}

enum Event {
    Input(event::Key),
    Tick,
}

fn main() {
    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();
    let clock_tx = tx.clone();

    // Input
    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    // Tick
    thread::spawn(move || loop {
        clock_tx.send(Event::Tick).unwrap();
        thread::sleep(time::Duration::from_millis(500));
    });

    // App
    let mut app = App::new();

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    draw(&mut terminal, &app).unwrap();

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }

        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input {
                event::Key::Char('q') => {
                    break;
                }
                event::Key::Left => {
                    app.selected = None;
                }
                event::Key::Down => {
                    app.selected = if let Some(selected) = app.selected {
                        if selected >= app.items.len() - 1 {
                            Some(0)
                        } else {
                            Some(selected + 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                event::Key::Up => {
                    app.selected = if let Some(selected) = app.selected {
                        if selected > 0 {
                            Some(selected - 1)
                        } else {
                            Some(app.items.len() - 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                _ => {}
            },
            Event::Tick => {
                app.advance();
            }
        }
        draw(&mut terminal, &app).unwrap();
    }

    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
}

fn draw(t: &mut Terminal<MouseBackend>, app: &App) -> Result<(), io::Error> {
    t.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(app.size);

        let style = Style::default().fg(Color::Black).bg(Color::White);
        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("List"))
            .items(&app.items)
            .select(app.selected)
            .style(style)
            .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::Bold))
            .highlight_symbol(">")
            .render(&mut f, chunks[0]);
        {
            let events = app.events.iter().map(|&(evt, level)| {
                Item::StyledData(
                    format!("{}: {}", level, evt),
                    match level {
                        "ERROR" => &app.error_style,
                        "CRITICAL" => &app.critical_style,
                        "WARNING" => &app.warning_style,
                        _ => &app.info_style,
                    },
                )
            });
            List::new(events)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .start_corner(Corner::BottomLeft)
                .render(&mut f, chunks[1]);
        }
    })
}
