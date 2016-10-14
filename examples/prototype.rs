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
use std::cmp::min;

use rand::distributions::{IndependentSample, Range};

use termion::event;
use termion::input::TermRead;

use log::LogLevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

use tui::Terminal;
use tui::widgets::{Widget, Block, List, Gauge, Sparkline, Text, border, Chart};
use tui::layout::{Group, Direction, Alignment, Size};
use tui::style::Color;

#[derive(Clone)]
struct RandomSignal {
    range: Range<u64>,
    rng: rand::ThreadRng,
}

impl RandomSignal {
    fn new(r: Range<u64>) -> RandomSignal {
        RandomSignal {
            range: r,
            rng: rand::thread_rng(),
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.range.ind_sample(&mut self.rng))
    }
}

#[derive(Clone)]
struct SinSignal {
    x: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    fn new(period: f64, scale: f64) -> SinSignal {
        SinSignal {
            x: 0.0,
            period: period,
            scale: scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        self.x += 1.0;
        Some(((self.x * 1.0 / self.period).sin() + 1.0) * self.scale)
    }
}

struct App {
    name: String,
    fetching: bool,
    items: Vec<String>,
    selected: usize,
    show_chart: bool,
    progress: u16,
    data: Vec<u64>,
    data2: Vec<u64>,
    colors: [Color; 2],
    color_index: usize,
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

    let mut rand_signal = RandomSignal::new(Range::new(0, 100));
    let mut sin_signal = SinSignal::new(4.0, 20.0);

    let mut app = App {
        name: String::from("Test app"),
        fetching: false,
        items: ["1", "2", "3"].into_iter().map(|e| String::from(*e)).collect(),
        selected: 0,
        show_chart: true,
        progress: 0,
        data: rand_signal.clone().take(100).collect(),
        data2: sin_signal.clone().take(100).map(|i| i as u64).collect(),
        colors: [Color::Magenta, Color::Red],
        color_index: 0,
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
            thread::sleep(time::Duration::from_millis(500));
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
                        app.show_chart = !app.show_chart;
                    }
                    _ => {}
                }
            }
            Event::Tick => {
                app.progress += 5;
                if app.progress > 100 {
                    app.progress = 0;
                }
                app.data.insert(0, rand_signal.next().unwrap());
                app.data.pop();
                app.data2.remove(0);
                app.data2.push(sin_signal.next().unwrap() as u64);
                app.selected += 1;
                if app.selected >= app.items.len() {
                    app.selected = 0;
                }
                app.color_index += 1;
                if app.color_index >= app.colors.len() {
                    app.color_index = 0;
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
        .render(&Terminal::size().unwrap(), |chunks, tree| {
            tree.add(Block::default().borders(border::ALL).title("Graphs").render(&chunks[0]));
            tree.add(Group::default()
                .direction(Direction::Vertical)
                .alignment(Alignment::Left)
                .margin(1)
                .chunks(&[Size::Fixed(2), Size::Fixed(3)])
                .render(&chunks[0], |chunks, tree| {
                    tree.add(Gauge::default()
                        .block(*Block::default().title("Gauge:"))
                        .bg(Color::Yellow)
                        .percent(app.progress)
                        .render(&chunks[0]));
                    tree.add(Sparkline::default()
                        .block(*Block::default().title("Sparkline:"))
                        .fg(Color::Green)
                        .data(&app.data)
                        .render(&chunks[1]));
                }));
            let sizes = if app.show_chart {
                vec![Size::Max(40), Size::Min(20)]
            } else {
                vec![Size::Max(40)]
            };
            tree.add(Group::default()
                .direction(Direction::Horizontal)
                .alignment(Alignment::Left)
                .chunks(&sizes)
                .render(&chunks[1], |chunks, tree| {
                    tree.add(List::default()
                        .block(*Block::default().borders(border::ALL).title("List"))
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
                    if app.show_chart {
                        tree.add(Chart::default()
                            .block(*Block::default()
                                .borders(border::ALL)
                                .title("Chart"))
                            .fg(Color::Cyan)
                            .axis([0, 40])
                            .data(&app.data2)
                            .render(&chunks[1]));
                    }
                }));
            tree.add(Text::default()
                .block(*Block::default().borders(border::ALL).title("Footer"))
                .fg(app.colors[app.color_index])
                .text("This is a footer")
                .render(&chunks[2]));
        });
    terminal.render(ui);
}
