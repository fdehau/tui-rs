extern crate tui;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate termion;
extern crate rand;

use std::io;
use std::thread;
use std::env;
use std::time;
use std::sync::mpsc;

use rand::distributions::{IndependentSample, Range};

use termion::event;
use termion::input::TermRead;

use log::LogLevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

use tui::{Terminal, TermionBackend};
use tui::widgets::{Widget, Block, List, Gauge, Sparkline, Paragraph, border, Chart, Axis, Dataset,
                   BarChart, Marker, Tabs, Table};
use tui::widgets::canvas::{Canvas, Map, MapResolution, Line};
use tui::layout::{Group, Direction, Size, Rect};
use tui::style::{Style, Color, Modifier};

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
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
        SinSignal {
            x: 0.0,
            interval: interval,
            period: period,
            scale: scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

struct Server<'a> {
    name: &'a str,
    location: &'a str,
    coords: (f64, f64),
    status: &'a str,
}

struct MyTabs {
    titles: [&'static str; 2],
    selection: usize,
}

impl MyTabs {
    fn next(&mut self) {
        self.selection = (self.selection + 1) % self.titles.len();
    }

    fn previous(&mut self) {
        if self.selection > 0 {
            self.selection -= 1;
        }
    }
}

struct App<'a> {
    size: Rect,
    items: Vec<&'a str>,
    items2: Vec<&'a str>,
    selected: usize,
    tabs: MyTabs,
    show_chart: bool,
    progress: u16,
    data: Vec<u64>,
    data2: Vec<(f64, f64)>,
    data3: Vec<(f64, f64)>,
    data4: Vec<(&'a str, u64)>,
    window: [f64; 2],
    colors: [Color; 2],
    color_index: usize,
    servers: Vec<Server<'a>>,
}

enum Event {
    Input(event::Key),
    Tick,
}

fn main() {


    for argument in env::args() {
        if argument == "--log" {
            let log = FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new("{l} / {d(%H:%M:%S)} / \
                                                       {M}:{L}{n}{m}{n}{n}")))
                .build("demo.log")
                .unwrap();

            let config = Config::builder()
                .appender(Appender::builder().build("log", Box::new(log)))
                .build(Root::builder().appender("log").build(LogLevelFilter::Debug))
                .unwrap();
            log4rs::init_config(config).unwrap();
        }
    }

    info!("Start");

    let mut rand_signal = RandomSignal::new(Range::new(0, 100));
    let mut sin_signal = SinSignal::new(0.2, 3.0, 18.0);
    let mut sin_signal2 = SinSignal::new(0.1, 2.0, 10.0);

    let mut app = App {
        size: Rect::default(),
        items: vec!["Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8",
                    "Item9", "Item10", "Item11", "Item12", "Item13", "Item14", "Item15", "Item16",
                    "Item17", "Item18", "Item19", "Item20", "Item21", "Item22", "Item23", "Item24"],
        items2: vec!["Event1", "Event2", "Event3", "Event4", "Event5", "Event6", "Event7",
                     "Event8", "Event9", "Event10", "Event11", "Event12", "Event13", "Event14",
                     "Event15", "Event16", "Event17", "Event18", "Event19", "Event20", "Event21",
                     "Event22", "Event23", "Event24", "Event25", "Event26"],
        selected: 0,
        tabs: MyTabs {
            titles: ["Tab0", "Tab1"],
            selection: 0,
        },
        show_chart: true,
        progress: 0,
        data: rand_signal.clone().take(300).collect(),
        data2: sin_signal.clone().take(100).collect(),
        data3: sin_signal2.clone().take(200).collect(),
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
        window: [0.0, 20.0],
        colors: [Color::Magenta, Color::Red],
        color_index: 0,
        servers: vec![Server {
                          name: "NorthAmerica-1",
                          location: "New York City",
                          coords: (40.71, -74.00),
                          status: "Up",
                      },
                      Server {
                          name: "Europe-1",
                          location: "Paris",
                          coords: (48.85, 2.35),
                          status: "Failure",
                      },
                      Server {
                          name: "SouthAmerica-1",
                          location: "São Paulo",
                          coords: (-23.54, -46.62),
                          status: "Up",
                      },
                      Server {
                          name: "Asia-1",
                          location: "Singapore",
                          coords: (1.35, 103.86),
                          status: "Up",
                      }],
    };
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    for _ in 0..100 {
        sin_signal.next();
    }
    for _ in 0..200 {
        sin_signal2.next();
    }

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

    thread::spawn(move || {
        let tx = tx.clone();
        loop {
            tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    let mut terminal = Terminal::new(TermionBackend::new().unwrap()).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }
        draw(&mut terminal, &app).unwrap();
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
                    event::Key::Left => {
                        app.tabs.previous();
                    }
                    event::Key::Right => {
                        app.tabs.next();
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
                for _ in 0..5 {
                    app.data2.remove(0);
                    app.data2.push(sin_signal.next().unwrap());
                }
                for _ in 0..10 {
                    app.data3.remove(0);
                    app.data3.push(sin_signal2.next().unwrap());
                }
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
    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<TermionBackend>, app: &App) -> Result<(), io::Error> {

    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(3), Size::Min(0)])
        .render(t, &app.size, |t, chunks| {
            Tabs::default()
                .block(Block::default().borders(border::ALL).title("Tabs"))
                .titles(&app.tabs.titles)
                .style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(app.tabs.selection)
                .render(t, &chunks[0]);
            match app.tabs.selection {
                0 => {
                    draw_main(t, app, &chunks[1]);
                }
                1 => {
                    Group::default()
                        .direction(Direction::Horizontal)
                        .sizes(&[Size::Percent(30), Size::Percent(70)])
                        .render(t, &chunks[1], |t, chunks| {
                            Table::default()
                                .block(Block::default()
                                    .title("Servers")
                                    .borders(border::ALL))
                                .header(&["Server", "Location", "Status"])
                                .header_style(Style::default().fg(Color::Red))
                                .widths(&[15, 15, 10])
                                .rows(app.servers
                                    .iter()
                                    .map(|s| vec![s.name, s.location, s.status])
                                    .collect::<Vec<Vec<&str>>>())
                                .render(t, &chunks[0]);

                            Canvas::default()
                                .block(Block::default().title("World").borders(border::ALL))
                                .paint(|ctx| {
                                    ctx.draw(&Map {
                                        color: Color::White,
                                        resolution: MapResolution::High,
                                    });
                                    ctx.layer();
                                    for (i, s1) in app.servers.iter().enumerate() {
                                        for s2 in &app.servers[i + 1..] {
                                            ctx.draw(&Line {
                                                x1: s1.coords.1,
                                                y1: s1.coords.0,
                                                y2: s2.coords.0,
                                                x2: s2.coords.1,
                                                color: Color::Yellow,
                                            });
                                        }
                                    }
                                    for server in &app.servers {
                                        let color = if server.status == "Up" {
                                            Color::Green
                                        } else {
                                            Color::Red
                                        };
                                        ctx.print(server.coords.1, server.coords.0, "X", color);
                                    }
                                })
                                .x_bounds([-180.0, 180.0])
                                .y_bounds([-90.0, 90.0])
                                .render(t, &chunks[1]);
                        })
                }
                _ => {}
            };
        });
    try!(t.draw());
    Ok(())
}

fn draw_main(t: &mut Terminal<TermionBackend>, app: &App, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(7), Size::Min(7), Size::Fixed(7)])
        .render(t, area, |t, chunks| {
            Block::default()
                .borders(border::ALL)
                .title("Graphs")
                .render(t, &chunks[0]);
            Group::default()
                .direction(Direction::Vertical)
                .margin(1)
                .sizes(&[Size::Fixed(2), Size::Fixed(3)])
                .render(t, &chunks[0], |t, chunks| {
                    Gauge::default()
                        .block(Block::default().title("Gauge:"))
                        .style(Style::default().fg(Color::Magenta).bg(Color::Black).modifier(Modifier::Italic))
                        .percent(app.progress)
                        .render(t, &chunks[0]);
                    Sparkline::default()
                        .block(Block::default().title("Sparkline:"))
                        .style(Style::default().fg(Color::Green))
                        .data(&app.data)
                        .render(t, &chunks[1]);
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
                                        .items(&app.items2)
                                        .style(Style::default().fg(Color::Gray))
                                        .render(t, &chunks[1]);
                                });
                            BarChart::default()
                                .block(Block::default()
                                    .borders(border::ALL)
                                    .title("Bar chart"))
                                .data(&app.data4)
                                .bar_width(3)
                                .bar_gap(2)
                                .value_style(Style::default().fg(Color::Black).bg(Color::Green).modifier(Modifier::Italic))
                                .label_style(Style::default().fg(Color::Yellow))
                                .style(Style::default().fg(Color::Green))
                                .render(t, &chunks[1]);
                        });
                    if app.show_chart {
                        Chart::default()
                            .block(Block::default().title("Chart").borders(border::ALL))
                            .x_axis(Axis::default()
                                .title("X Axis")
                                .style(Style::default().fg(Color::Gray))
                                .labels_style(Style::default().modifier(Modifier::Italic))
                                .bounds(app.window)
                                .labels(&[&format!("{}", app.window[0]),
                                          &format!("{}", (app.window[0] + app.window[1]) / 2.0),
                                          &format!("{}", app.window[1])]))
                            .y_axis(Axis::default()
                                .title("Y Axis")
                                .style(Style::default().fg(Color::Gray))
                                .labels_style(Style::default().modifier(Modifier::Italic))
                                .bounds([-20.0, 20.0])
                                .labels(&["-20", "0", "20"]))
                            .datasets(&[Dataset::default()
                                            .name("data2")
                                            .marker(Marker::Dot)
                                            .style(Style::default().fg(Color::Cyan))
                                            .data(&app.data2),
                                        Dataset::default()
                                            .name("data3")
                                            .marker(Marker::Braille)
                                            .style(Style::default().fg(Color::Yellow))
                                            .data(&app.data3)])
                            .render(t, &chunks[1]);
                    }
                });
            Paragraph::default()
                .block(Block::default().borders(border::ALL).title("Footer"))
                .wrap(true)
                .style(Style::default().fg(app.colors[app.color_index]))
                .text("This is a paragraph with several lines.\nYou can change the color.\nUse \
                       \\{fg=[color];bg=[color];mod=[modifier] [text]} to highlight the text with a color. For example, {fg=red \
                       u}{fg=green n}{fg=yellow d}{fg=magenta e}{fg=cyan r} {fg=gray t}{fg=light_gray h}{fg=light_red \
                       e} {fg=light_green r}{fg=light_yellow a}{fg=light_magenta i}{fg=light_cyan n}{fg=white \
                       b}{fg=red o}{fg=green w}.\nOh, and if you didn't {mod=italic notice} you can {mod=bold automatically} \
                       {mod=invert wrap} your {mod=underline text} =).\nOne more thing is that it should display unicode \
                       characters properly: 日本国, ٩(-̮̮̃-̃)۶ ٩(●̮̮̃•̃)۶ ٩(͡๏̯͡๏)۶ ٩(-̮̮̃•̃).")
                .render(t, &chunks[2]);
        });
}
