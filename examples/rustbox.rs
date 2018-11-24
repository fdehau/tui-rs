extern crate rustbox;
extern crate tui;

use rustbox::Key;
use std::error::Error;

use tui::backend::RustboxBackend;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;

fn main() -> Result<(), failure::Error> {
    let mut terminal = Terminal::new(RustboxBackend::new().unwrap()).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    loop {
        draw(&mut terminal)?;
        match terminal.backend().rustbox().poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => if key == Key::Char('q') {
                break;
            },
            Err(e) => panic!("{}", e.description()),
            _ => {}
        };
    }
    terminal.show_cursor()?;
    Ok(())
}

fn draw(t: &mut Terminal<RustboxBackend>) -> Result<(), std::io::Error> {
    let size = t.size()?;
    let text = [
        Text::raw("It "),
        Text::styled("works", Style::default().fg(Color::Yellow)),
    ];
    t.draw(|mut f| {
        Paragraph::new(text.iter())
            .block(
                Block::default()
                    .title("Rustbox backend")
                    .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Magenta)),
            ).render(&mut f, size)
    })
}
