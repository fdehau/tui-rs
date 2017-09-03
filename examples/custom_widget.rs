extern crate tui;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::Widget;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;

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
        buf.set_string(area.left(), area.top(), self.text, &Style::default());
    }
}

impl<'a> Label<'a> {
    fn text(&mut self, text: &'a str) -> &mut Label<'a> {
        self.text = text;
        self
    }
}

fn main() {
    let mut terminal = Terminal::new(MouseBackend::new().unwrap()).unwrap();
    let size = terminal.size().unwrap();
    terminal.clear().unwrap();
    Label::default().text("Test").render(&mut terminal, &size);
    terminal.draw().unwrap();
}
