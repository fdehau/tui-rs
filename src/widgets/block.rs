
use buffer::Buffer;
use layout::Rect;
use style::Color;
use widgets::{Widget, WidgetType, border, Line, vline, hline};

#[derive(Hash, Clone, Copy)]
pub struct Block<'a> {
    title: Option<&'a str>,
    borders: border::Flags,
    border_fg: Color,
    border_bg: Color,
}

impl<'a> Default for Block<'a> {
    fn default() -> Block<'a> {
        Block {
            title: None,
            borders: border::NONE,
            border_fg: Color::White,
            border_bg: Color::Black,
        }
    }
}

impl<'a> Block<'a> {
    pub fn title(&mut self, title: &'a str) -> &mut Block<'a> {
        self.title = Some(title);
        self
    }

    pub fn borders(&mut self, flag: border::Flags) -> &mut Block<'a> {
        self.borders = flag;
        self
    }
}

impl<'a> Widget for Block<'a> {
    fn buffer(&self, area: &Rect) -> Buffer {

        let mut buf = Buffer::empty(*area);

        if self.borders == border::NONE {
            return buf;
        }

        if area.width < 2 || area.height < 2 {
            return buf;
        }

        // Sides
        if self.borders.intersects(border::LEFT) {
            let line = vline(area.x, area.y, area.height, self.border_fg, self.border_bg);
            buf.merge(&line);
        }
        if self.borders.intersects(border::TOP) {
            let line = hline(area.x, area.y, area.width, self.border_fg, self.border_bg);
            buf.merge(&line);
        }
        if self.borders.intersects(border::RIGHT) {
            let line = vline(area.x + area.width - 1,
                             area.y,
                             area.height,
                             self.border_fg,
                             self.border_bg);
            buf.merge(&line);
        }
        if self.borders.intersects(border::BOTTOM) {
            let line = hline(area.x,
                             area.y + area.height - 1,
                             area.width,
                             self.border_fg,
                             self.border_bg);
            buf.merge(&line);
        }

        // Corners
        if self.borders.contains(border::LEFT | border::TOP) {
            buf.set_symbol(0, 0, Line::TopLeft.get());
        }
        if self.borders.contains(border::RIGHT | border::TOP) {
            buf.set_symbol(area.width - 1, 0, Line::TopRight.get());
        }
        if self.borders.contains(border::BOTTOM | border::LEFT) {
            buf.set_symbol(0, area.height - 1, Line::BottomLeft.get());
        }
        if self.borders.contains(border::BOTTOM | border::RIGHT) {
            buf.set_symbol(area.width - 1, area.height - 1, Line::BottomRight.get());
        }
        if let Some(ref title) = self.title {
            buf.set_string(1, 0, &format!(" {} ", title));
        }
        buf
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Block
    }
}
