extern crate tui;
extern crate rustbox;

use std::error::Error;
use rustbox::Key;

use tui::{Terminal, RustboxBackend};
use tui::widgets::{Widget, Block, border, Paragraph};
use tui::layout::{Group, Direction, Size};
use tui::style::{Style, Color, Modifier};

fn main() {
    let mut terminal = Terminal::new(RustboxBackend::new().unwrap()).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    draw(&mut terminal);
    loop {
        match terminal.backend().rustbox().poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                if key == Key::Char('q') {
                    break;
                }
            }
            Err(e) => panic!("{}", e.description()),
            _ => {}
        };
        draw(&mut terminal);
    }
    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<RustboxBackend>) {

    let size = t.size().unwrap();

    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(100)])
        .render(t, &size, |t, chunks| {
            Paragraph::default()
                .block(Block::default()
                    .title("Rustbox backend")
                    .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold))
                    .borders(border::ALL)
                    .border_style(Style::default().fg(Color::Magenta)))
                .text("It {yellow works}!")
                .render(t, &chunks[0]);
        });

    t.draw().unwrap();
}
