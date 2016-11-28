extern crate tui;
extern crate termion;

use std::io;
use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, border};
use tui::layout::{Group, Direction, Size};
use tui::style::{Style, Color, Modifier};

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

    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    Block::default()
        .borders(border::ALL)
        .render(t, &size);
    Group::default()
        .direction(Direction::Vertical)
        .margin(4)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, &size, |t, chunks| {
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
                        .title_style(Style::default()
                            .fg(Color::White)
                            .bg(Color::Red)
                            .modifier(Modifier::Bold))
                        .render(t, &chunks[1]);
                });
            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(50), Size::Percent(50)])
                .render(t, &chunks[1], |t, chunks| {
                    Block::default()
                        .title("With borders")
                        .borders(border::ALL)
                        .render(t, &chunks[0]);
                    Block::default()
                        .title("With styled borders")
                        .border_style(Style::default().fg(Color::Cyan))
                        .borders(border::LEFT | border::RIGHT)
                        .render(t, &chunks[1]);
                });
        });

    t.draw().unwrap();
}
