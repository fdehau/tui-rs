extern crate tui;
extern crate termion;

use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, border, SelectableList, List};
use tui::layout::{Group, Direction, Size};
use tui::style::{Style, Color, Modifier};

struct App<'a> {
    items: Vec<&'a str>,
    selected: usize,
    events: Vec<(&'a str, &'a str)>,
    info_style: Style,
    warning_style: Style,
    error_style: Style,
    critical_style: Style,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: vec!["Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8",
                        "Item9", "Item10", "Item11", "Item12", "Item13", "Item14", "Item15",
                        "Item16", "Item17", "Item18", "Item19", "Item20", "Item21", "Item22",
                        "Item23", "Item24"],
            selected: 0,
            events: vec![("Event1", "INFO"),
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
                         ("Event26", "INFO")],
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
    let backend = TermionBackend::new().unwrap();
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
    thread::spawn(move || {
        loop {
            clock_tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    // App
    let mut app = App::new();

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    draw(&mut terminal, &app);

    loop {
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                match input {
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
                }
            }
            Event::Tick => {
                app.advance();
            }
        }
        draw(&mut terminal, &app);
    }

    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<TermionBackend>, app: &App) {

    let size = t.size().unwrap();

    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, &size, |t, chunks| {
            SelectableList::default()
                .block(Block::default()
                    .borders(border::ALL)
                    .title("List"))
                .items(&app.items)
                .select(app.selected)
                .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold))
                .highlight_symbol(">")
                .render(t, &chunks[0]);
            List::default()
                .block(Block::default()
                    .borders(border::ALL)
                    .title("List"))
                .items(&app.events
                    .iter()
                    .map(|&(evt, level)| {
                        (format!("{}: {}", level, evt),
                         match level {
                            "ERROR" => &app.error_style,
                            "CRITICAL" => &app.critical_style,
                            "WARNING" => &app.warning_style,
                            _ => &app.info_style,
                        })
                    })
                    .collect::<Vec<(String, &Style)>>())
                .render(t, &chunks[1]);
        });

    t.draw().unwrap();
}
