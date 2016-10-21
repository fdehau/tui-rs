
use buffer::Buffer;
use layout::Rect;
use style::Color;
use widgets::{Widget, border};
use symbols::line;

#[derive(Clone, Copy)]
pub struct Block<'a> {
    title: Option<&'a str>,
    title_color: Color,
    borders: border::Flags,
    border_color: Color,
    bg: Color,
}

impl<'a> Default for Block<'a> {
    fn default() -> Block<'a> {
        Block {
            title: None,
            title_color: Color::Reset,
            borders: border::NONE,
            border_color: Color::Reset,
            bg: Color::Reset,
        }
    }
}

impl<'a> Block<'a> {
    pub fn title(mut self, title: &'a str) -> Block<'a> {
        self.title = Some(title);
        self
    }

    pub fn title_color(mut self, color: Color) -> Block<'a> {
        self.title_color = color;
        self
    }

    pub fn border_color(mut self, color: Color) -> Block<'a> {
        self.border_color = color;
        self
    }

    pub fn bg(mut self, color: Color) -> Block<'a> {
        self.bg = color;
        self
    }

    pub fn borders(mut self, flag: border::Flags) -> Block<'a> {
        self.borders = flag;
        self
    }

    pub fn inner(&self, area: Rect) -> Rect {
        if area.width < 2 || area.height < 2 {
            return Rect::default();
        }
        let mut inner = area;
        if self.borders.intersects(border::LEFT) {
            inner.x += 1;
            inner.width -= 1;
        }
        if self.borders.intersects(border::TOP) || self.title.is_some() {
            inner.y += 1;
            inner.height -= 1;
        }
        if self.borders.intersects(border::RIGHT) {
            inner.width -= 1;
        }
        if self.borders.intersects(border::BOTTOM) {
            inner.height -= 1;
        }
        inner
    }
}

impl<'a> Widget<'a> for Block<'a> {
    fn buffer(&'a self, area: &Rect) -> Buffer<'a> {

        let mut buf = Buffer::empty(*area);

        if area.width < 2 || area.height < 2 {
            return buf;
        }

        // Sides
        if self.borders.intersects(border::LEFT) {
            for y in 0..area.height {
                buf.update_cell(0, y, line::VERTICAL, self.border_color, self.bg);
            }
        }
        if self.borders.intersects(border::TOP) {
            for x in 0..area.width {
                buf.update_cell(x, 0, line::HORIZONTAL, self.border_color, self.bg);
            }
        }
        if self.borders.intersects(border::RIGHT) {
            let x = area.width - 1;
            for y in 0..area.height {
                buf.update_cell(x, y, line::VERTICAL, self.border_color, self.bg);
            }
        }
        if self.borders.intersects(border::BOTTOM) {
            let y = area.height - 1;
            for x in 0..area.width {
                buf.update_cell(x, y, line::HORIZONTAL, self.border_color, self.bg);
            }
        }

        // Corners
        if self.borders.contains(border::LEFT | border::TOP) {
            buf.set_symbol(0, 0, line::TOP_LEFT);
        }
        if self.borders.contains(border::RIGHT | border::TOP) {
            buf.set_symbol(area.width - 1, 0, line::TOP_RIGHT);
        }
        if self.borders.contains(border::BOTTOM | border::LEFT) {
            buf.set_symbol(0, area.height - 1, line::BOTTOM_LEFT);
        }
        if self.borders.contains(border::BOTTOM | border::RIGHT) {
            buf.set_symbol(area.width - 1, area.height - 1, line::BOTTOM_RIGHT);
        }
        if let Some(title) = self.title {
            let margin_x = if self.borders.intersects(border::LEFT) {
                1
            } else {
                0
            };
            buf.set_string(margin_x, 0, title, self.title_color, self.bg);
        }
        buf
    }
}
