extern crate tui;
extern crate termion;

use std::thread;
use std::sync::mpsc;
use std::io::{Write, stdin};

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::widgets::{Widget, Block, Border};
use tui::layout::{Group, Direction, Alignment, Size};

struct App {
    name: String,
    fetching: bool,
}

enum Event {
    Quit,
    Redraw,
}

fn main() {

    let mut app = App {
        name: String::from("Test app"),
        fetching: false,
    };
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let tx = tx.clone();
        let stdin = stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            match evt {
                event::Key::Char('q') => {
                    tx.send(Event::Quit).unwrap();
                    break;
                }
                event::Key::Char('r') => {
                    tx.send(Event::Redraw).unwrap();
                }
                _ => {}
            }
        }
    });
    let mut terminal = Terminal::new().unwrap();
    terminal.clear();
    terminal.hide_cursor();
    loop {
        draw(&mut terminal, &app);
        let evt = rx.recv().unwrap();
        match evt {
            Event::Quit => {
                break;
            }
            Event::Redraw => {}
        }
    }
    terminal.show_cursor();
}

fn draw(terminal: &mut Terminal, app: &App) {

    let ui = Group::default()
        .direction(Direction::Vertical)
        .alignment(Alignment::Left)
        .chunks(&[Size::Fixed(3.0), Size::Percent(100.0), Size::Fixed(3.0)])
        .render(&terminal.area(), |chunks| {
            vec![Block::default()
                     .borders(Border::TOP | Border::BOTTOM)
                     .title("Header")
                     .render(&chunks[0]),
                 Group::default()
                     .direction(Direction::Horizontal)
                     .alignment(Alignment::Left)
                     .chunks(&[Size::Percent(50.0), Size::Percent(50.0)])
                     .render(&chunks[1], |chunks| {
                    vec![Block::default()
                             .borders(Border::ALL)
                             .title("Podcasts")
                             .render(&chunks[0]),
                         Block::default()
                             .borders(Border::ALL)
                             .title("Episodes")
                             .render(&chunks[1])]
                }),
                 Block::default().borders(Border::ALL).title("Footer").render(&chunks[2])]
        });
    terminal.render(&ui);
}
