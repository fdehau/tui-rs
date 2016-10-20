extern crate tui;
extern crate termion;

use std::io;
use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::widgets::{Widget, Block};
use tui::layout::{Group, Direction, Alignment, Size};

fn main() {
    let mut terminal = Terminal::new().unwrap();
    let stdin = io::stdin();
    terminal.clear();
    terminal.hide_cursor();
    for c in stdin.keys() {
        let evt = c.unwrap();
        if evt == event::Key::Char('q') {
            break;
        }
        draw(&mut terminal);
    }
    terminal.show_cursor();
}

fn draw(t: &mut Terminal) {

    Group::default()
        .direction(Direction::Vertical)
        .alignment(Alignment::Left)
        .chunks(&[Size::Fixed(7), Size::Min(5), Size::Fixed(3)])
        .render(t, &Terminal::size().unwrap(), |t, chunks| {
            Block::default().title("Block").render(&chunks[0], t);
            Group::default()
                .direction(Direction::Vertical)
                .alignment(Alignment::Left)
                .chunks(&[Size::Fixed(7), Size::Min(5), Size::Fixed(3)])
                .render(t, &chunks[1], |t, chunks| {
                    Block::default().title("Block").render(&chunks[0], t);
                });
        });
}
