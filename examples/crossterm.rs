extern crate crossterm;
extern crate tui;

use std::error::Error;
use std::io;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;

fn main() {
    let mut terminal = Terminal::new(CrosstermBackend::new()).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    draw(&mut terminal).unwrap();
    loop {
        {
            let input = crossterm::input(terminal.backend().screen());
            match input.read_char() {
                Ok(c) => if c == 'q' {
                    break;
                },
                Err(e) => panic!("{}", e.description()),
            };
        }
        draw(&mut terminal).unwrap();
    }
    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<CrosstermBackend>) -> io::Result<()> {
    let size = t.size()?;
    t.draw(|mut f| {
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
    })
}
