extern crate termion;
extern crate tui;

use std::io;
use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Paragraph, Widget};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Style};

fn main() {
    let mut terminal = Terminal::new(MouseBackend::new().unwrap()).unwrap();
    let stdin = io::stdin();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    let mut term_size = terminal.size().unwrap();
    draw(&mut terminal, &term_size);

    for c in stdin.keys() {
        let size = terminal.size().unwrap();
        if size != term_size {
            terminal.resize(size).unwrap();
            term_size = size;
        }

        draw(&mut terminal, &term_size);
        let evt = c.unwrap();
        if evt == event::Key::Char('q') {
            break;
        }
    }
    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<MouseBackend>, size: &Rect) {
    Block::default()
        .style(Style::default().bg(Color::White))
        .render(t, size);

    Group::default()
        .direction(Direction::Vertical)
        .margin(5)
        .sizes(&[Size::Percent(100)])
        .render(t, size, |t, chunks| {
            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(100)])
                .render(t, &chunks[0], |t, chunks| {
                    Paragraph::default()
                        .text(
                            "This is a line\n{fg=red This is a line}\n{bg=red This is a \
                             line}\n{mod=italic This is a line}\n{mod=bold This is a \
                             line}\n{mod=crossed_out This is a line}\n{mod=invert This is a \
                             line}\n{mod=underline This is a \
                             line}\n{bg=green;fg=yellow;mod=italic This is a line}\n",
                        )
                        .render(t, &chunks[0]);
                });
        });

    t.draw().unwrap();
}
