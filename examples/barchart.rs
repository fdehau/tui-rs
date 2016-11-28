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
use tui::widgets::{Widget, Block, border, BarChart};
use tui::layout::{Group, Direction, Size};
use tui::style::{Style, Color, Modifier};

struct App<'a> {
    data: Vec<(&'a str, u64)>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            data: vec![("B1", 9),
                       ("B2", 12),
                       ("B3", 5),
                       ("B4", 8),
                       ("B5", 2),
                       ("B6", 4),
                       ("B7", 5),
                       ("B8", 9),
                       ("B9", 14),
                       ("B10", 15),
                       ("B11", 1),
                       ("B12", 0),
                       ("B13", 4),
                       ("B14", 6),
                       ("B15", 4),
                       ("B16", 6),
                       ("B17", 4),
                       ("B18", 7),
                       ("B19", 13),
                       ("B20", 8),
                       ("B21", 11),
                       ("B22", 9),
                       ("B23", 3),
                       ("B24", 5)],
        }
    }

    fn advance(&mut self) {
        let value = self.data.pop().unwrap();
        self.data.insert(0, value);
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
                if input == event::Key::Char('q') {
                    break;
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
        .direction(Direction::Vertical)
        .margin(2)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, &size, |t, chunks| {
            BarChart::default()
                .block(Block::default().title("Data1").borders(border::ALL))
                .data(&app.data)
                .bar_width(9)
                .style(Style::default().fg(Color::Yellow))
                .value_style(Style::default().fg(Color::Black).bg(Color::Yellow))
                .render(t, &chunks[0]);
            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(50), Size::Percent(50)])
                .render(t, &chunks[1], |t, chunks| {
                    BarChart::default()
                        .block(Block::default().title("Data2").borders(border::ALL))
                        .data(&app.data)
                        .bar_width(5)
                        .bar_gap(3)
                        .style(Style::default().fg(Color::Green))
                        .value_style(Style::default().bg(Color::Green).modifier(Modifier::Bold))
                        .render(t, &chunks[0]);
                    BarChart::default()
                        .block(Block::default().title("Data3").borders(border::ALL))
                        .data(&app.data)
                        .style(Style::default().fg(Color::Red))
                        .bar_width(7)
                        .bar_gap(0)
                        .value_style(Style::default().bg(Color::Red))
                        .label_style(Style::default().fg(Color::Cyan).modifier(Modifier::Italic))
                        .render(t, &chunks[1]);
                })
        });

    t.draw().unwrap();
}
