extern crate crossterm;
extern crate failure;
extern crate tui;

use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;

struct App {
    size: Rect,
}

impl Default for App {
    fn default() -> App {
        App {
            size: Rect::default(),
        }
    }
}

fn main() -> Result<(), failure::Error> {
    let mut terminal = Terminal::new(CrosstermBackend::new())?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    let mut app = App::default();
    loop {
        let size = terminal.size()?;
        if app.size != size {
            terminal.resize(size)?;
            app.size = size;
        }

        terminal.draw(|mut f| {
            let text = [
                Text::raw("It "),
                Text::styled("works", Style::default().fg(Color::Yellow)),
            ];
            Paragraph::new(text.iter())
                .block(
                    Block::default()
                        .title("Crossterm Backend")
                        .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Magenta)),
                ).render(&mut f, size);
        })?;

        let input = crossterm::input();
        match input.read_char()? {
            'q' => {
                break;
            }
            _ => {}
        };
    }
    Ok(())
}
