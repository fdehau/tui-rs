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

use rand::distributions::{IndependentSample, Range};

use termion::event;
use termion::input::TermRead;

use log::LogLevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

use tui::Terminal;
use tui::widgets::{Widget, Block, List, Gauge, Sparkline, Text, border, Chart, Axis, Dataset,
                   BarChart};
use tui::layout::{Group, Direction, Size, Rect};
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
    type Item = (f64, f64);
    fn next(&mut self) -> Option<(f64, f64)> {
        self.x += 1.0;
        Some((self.x, ((self.x * 1.0 / self.period).sin() + 1.0) * self.scale))
    }
}

struct App<'a> {
    size: Rect,
    items: Vec<&'a str>,
    items2: Vec<&'a str>,
    selected: usize,
    show_chart: bool,
    progress: u16,
    data: Vec<u64>,
    data2: Vec<(f64, f64)>,
    data3: Vec<(f64, f64)>,
    data4: Vec<(&'a str, u64)>,
    window: [f64; 2],
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

    log4rs::init_config(config).unwrap();
    info!("Start");

    let mut rand_signal = RandomSignal::new(Range::new(0, 100));
    let mut sin_signal = SinSignal::new(4.0, 20.0);
    let mut sin_signal2 = SinSignal::new(2.0, 10.0);

    let mut app = App {
        size: Rect::default(),
        items: vec!["Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8",
                    "Item9", "Item10"],
        items2: vec!["Event1", "Event2", "Event3", "Event4", "Event5", "Event6", "Event7",
                     "Event8", "Event9", "Event10", "Event11", "Event12", "Event13", "Event14",
                     "Event15", "Event16", "Event17", "Event18", "Event19"],
        selected: 0,
        show_chart: true,
        progress: 0,
        data: rand_signal.clone().take(200).collect(),
        data2: sin_signal.clone().take(20).collect(),
        data3: sin_signal2.clone().take(20).collect(),
        data4: vec![("B1", 9),
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
                    ("B15", 4)],
        window: [0.0, 20.0],
        colors: [Color::Magenta, Color::Red],
        color_index: 0,
    };
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    for _ in 0..20 {
        sin_signal.next();
        sin_signal2.next();
    }

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
        let size = Terminal::size().unwrap();
        if size != app.size {
            terminal.resize(size);
            app.size = size;
        }
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
                app.data2.push(sin_signal.next().unwrap());
                app.data3.remove(0);
                app.data3.push(sin_signal2.next().unwrap());
                let i = app.data4.pop().unwrap();
                app.data4.insert(0, i);
                app.window[0] += 1.0;
                app.window[1] += 1.0;
                let i = app.items2.pop().unwrap();
                app.items2.insert(0, i);
                app.color_index += 1;
                if app.color_index >= app.colors.len() {
                    app.color_index = 0;
                }
            }
        }
    }
    terminal.show_cursor();
}

fn draw(t: &mut Terminal, app: &App) {


    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(7), Size::Min(7), Size::Fixed(7)])
        .render(t, &app.size, |t, chunks| {
            Block::default().borders(border::ALL).title("Graphs").render(&chunks[0], t);
            Group::default()
                .direction(Direction::Vertical)
                .margin(1)
                .sizes(&[Size::Fixed(2), Size::Fixed(3)])
                .render(t, &chunks[0], |t, chunks| {
                    Gauge::default()
                        .block(Block::default().title("Gauge:"))
                        .color(Color::Magenta)
                        .percent(app.progress)
                        .render(&chunks[0], t);
                    Sparkline::default()
                        .block(Block::default().title("Sparkline:"))
                        .color(Color::Green)
                        .data(&app.data)
                        .render(&chunks[1], t);
                });
            let sizes = if app.show_chart {
                vec![Size::Percent(50), Size::Percent(50)]
            } else {
                vec![Size::Percent(100)]
            };
            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&sizes)
                .render(t, &chunks[1], |t, chunks| {
                    Group::default()
                        .direction(Direction::Vertical)
                        .sizes(&[Size::Percent(50), Size::Percent(50)])
                        .render(t, &chunks[0], |t, chunks| {
                            Group::default()
                                .direction(Direction::Horizontal)
                                .sizes(&[Size::Percent(50), Size::Percent(50)])
                                .render(t, &chunks[0], |t, chunks| {
                                    List::default()
                                        .block(Block::default().borders(border::ALL).title("List"))
                                        .items(&app.items)
                                        .select(app.selected)
                                        .selection_color(Color::Yellow)
                                        .selection_symbol(">")
                                        .render(&chunks[0], t);
                                    List::default()
                                        .block(Block::default().borders(border::ALL).title("List"))
                                        .items(&app.items2)
                                        .render(&chunks[1], t);
                                });
                            BarChart::default()
                                .block(Block::default().borders(border::ALL).title("Bar chart"))
                                .data(&app.data4)
                                .bar_width(3)
                                .bar_gap(2)
                                .bar_color(Color::Green)
                                .value_color(Color::Black)
                                .label_color(Color::Yellow)
                                .render(&chunks[1], t);
                        });
                    if app.show_chart {
                        Chart::default()
                            .block(Block::default().title("Chart"))
                            .x_axis(Axis::default()
                                .title("X Axis")
                                .color(Color::Gray)
                                .bounds(app.window)
                                .labels(&[&format!("{}", app.window[0]),
                                          &format!("{}", (app.window[0] + app.window[1]) / 2.0),
                                          &format!("{}", app.window[1])]))
                            .y_axis(Axis::default()
                                .title("Y Axis")
                                .color(Color::Gray)
                                .bounds([0.0, 40.0])
                                .labels(&["0", "20", "40"]))
                            .datasets(&[Dataset::default().color(Color::Cyan).data(&app.data2),
                                        Dataset::default().color(Color::Yellow).data(&app.data3)])
                            .render(&chunks[1], t);
                    }
                });
            Text::default()
                .block(Block::default().borders(border::ALL).title("Footer"))
                .wrap(true)
                .fg(app.colors[app.color_index])
                .text("This a paragraph with several lines.\nYou can change the color.\nUse \
                       \\{[color] [text]} to highlight the text with the color. For example, \
                       {red u}{green n}{yellow d}{magenta e}{cyan r} {gray t}{light_gray \
                       h}{light_red e} {light_green r}{light_yellow a}{light_magenta \
                       i}{light_cyan n}{white b}{red o}{green w}.\nOh, and if you didn't notice \
                       you can automatically wrap your text =).")
                .render(&chunks[2], t);
        });
    t.finish();
}
