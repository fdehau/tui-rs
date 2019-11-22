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
use tui::widgets::{PieChart, Block, Borders, Widget};
use tui::Terminal;

use crate::util::event::{Event, Events};

struct App {
    data: Vec<(f64, Color)>,
}

impl App {
    fn new() -> App {
        App {
            data: vec![
                (100.0, Color::Yellow),
                (66.67, Color::Blue),
                (75.0, Color::Green),
                (33.33, Color::Red),
            ],
        }
    }

    fn update(&mut self) {
        // let value = self.data.pop().unwrap();
        // self.data.insert(0, value);
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

    // App
    let mut app = App::new();

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());
            PieChart::default()
                .block(Block::default().title("Data1").borders(Borders::ALL))
                .data(&app.data)
                .background(Color::Black)
                .render(&mut f, chunks[0]);
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
