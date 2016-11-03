extern crate tui;

use tui::Terminal;
use tui::widgets::Widget;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Color;

struct Label<'a> {
    text: &'a str,
}

impl<'a> Default for Label<'a> {
    fn default() -> Label<'a> {
        Label { text: "" }
    }
}

impl<'a> Widget for Label<'a> {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {
        buf.set_string(area.left(),
                       area.top(),
                       self.text,
                       Color::Reset,
                       Color::Reset);
    }
}

impl<'a> Label<'a> {
    fn text(&mut self, text: &'a str) -> &mut Label<'a> {
        self.text = text;
        self
    }
}

fn main() {
    let mut terminal = Terminal::new().unwrap();
    terminal.clear().unwrap();
    Label::default().text("Test").render(&Terminal::size().unwrap(), &mut terminal);
    terminal.draw().unwrap();
}
