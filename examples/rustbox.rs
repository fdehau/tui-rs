extern crate rustbox;
extern crate tui;

use rustbox::Key;
use std::error::Error;

use tui::backend::RustboxBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Widget};
use tui::Terminal;

fn main() {
    let mut terminal = Terminal::new(RustboxBackend::new().unwrap()).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    draw(&mut terminal);
    loop {
        match terminal.backend().rustbox().poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => if key == Key::Char('q') {
                break;
            },
            Err(e) => panic!("{}", e.description()),
            _ => {}
        };
        draw(&mut terminal);
    }
    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<RustboxBackend>) {
    let size = t.size().unwrap();
    {
        let mut f = t.get_frame();
        Paragraph::default()
            .block(
                Block::default()
                    .title("Rustbox backend")
                    .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Magenta)),
            )
            .text("It {yellow works}!")
            .render(&mut f, &size);
    }

    t.draw().unwrap();
}
