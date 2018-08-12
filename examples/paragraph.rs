extern crate termion;
extern crate tui;

use std::io;
use termion::event;
use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Alignment, Color, Style};
use tui::widgets::{Block, Paragraph, Widget};
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
    {
        let mut f = t.get_frame();
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

        Paragraph::default()
            .alignment(Alignment::Left)
            .text(
                "This is a line\n{fg=red This is a line}\n{bg=red This is a \
                 line}\n{mod=italic This is a line}\n{mod=bold This is a \
                 line}\n{mod=crossed_out This is a line}\n{mod=invert This is a \
                 line}\n{mod=underline This is a \
                 line}\n{bg=green;fg=yellow;mod=italic This is a line}\n",
            )
            .render(&mut f, &chunks[0]);

        Paragraph::default()
            .alignment(Alignment::Center)
            .wrap(true)
            .text(
                "This is a line\n{fg=red This is a line}\n{bg=red This is a \
                 line}\n{mod=italic This is a line}\n{mod=bold This is a \
                 line}\n{mod=crossed_out This is a line}\n{mod=invert This is a \
                 line}\n{mod=underline This is a \
                 line}\n{bg=green;fg=yellow;mod=italic This is a line}\n",
            )
            .render(&mut f, &chunks[1]);
        Paragraph::default()
            .alignment(Alignment::Right)
            .wrap(true)
            .text(
                "This is a line\n{fg=red This is a line}\n{bg=red This is a \
                 line}\n{mod=italic This is a line}\n{mod=bold This is a \
                 line}\n{mod=crossed_out This is a line}\n{mod=invert This is a \
                 line}\n{mod=underline This is a \
                 line}\n{bg=green;fg=yellow;mod=italic This is a line}\n",
            )
            .render(&mut f, &chunks[2]);
    }
    t.draw().unwrap();
}
