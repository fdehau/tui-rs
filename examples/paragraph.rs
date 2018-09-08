extern crate termion;
extern crate tui;

use std::io;
use termion::event;
use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Paragraph, Text, Widget};
use tui::Terminal;

fn main() {
    let mut terminal = Terminal::new(MouseBackend::new().unwrap()).unwrap();
    let stdin = io::stdin();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    let mut term_size = terminal.size().unwrap();
    draw(&mut terminal, term_size).unwrap();

    for c in stdin.keys() {
        let size = terminal.size().unwrap();
        if size != term_size {
            terminal.resize(size).unwrap();
            term_size = size;
        }

        draw(&mut terminal, term_size).unwrap();
        let evt = c.unwrap();
        if evt == event::Key::Char('q') {
            break;
        }
    }
    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<MouseBackend>, size: Rect) -> Result<(), io::Error> {
    t.draw(|mut f| {
        Block::default()
            .style(Style::default().bg(Color::White))
            .render(&mut f, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(30),
                    Constraint::Percentage(30),
                ].as_ref(),
            )
            .split(size);

        let text = [
            Text::raw("This a line\n"),
            Text::styled("This a line\n", Style::default().fg(Color::Red)),
            Text::styled("This a line\n", Style::default().bg(Color::Blue)),
            Text::styled(
                "This a longer line\n",
                Style::default().modifier(Modifier::CrossedOut),
            ),
            Text::styled(
                "This a line\n",
                Style::default().fg(Color::Green).modifier(Modifier::Italic),
            ),
        ];

        Paragraph::new(text.iter())
            .alignment(Alignment::Left)
            .render(&mut f, chunks[0]);
        Paragraph::new(text.iter())
            .alignment(Alignment::Center)
            .wrap(true)
            .render(&mut f, chunks[1]);
        Paragraph::new(text.iter())
            .alignment(Alignment::Right)
            .wrap(true)
            .render(&mut f, chunks[2]);
    })
}
