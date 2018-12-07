extern crate failure;
extern crate termion;
extern crate tui;

#[allow(dead_code)]
mod util;

use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::Widget;
use tui::Terminal;

use util::event::{Event, Events};

struct Label<'a> {
    text: &'a str,
}

impl<'a> Default for Label<'a> {
    fn default() -> Label<'a> {
        Label { text: "" }
    }
}

impl<'a> Widget for Label<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), self.text, Style::default());
    }
}

impl<'a> Label<'a> {
    fn text(&mut self, text: &'a str) -> &mut Label<'a> {
        self.text = text;
        self
    }
}

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
        let size = terminal.size()?;

        terminal.draw(|mut f| {
            Label::default().text("Test").render(&mut f, size);
        })?;

        match events.next()? {
            Event::Input(key) => {
                if key == Key::Char('q') {
                    break;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
