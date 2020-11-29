#[allow(dead_code)]
mod util;

use crate::util::{
    event::{Event, Events},
    SinSignal,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Terminal,
};

const DATA: [(f64, f64); 5] = [(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)];
const DATA2: [(f64, f64); 7] = [
    (0.0, 0.0),
    (10.0, 1.0),
    (20.0, 0.5),
    (30.0, 1.5),
    (40.0, 1.0),
    (50.0, 2.5),
    (60.0, 3.0),
];

struct App {
    signal1: SinSignal,
    data1: Vec<(f64, f64)>,
    signal2: SinSignal,
    data2: Vec<(f64, f64)>,
    window: [f64; 2],
    tick: usize,

    signal3: SinSignal,
    data3: Vec<(f64, f64)>,

    signal4: SinSignal,
    data4: Vec<(f64, f64)>,
    window4: [f64; 2],
}

fn update_data(data: &mut Vec<(f64, f64)>, signal: &mut SinSignal, amount: usize) {
    for _ in 0..amount {
        data.remove(0);
    }
    data.extend(signal.by_ref().take(amount));
}

impl App {
    fn new() -> App {
        let mut signal1 = SinSignal::new(0.2, 3.0, 18.0);
        let mut signal2 = SinSignal::new(0.1, 2.0, 10.0);
        let mut signal3 = SinSignal::new(0.2, 3.0, 18.0);
        let mut signal4 = SinSignal::new(0.1, 2.0, 10.0);
        let data1 = signal1.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(200).collect::<Vec<(f64, f64)>>();

        let data3 = signal3.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        let data4 = signal4.by_ref().take(200).collect::<Vec<(f64, f64)>>();

        App {
            signal1,
            data1,
            signal2,
            data2,
            window: [0.0, 20.0],
            tick: 0,
            signal3,
            data3,
            signal4,
            data4,
            window4: [0.0, 20.0],
        }
    }

    fn update(&mut self) {
        self.tick += 1;
        update_data(&mut self.data1, &mut self.signal1, 5);
        update_data(&mut self.data2, &mut self.signal2, 10);

        update_data(&mut self.data3, &mut self.signal3, 5);

        self.window[0] += 1.0;
        self.window[1] += 1.0;

        if self.tick == 5 {
            self.tick = 0;

            self.window4[0] += 1.0;
            self.window4[1] += 1.0;

            update_data(&mut self.data4, &mut self.signal4, 10);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    // App
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[2]);

            let x_labels = vec![
                Span::styled(
                    format!("{}", app.window[0]),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!("{}", (app.window[0] + app.window[1]) / 2.0)),
                Span::styled(
                    format!("{}", app.window[1]),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ];
            let datasets = vec![
                Dataset::default()
                    .name("data2")
                    .marker(symbols::Marker::Dot)
                    .style(Style::default().fg(Color::Cyan))
                    .data(&app.data1),
                Dataset::default()
                    .name("data3")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .data(&app.data2),
            ];

            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart 1",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("X Axis")
                        .style(Style::default().fg(Color::Gray))
                        .labels(x_labels)
                        .bounds(app.window),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        .labels(vec![
                            Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw("0"),
                            Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
                        ])
                        .bounds([-20.0, 20.0]),
                );
            f.render_widget(chart, chunks[0]);

            let datasets = vec![Dataset::default()
                .name("data")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .graph_type(GraphType::Line)
                .data(&DATA)];
            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart 2",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("X Axis")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 5.0])
                        .labels(vec![
                            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw("2.5"),
                            Span::styled("5.0", Style::default().add_modifier(Modifier::BOLD)),
                        ]),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 5.0])
                        .labels(vec![
                            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw("2.5"),
                            Span::styled("5.0", Style::default().add_modifier(Modifier::BOLD)),
                        ]),
                );
            f.render_widget(chart, chunks[1]);

            let datasets = vec![Dataset::default()
                .name("data")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .graph_type(GraphType::Line)
                .data(&DATA2)];
            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart 3",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("X Axis")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 50.0])
                        .labels(vec![
                            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw("25"),
                            Span::styled("50", Style::default().add_modifier(Modifier::BOLD)),
                        ]),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 5.0])
                        .labels(vec![
                            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw("2.5"),
                            Span::styled("5", Style::default().add_modifier(Modifier::BOLD)),
                        ]),
                );
            f.render_widget(chart, bottom_chunks[0]);

            let datasets = vec![
                Dataset::default()
                    .name("data3")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .data(&app.data3)
                    .x_axis_bounds(app.window),
                Dataset::default()
                    .name("data3")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .data(&app.data4)
                    .x_axis_bounds(app.window4),
            ];

            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart 4",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        .labels(vec![
                            Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw("0"),
                            Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
                        ])
                        .bounds([-20.0, 20.0]),
                );
            f.render_widget(chart, bottom_chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => {
                if input == Key::Char('q') {
                    break;
                }
            }
            Event::Tick => {
                app.update();
            }
        }
    }

    Ok(())
}
