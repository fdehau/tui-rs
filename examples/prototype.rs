extern crate tui;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate termion;
extern crate rand;

use std::thread;
use std::time;
use std::sync::mpsc;
use std::io::stdin;

use termion::event;
use termion::input::TermRead;

use log::LogLevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

use tui::Terminal;
use tui::widgets::{Widget, Block, List, Gauge, Sparkline, border};
use tui::layout::{Group, Direction, Alignment, Size};
use tui::style::Color;

struct App {
    name: String,
    fetching: bool,
    items: Vec<String>,
    selected: usize,
    show_episodes: bool,
    progress: u16,
    data: Vec<u64>,
}

enum Event {
    Input(event::Key),
    Tick,
}

fn main() {

    let log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} / {d(%H:%M:%S)} / {M}:{L}{n}{m}{n}{n}")))
        .build("prototype.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("log", Box::new(log)))
        .build(Root::builder().appender("log").build(LogLevelFilter::Debug))
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();
    info!("Start");

    let mut app = App {
        name: String::from("Test app"),
        fetching: false,
        items: ["1", "2", "3"].into_iter().map(|e| String::from(*e)).collect(),
        selected: 0,
        show_episodes: false,
        progress: 0,
        data: (0..100).map(|_| rand::random::<u8>() as u64).collect(),
    };
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    thread::spawn(move || {
        let tx = tx.clone();
        loop {
            tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(1000));
        }
    });

    let mut terminal = Terminal::new().unwrap();
    terminal.clear();
    terminal.hide_cursor();

    loop {
        draw(&mut terminal, &app);
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                match input {
                    event::Key::Char('q') => {
                        break;
                    }
                    event::Key::Up => {
                        if app.selected > 0 {
                            app.selected -= 1
                        };
                    }
                    event::Key::Down => {
                        if app.selected < app.items.len() - 1 {
                            app.selected += 1;
                        }
                    }
                    event::Key::Char('t') => {
                        app.show_episodes = !app.show_episodes;
                    }
                    _ => {}
                }
            }
            Event::Tick => {
                app.progress += 5;
                if app.progress > 100 {
                    app.progress = 0;
                }
                app.data.insert(0, rand::random::<u8>() as u64);
                app.data.pop();
                app.selected += 1;
                if app.selected >= app.items.len() {
                    app.selected = 0;
                }
            }
        }
    }
    terminal.show_cursor();
}

fn draw(terminal: &mut Terminal, app: &App) {

    let ui = Group::default()
        .direction(Direction::Vertical)
        .alignment(Alignment::Left)
        .chunks(&[Size::Fixed(7), Size::Min(5), Size::Fixed(3)])
        .render(&terminal.area(), |chunks, tree| {
            tree.add(Block::default().borders(border::ALL).title("Graphs").render(&chunks[0]));
            tree.add(Group::default()
                .direction(Direction::Vertical)
                .alignment(Alignment::Left)
                .margin(1)
                .chunks(&[Size::Fixed(2), Size::Fixed(3)])
                .render(&chunks[0], |chunks, tree| {
                    tree.add(Gauge::new()
                        .block(*Block::default().title("Gauge:"))
                        .bg(Color::Yellow)
                        .percent(app.progress)
                        .render(&chunks[0]));
                    tree.add(Sparkline::new()
                        .block(*Block::default().title("Sparkline:"))
                        .fg(Color::Green)
                        .data(&app.data)
                        .render(&chunks[1]));
                }));
            let sizes = if app.show_episodes {
                vec![Size::Min(20), Size::Max(40)]
            } else {
                vec![Size::Min(20)]
            };
            tree.add(Group::default()
                .direction(Direction::Horizontal)
                .alignment(Alignment::Left)
                .chunks(&sizes)
                .render(&chunks[1], |chunks, tree| {
                    tree.add(List::default()
                        .block(*Block::default().borders(border::ALL).title("Podcasts"))
                        .items(&app.items)
                        .select(app.selected)
                        .formatter(|i, s| {
                            let (prefix, fg) = if s {
                                (">", Color::Cyan)
                            } else {
                                ("*", Color::White)
                            };
                            (format!("{} {}", prefix, i), fg, Color::Black)
                        })
                        .render(&chunks[0]));
                    if app.show_episodes {
                        tree.add(Block::default()
                            .borders(border::ALL)
                            .title("Episodes")
                            .render(&chunks[1]));
                    }
                }));
            tree.add(Block::default().borders(border::ALL).title("Footer").render(&chunks[2]));
        });
    terminal.render(ui);
}
