extern crate tui;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate termion;

use std::thread;
use std::sync::mpsc;
use std::io::{Write, stdin};

use termion::event;
use termion::input::TermRead;

use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

use tui::Terminal;
use tui::widgets::{Widget, Block, List, Border};
use tui::layout::{Group, Direction, Alignment, Size};

struct App {
    name: String,
    fetching: bool,
    items: Vec<String>,
    selected: usize,
    show_episodes: bool,
}

enum Event {
    Input(event::Key),
}

fn main() {

    let log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("prototype.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("log", Box::new(log)))
        .logger(Logger::builder()
            .appender("log")
            .additive(false)
            .build("log", LogLevelFilter::Info))
        .build(Root::builder().appender("log").build(LogLevelFilter::Info))
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();
    info!("Start");

    let mut app = App {
        name: String::from("Test app"),
        fetching: false,
        items: ["1", "2", "3"].into_iter().map(|e| String::from(*e)).collect(),
        selected: 0,
        show_episodes: false,
    };
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let tx = tx.clone();
        let stdin = stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
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
        }
    }
    terminal.show_cursor();
}

fn draw(terminal: &mut Terminal, app: &App) {

    let ui = Group::default()
        .direction(Direction::Vertical)
        .alignment(Alignment::Left)
        .chunks(&[Size::Fixed(3), Size::Percent(100), Size::Fixed(3)])
        .render(&terminal.area(), |chunks, tree| {
            tree.add(Block::default()
                .borders(Border::ALL)
                .title("Header")
                .render(&chunks[0]));
            let sizes = if app.show_episodes {
                vec![Size::Percent(50), Size::Percent(50)]
            } else {
                vec![Size::Percent(50), Size::Percent(50)]
            };
            tree.add(Group::default()
                .direction(Direction::Horizontal)
                .alignment(Alignment::Left)
                .chunks(&sizes)
                .render(&chunks[1], |chunks, tree| {
                    tree.add(List::default()
                        .block(|b| {
                            b.borders(Border::ALL).title("Podcasts");
                        })
                        .items(&app.items)
                        .select(app.selected)
                        .formatter(|i, s| {
                            let prefix = if s { ">" } else { "*" };
                            format!("{} {}", prefix, i)
                        })
                        .render(&chunks[0]));
                    if app.show_episodes {
                        tree.add(Block::default()
                            .borders(Border::ALL)
                            .title("Episodes")
                            .render(&chunks[1]));
                    }
                }));
            tree.add(Block::default().borders(Border::ALL).title("Footer").render(&chunks[2]));
        });
    terminal.render(ui);
}
