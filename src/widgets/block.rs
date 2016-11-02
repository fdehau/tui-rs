use buffer::Buffer;
use layout::Rect;
use style::Color;
use widgets::{Widget, border};
use symbols::line;

#[derive(Clone, Copy)]
pub struct Block<'a> {
    pub title: Option<&'a str>,
    pub title_color: Color,
    pub borders: border::Flags,
    pub border_color: Color,
    pub background_color: Color,
}

impl<'a> Default for Block<'a> {
    fn default() -> Block<'a> {
        Block {
            title: None,
            title_color: Color::Reset,
            borders: border::NONE,
            border_color: Color::Reset,
            background_color: Color::Reset,
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

    pub fn background_color(mut self, color: Color) -> Block<'a> {
        self.background_color = color;
        self
    }

    pub fn borders(mut self, flag: border::Flags) -> Block<'a> {
        self.borders = flag;
        self
    }

    pub fn inner(&self, area: &Rect) -> Rect {
        if area.width < 2 || area.height < 2 {
            return Rect::default();
        }
        let mut inner = *area;
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

impl<'a> Widget for Block<'a> {
    fn buffer(&self, area: &Rect, buf: &mut Buffer) {

        if area.width < 2 || area.height < 2 {
            return;
        }

        if self.background_color != Color::Reset {
            self.background(area, buf, self.background_color)
        }

        // Sides
        if self.borders.intersects(border::LEFT) {
            for y in area.top()..area.bottom() {
                buf.set_cell(area.left(),
                             y,
                             line::VERTICAL,
                             self.border_color,
                             self.background_color);
            }
        }
        if self.borders.intersects(border::TOP) {
            for x in area.left()..area.right() {
                buf.set_cell(x,
                             area.top(),
                             line::HORIZONTAL,
                             self.border_color,
                             self.background_color);
            }
        }
        if self.borders.intersects(border::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.set_cell(x,
                             y,
                             line::VERTICAL,
                             self.border_color,
                             self.background_color);
            }
        }
        if self.borders.intersects(border::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.set_cell(x,
                             y,
                             line::HORIZONTAL,
                             self.border_color,
                             self.background_color);
            }
        }

        // Corners
        if self.borders.contains(border::LEFT | border::TOP) {
            buf.set_symbol(area.left(), area.top(), line::TOP_LEFT);
        }
        if self.borders.contains(border::RIGHT | border::TOP) {
            buf.set_symbol(area.right() - 1, area.top(), line::TOP_RIGHT);
        }
        if self.borders.contains(border::LEFT | border::BOTTOM) {
            buf.set_symbol(area.left(), area.bottom() - 1, line::BOTTOM_LEFT);
        }
        if self.borders.contains(border::RIGHT | border::BOTTOM) {
            buf.set_symbol(area.right() - 1, area.bottom() - 1, line::BOTTOM_RIGHT);
        }

        if area.width > 2 {
            if let Some(title) = self.title {
                let lx = if self.borders.intersects(border::LEFT) {
                    1
                } else {
                    0
                };
                let rx = if self.borders.intersects(border::RIGHT) {
                    1
                } else {
                    0
                };
                let width = area.width - lx - rx;
                buf.set_stringn(area.left() + lx,
                                area.top(),
                                title,
                                width as usize,
                                self.title_color,
                                self.background_color);
            }
        }
    }
}
