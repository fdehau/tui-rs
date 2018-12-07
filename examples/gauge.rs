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
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Gauge, Widget};
use tui::Terminal;

use util::event::{Event, Events};

struct App {
    progress1: u16,
    progress2: u16,
    progress3: f64,
    progress4: u16,
}

impl App {
    fn new() -> App {
        App {
            progress1: 0,
            progress2: 0,
            progress3: 0.0,
            progress4: 0,
        }
    }

    fn update(&mut self) {
        self.progress1 += 5;
        if self.progress1 > 100 {
            self.progress1 = 0;
        }
        self.progress2 += 10;
        if self.progress2 > 100 {
            self.progress2 = 0;
        }
        self.progress3 += 0.001;
        if self.progress3 > 1.0 {
            self.progress3 = 0.0;
        }
        self.progress4 += 3;
        if self.progress4 > 100 {
            self.progress4 = 0;
        }
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

    let events = Events::new();

    let mut app = App::new();

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            Gauge::default()
                .block(Block::default().title("Gauge1").borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow))
                .percent(app.progress1)
                .render(&mut f, chunks[0]);
            Gauge::default()
                .block(Block::default().title("Gauge2").borders(Borders::ALL))
                .style(Style::default().fg(Color::Magenta).bg(Color::Green))
                .percent(app.progress2)
                .label(&format!("{}/100", app.progress2))
                .render(&mut f, chunks[1]);
            Gauge::default()
                .block(Block::default().title("Gauge3").borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow))
                .ratio(app.progress3)
                .render(&mut f, chunks[2]);
            Gauge::default()
                .block(Block::default().title("Gauge4").borders(Borders::ALL))
                .style(Style::default().fg(Color::Cyan).modifier(Modifier::Italic))
                .percent(app.progress4)
                .label(&format!("{}/100", app.progress2))
                .render(&mut f, chunks[3]);
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
