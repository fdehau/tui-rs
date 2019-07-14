#[allow(dead_code)]
mod util;

use std::io;

use itui::{
    backend::TermionBackend, buffer::Buffer, layout::Rect, style::Style, widgets::Widget, Terminal,
};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};

use crate::util::event::{Event, Events};

struct Label<'a> {
    text: &'a str,
    area: Rect,
}

impl<'a> Default for Label<'a> {
    fn default() -> Label<'a> {
        Label {
            text: "",
            area: Default::default(),
        }
    }
}

impl<'a> Widget for Label<'a> {
    fn draw(&mut self, buf: &mut Buffer) {
        buf.set_string(
            self.area.left(),
            self.area.top(),
            self.text,
            Style::default(),
        );
    }
    fn get_area(&self) -> Rect {
        self.area
    }
}

impl<'a> Label<'a> {
    fn text(&mut self, text: &'a str) -> &mut Label<'a> {
        self.text = text;
        self
    }
    fn area(&mut self, area: Rect) -> &mut Self {
        self.area = area;
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
        terminal.draw(|mut f| {
            let size = f.size();
            Label::default().text("Test").area(size).render(&mut f);
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
