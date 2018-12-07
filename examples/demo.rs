extern crate failure;
extern crate log;
extern crate stderrlog;
extern crate termion;
extern crate tui;

#[allow(dead_code)]
mod util;

use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution};
use tui::widgets::{
    Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, Marker, Paragraph, Row,
    SelectableList, Sparkline, Table, Tabs, Text, Widget,
};
use tui::{Frame, Terminal};

use util::event::{Event, Events};
use util::{RandomSignal, SinSignal, TabsState};

struct Server<'a> {
    name: &'a str,
    location: &'a str,
    coords: (f64, f64),
    status: &'a str,
}

struct App<'a> {
    items: Vec<&'a str>,
    events: Vec<(&'a str, &'a str)>,
    selected: usize,
    tabs: TabsState<'a>,
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

fn main() -> Result<(), failure::Error> {
    stderrlog::new()
        .module(module_path!())
        .verbosity(4)
        .init()?;

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut rand_signal = RandomSignal::new(0, 100);
    let mut sin_signal = SinSignal::new(0.2, 3.0, 18.0);
    let mut sin_signal2 = SinSignal::new(0.1, 2.0, 10.0);

    let mut app = App {
        items: vec![
            "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9",
            "Item10", "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17",
            "Item18", "Item19", "Item20", "Item21", "Item22", "Item23", "Item24",
        ],
        events: vec![
            ("Event1", "INFO"),
            ("Event2", "INFO"),
            ("Event3", "CRITICAL"),
            ("Event4", "ERROR"),
            ("Event5", "INFO"),
            ("Event6", "INFO"),
            ("Event7", "WARNING"),
            ("Event8", "INFO"),
            ("Event9", "INFO"),
            ("Event10", "INFO"),
            ("Event11", "CRITICAL"),
            ("Event12", "INFO"),
            ("Event13", "INFO"),
            ("Event14", "INFO"),
            ("Event15", "INFO"),
            ("Event16", "INFO"),
            ("Event17", "ERROR"),
            ("Event18", "ERROR"),
            ("Event19", "INFO"),
            ("Event20", "INFO"),
            ("Event21", "WARNING"),
            ("Event22", "INFO"),
            ("Event23", "INFO"),
            ("Event24", "WARNING"),
            ("Event25", "INFO"),
            ("Event26", "INFO"),
        ],
        selected: 0,
        tabs: TabsState::new(vec!["Tab0", "Tab1"]),
        show_chart: true,
        progress: 0,
        data: rand_signal.by_ref().take(300).collect(),
        data2: sin_signal.by_ref().take(100).collect(),
        data3: sin_signal2.by_ref().take(200).collect(),
        data4: vec![
            ("B1", 9),
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
            ("B24", 5),
        ],
        window: [0.0, 20.0],
        colors: [Color::Magenta, Color::Red],
        color_index: 0,
        servers: vec![
            Server {
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
            },
        ],
    };

    loop {
        // Draw UI
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.size());
            Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .titles(&app.tabs.titles)
                .style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(app.tabs.index)
                .render(&mut f, chunks[0]);
            match app.tabs.index {
                0 => draw_first_tab(&mut f, &app, chunks[1]),
                1 => draw_second_tab(&mut f, &app, chunks[1]),
                _ => {}
            };
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Up => {
                    if app.selected > 0 {
                        app.selected -= 1
                    };
                }
                Key::Down => {
                    if app.selected < app.items.len() - 1 {
                        app.selected += 1;
                    }
                }
                Key::Left => {
                    app.tabs.previous();
                }
                Key::Right => {
                    app.tabs.next();
                }
                Key::Char('t') => {
                    app.show_chart = !app.show_chart;
                }
                _ => {}
            },
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
                let i = app.events.pop().unwrap();
                app.events.insert(0, i);
                app.color_index += 1;
                if app.color_index >= app.colors.len() {
                    app.color_index = 0;
                }
            }
        }
    }
    Ok(())
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(7),
                Constraint::Min(7),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    draw_gauges(f, app, chunks[0]);
    draw_charts(f, app, chunks[1]);
    draw_text(f, chunks[2]);
}

fn draw_gauges<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(2), Constraint::Length(3)].as_ref())
        .margin(1)
        .split(area);
    Block::default()
        .borders(Borders::ALL)
        .title("Graphs")
        .render(f, area);
    Gauge::default()
        .block(Block::default().title("Gauge:"))
        .style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .modifier(Modifier::Italic),
        )
        .label(&format!("{} / 100", app.progress))
        .percent(app.progress)
        .render(f, chunks[0]);
    Sparkline::default()
        .block(Block::default().title("Sparkline:"))
        .style(Style::default().fg(Color::Green))
        .data(&app.data)
        .render(f, chunks[1]);
}

fn draw_charts<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let constraints = if app.show_chart {
        vec![Constraint::Percentage(50), Constraint::Percentage(50)]
    } else {
        vec![Constraint::Percentage(100)]
    };
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);
    {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);
        {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[0]);
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("List"))
                .items(&app.items)
                .select(Some(app.selected))
                .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold))
                .highlight_symbol(">")
                .render(f, chunks[0]);
            let info_style = Style::default().fg(Color::White);
            let warning_style = Style::default().fg(Color::Yellow);
            let error_style = Style::default().fg(Color::Magenta);
            let critical_style = Style::default().fg(Color::Red);
            let events = app.events.iter().map(|&(evt, level)| {
                Text::styled(
                    format!("{}: {}", level, evt),
                    match level {
                        "ERROR" => error_style,
                        "CRITICAL" => critical_style,
                        "WARNING" => warning_style,
                        _ => info_style,
                    },
                )
            });
            List::new(events)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .render(f, chunks[1]);
        }
        BarChart::default()
            .block(Block::default().borders(Borders::ALL).title("Bar chart"))
            .data(&app.data4)
            .bar_width(3)
            .bar_gap(2)
            .value_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .modifier(Modifier::Italic),
            )
            .label_style(Style::default().fg(Color::Yellow))
            .style(Style::default().fg(Color::Green))
            .render(f, chunks[1]);
    }
    if app.show_chart {
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
                    .data(&app.data2),
                Dataset::default()
                    .name("data3")
                    .marker(Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .data(&app.data3),
            ])
            .render(f, chunks[1]);
    }
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = [
        Text::raw("This is a paragraph with several lines. You can change style your text the way you want.\n\nFox example: "),
        Text::styled("under", Style::default().fg(Color::Red)),
        Text::raw(" "),
        Text::styled("the", Style::default().fg(Color::Green)),
        Text::raw(" "),
        Text::styled("rainbow", Style::default().fg(Color::Blue)),
        Text::raw(".\nOh and if you didn't "),
        Text::styled("notice", Style::default().modifier(Modifier::Italic)),
        Text::raw(" you can "),
        Text::styled("automatically", Style::default().modifier(Modifier::Bold)),
        Text::raw(" "),
        Text::styled("wrap", Style::default().modifier(Modifier::Invert)),
        Text::raw(" your "),
        Text::styled("text", Style::default().modifier(Modifier::Underline)),
        Text::raw(".\nOne more thing is that it should display unicode characters: 10€")
    ];
    Paragraph::new(text.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Footer")
                .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold)),
        )
        .wrap(true)
        .render(f, area);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);
    let up_style = Style::default().fg(Color::Green);
    let failure_style = Style::default().fg(Color::Red);
    let header = ["Server", "Location", "Status"];
    let rows = app.servers.iter().map(|s| {
        let style = if s.status == "Up" {
            up_style
        } else {
            failure_style
        };
        Row::StyledData(vec![s.name, s.location, s.status].into_iter(), style)
    });
    Table::new(header.into_iter(), rows)
        .block(Block::default().title("Servers").borders(Borders::ALL))
        .header_style(Style::default().fg(Color::Yellow))
        .widths(&[15, 15, 10])
        .render(f, chunks[0]);

    Canvas::default()
        .block(Block::default().title("World").borders(Borders::ALL))
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
        .render(f, chunks[1]);
}
