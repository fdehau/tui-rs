extern crate failure;
extern crate termion;
extern crate tui;

#[allow(dead_code)]
mod util;

use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Sparkline, Widget};
use tui::Terminal;

use util::event::{Event, Events};
use util::RandomSignal;

struct App {
    signal: RandomSignal,
    data1: Vec<u64>,
    data2: Vec<u64>,
    data3: Vec<u64>,
}

impl App {
    fn new() -> App {
        let mut signal = RandomSignal::new(0, 100);
        let data1 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data2 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data3 = signal.by_ref().take(200).collect::<Vec<u64>>();
        App {
            signal,
            data1,
            data2,
            data3,
        }
    }

    fn update(&mut self) {
        let value = self.signal.next().unwrap();
        self.data1.pop();
        self.data1.insert(0, value);
        let value = self.signal.next().unwrap();
        self.data2.pop();
        self.data2.insert(0, value);
        let value = self.signal.next().unwrap();
        self.data3.pop();
        self.data3.insert(0, value);
    }
}

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup event handlers
    let events = Events::new();

    // Create default app state
    let mut app = App::new();

    loop {
        let size = terminal.size()?;

        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(7),
                        Constraint::Min(0),
                    ]
                        .as_ref(),
                ).split(size);
            Sparkline::default()
                .block(
                    Block::default()
                        .title("Data1")
                        .borders(Borders::LEFT | Borders::RIGHT),
                ).data(&app.data1)
                .style(Style::default().fg(Color::Yellow))
                .render(&mut f, chunks[0]);
            Sparkline::default()
                .block(
                    Block::default()
                        .title("Data2")
                        .borders(Borders::LEFT | Borders::RIGHT),
                ).data(&app.data2)
                .style(Style::default().bg(Color::Green))
                .render(&mut f, chunks[1]);
            // Multiline
            Sparkline::default()
                .block(
                    Block::default()
                        .title("Data3")
                        .borders(Borders::LEFT | Borders::RIGHT),
                ).data(&app.data3)
                .style(Style::default().fg(Color::Red))
                .render(&mut f, chunks[2]);
        })?;

        match events.next()? {
            Event::Input(input) => if input == Key::Char('q') {
                break;
            },
            Event::Tick => {
                app.update();
            }
        }
    }

    Ok(())
}
