extern crate termion;
extern crate tui;

use std::io;
use termion::event;
use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

fn main() {
    let mut terminal = Terminal::new(MouseBackend::new().unwrap()).unwrap();
    let stdin = io::stdin();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    let mut term_size = terminal.size().unwrap();
    draw(&mut terminal, &term_size);
    for c in stdin.keys() {
        let size = terminal.size().unwrap();
        if term_size != size {
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
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    Block::default().borders(Borders::ALL).render(t, size);
    Group::default()
        .direction(Direction::Vertical)
        .margin(4)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, size, |t, chunks| {
            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(50), Size::Percent(50)])
                .render(t, &chunks[0], |t, chunks| {
                    Block::default()
                        .title("With background")
                        .title_style(Style::default().fg(Color::Yellow))
                        .style(Style::default().bg(Color::Green))
                        .render(t, &chunks[0]);
                    Block::default()
                        .title("Styled title")
                        .title_style(
                            Style::default()
                                .fg(Color::White)
                                .bg(Color::Red)
                                .modifier(Modifier::Bold),
                        )
                        .render(t, &chunks[1]);
                });
            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(50), Size::Percent(50)])
                .render(t, &chunks[1], |t, chunks| {
                    Block::default()
                        .title("With borders")
                        .borders(Borders::ALL)
                        .render(t, &chunks[0]);
                    Block::default()
                        .title("With styled borders")
                        .border_style(Style::default().fg(Color::Cyan))
                        .borders(Borders::LEFT | Borders::RIGHT)
                        .render(t, &chunks[1]);
                });
        });

    t.draw().unwrap();
}
