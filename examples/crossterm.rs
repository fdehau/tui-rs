extern crate crossterm;
extern crate failure;
extern crate tui;

use tui::backend::CrosstermBackend;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;

fn main() -> Result<(), failure::Error> {
    let mut terminal = Terminal::new(CrosstermBackend::new())?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        terminal.draw(|mut f| {
            let size = f.size();
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
                )
                .render(&mut f, size);
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
