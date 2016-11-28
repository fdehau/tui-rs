extern crate tui;
extern crate termion;

use std::io;
use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Paragraph};
use tui::layout::{Group, Direction, Size};
use tui::style::{Style, Color};

fn main() {
    let mut terminal = Terminal::new(TermionBackend::new().unwrap()).unwrap();
    let stdin = io::stdin();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    draw(&mut terminal);
    for c in stdin.keys() {
        draw(&mut terminal);
        let evt = c.unwrap();
        if evt == event::Key::Char('q') {
            break;
        }
    }
    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<TermionBackend>) {

    let size = t.size().unwrap();

    Block::default()
        .style(Style::default().bg(Color::White))
        .render(t, &size);

    Group::default()
        .direction(Direction::Vertical)
        .margin(5)
        .sizes(&[Size::Percent(100)])
        .render(t, &size, |t, chunks| {
            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(100)])
                .render(t, &chunks[0], |t, chunks| {
                    Paragraph::default()
                        .text("This is a line\n{fg=red This is a line}\n{bg=red This is a \
                               line}\n{mod=italic This is a line}\n{mod=bold This is a \
                               line}\n{mod=crossed_out This is a line}\n{mod=invert This is a \
                               line}\n{mod=underline This is a \
                               line}\n{bg=green;fg=yellow;mod=italic This is a line}\n")
                        .render(t, &chunks[0]);
                });
        });

    t.draw().unwrap();
}
