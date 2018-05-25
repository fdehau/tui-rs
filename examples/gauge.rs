extern crate termion;
extern crate tui;

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Gauge, Widget};

struct App {
    size: Rect,
    progress1: u16,
    progress2: u16,
    progress3: u16,
    progress4: u16,
}

impl App {
    fn new() -> App {
        App {
            size: Rect::default(),
            progress1: 0,
            progress2: 0,
            progress3: 0,
            progress4: 0,
        }
    }

    fn advance(&mut self) {
        self.progress1 += 5;
        if self.progress1 > 100 {
            self.progress1 = 0;
        }
        self.progress2 += 10;
        if self.progress2 > 100 {
            self.progress2 = 0;
        }
        self.progress3 += 1;
        if self.progress3 > 100 {
            self.progress3 = 0;
        }
        self.progress4 += 3;
        if self.progress4 > 100 {
            self.progress4 = 0;
        }
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
    draw(&mut terminal, &app);

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }

        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => if input == event::Key::Char('q') {
                break;
            },
            Event::Tick => {
                app.advance();
            }
        }
        draw(&mut terminal, &app);
    }

    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<MouseBackend>, app: &App) {
    Group::default()
        .direction(Direction::Vertical)
        .margin(2)
        .sizes(&[
            Size::Percent(25),
            Size::Percent(25),
            Size::Percent(25),
            Size::Percent(25),
        ])
        .render(t, &app.size, |t, chunks| {
            Gauge::default()
                .block(Block::default().title("Gauge1").borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow))
                .percent(app.progress1)
                .render(t, &chunks[0]);
            Gauge::default()
                .block(Block::default().title("Gauge2").borders(Borders::ALL))
                .style(Style::default().fg(Color::Magenta).bg(Color::Green))
                .percent(app.progress2)
                .label(&format!("{}/100", app.progress2))
                .render(t, &chunks[1]);
            Gauge::default()
                .block(Block::default().title("Gauge2").borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow))
                .percent(app.progress3)
                .render(t, &chunks[2]);
            Gauge::default()
                .block(Block::default().title("Gauge3").borders(Borders::ALL))
                .style(Style::default().fg(Color::Cyan).modifier(Modifier::Italic))
                .percent(app.progress4)
                .label(&format!("{}/100", app.progress2))
                .render(t, &chunks[3]);
        });

    t.draw().unwrap();
}
