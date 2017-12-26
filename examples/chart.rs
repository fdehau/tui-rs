extern crate termion;
extern crate tui;

mod util;
use util::*;

use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Borders, Axis, Block, Chart, Dataset, Marker, Widget};
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};

struct App {
    size: Rect,
    signal1: SinSignal,
    data1: Vec<(f64, f64)>,
    signal2: SinSignal,
    data2: Vec<(f64, f64)>,
    window: [f64; 2],
}

impl App {
    fn new() -> App {
        let mut signal1 = SinSignal::new(0.2, 3.0, 18.0);
        let mut signal2 = SinSignal::new(0.1, 2.0, 10.0);
        let data1 = signal1.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        App {
            size: Rect::default(),
            signal1: signal1,
            data1: data1,
            signal2: signal2,
            data2: data2,
            window: [0.0, 20.0],
        }
    }

    fn advance(&mut self) {
        for _ in 0..5 {
            self.data1.remove(0);
        }
        self.data1.extend(self.signal1.by_ref().take(5));
        for _ in 0..10 {
            self.data2.remove(0);
        }
        self.data2.extend(self.signal2.by_ref().take(10));
        self.window[0] += 1.0;
        self.window[1] += 1.0;
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
        if app.size != size {
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
    Chart::default()
        .block(
            Block::default()
                .title("Chart")
                .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::Bold))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .labels_style(Style::default().modifier(Modifier::Italic))
                .bounds(app.window)
                .labels(&[
                    &format!("{}", app.window[0]),
                    &format!("{}", (app.window[0] + app.window[1]) / 2.0),
                    &format!("{}", app.window[1]),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .labels_style(Style::default().modifier(Modifier::Italic))
                .bounds([-20.0, 20.0])
                .labels(&["-20", "0", "20"]),
        )
        .datasets(&[
            Dataset::default()
                .name("data2")
                .marker(Marker::Dot)
                .style(Style::default().fg(Color::Cyan))
                .data(&app.data1),
            Dataset::default()
                .name("data3")
                .marker(Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .data(&app.data2),
        ])
        .render(t, &app.size);

    t.draw().unwrap();
}
