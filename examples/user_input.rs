/// A simple example demonstrating how to handle user input. This is
/// a bit out of the scope of the library as it does not provide any
/// input handling out of the box. However, it may helps some to get
/// started.
///
/// This is a very simple example:
///   * A input box always focused. Every character you type is registered
///   here
///   * Pressing Backspace erases a character
///   * Pressing Enter pushes the current input in the history of previous
///   messages
extern crate termion;
extern crate tui;

use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event;
use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Item, List, Paragraph, Widget};
use tui::Terminal;

struct App {
    size: Rect,
    input: String,
    messages: Vec<String>,
}

impl App {
    fn new() -> App {
        App {
            size: Rect::default(),
            input: String::new(),
            messages: Vec::new(),
        }
    }
}

enum Event {
    Input(event::Key),
}

fn main() {
    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

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

    // App
    let mut app = App::new();

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    draw(&mut terminal, &app);

    loop {
        let size = terminal.size().unwrap();
        if app.size != size {
            terminal.resize(size).unwrap();
            app.size = size;
        }

        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input {
                event::Key::Char('q') => {
                    break;
                }
                event::Key::Char('\n') => {
                    app.messages.push(app.input.drain(..).collect());
                }
                event::Key::Char(c) => {
                    app.input.push(c);
                }
                event::Key::Backspace => {
                    app.input.pop();
                }
                _ => {}
            },
        }
        draw(&mut terminal, &app);
    }

    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
}

fn draw(t: &mut Terminal<MouseBackend>, app: &App) {
    {
        let mut f = t.get_frame();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
            .split(&app.size);
        Paragraph::default()
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Input"))
            .text(&app.input)
            .render(&mut f, &chunks[0]);
        List::new(
            app.messages
                .iter()
                .enumerate()
                .map(|(i, m)| Item::Data(format!("{}: {}", i, m))),
        ).block(Block::default().borders(Borders::ALL).title("Messages"))
            .render(&mut f, &chunks[1]);
    }

    t.draw().unwrap();
}
