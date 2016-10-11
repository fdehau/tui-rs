
use buffer::Buffer;
use layout::Rect;
use style::Color;
use widgets::{Widget, WidgetType, Border, Line, vline, hline};

#[derive(Hash)]
pub struct Block<'a> {
    title: Option<&'a str>,
    borders: Border::Flags,
    border_fg: Color,
    border_bg: Color,
}

impl<'a> Default for Block<'a> {
    fn default() -> Block<'a> {
        Block {
            title: None,
            borders: Border::NONE,
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

    pub fn borders(&mut self, flag: Border::Flags) -> &mut Block<'a> {
        self.borders = flag;
        self
    }
}

impl<'a> Widget for Block<'a> {
    fn buffer(&self, area: &Rect) -> Buffer {

        let mut buf = Buffer::empty(*area);

        if area.area() == 0 {
            return buf;
        }

        if self.borders == Border::NONE {
            return buf;
        }

        // Sides
        if self.borders.intersects(Border::LEFT) {
            let line = vline(area.x, area.y, area.height, self.border_fg, self.border_bg);
            buf.merge(&line);
        }
        if self.borders.intersects(Border::TOP) {
            let line = hline(area.x, area.y, area.width, self.border_fg, self.border_bg);
            buf.merge(&line);
        }
        if self.borders.intersects(Border::RIGHT) {
            let line = vline(area.x + area.width - 1,
                             area.y,
                             area.height,
                             self.border_fg,
                             self.border_bg);
            buf.merge(&line);
        }
        if self.borders.intersects(Border::BOTTOM) {
            let line = hline(area.x,
                             area.y + area.height - 1,
                             area.width,
                             self.border_fg,
                             self.border_bg);
            buf.merge(&line);
        }

        // Corners
        if self.borders.contains(Border::LEFT | Border::TOP) {
            buf.set_symbol(0, 0, Line::TopLeft.get());
        }
        if self.borders.contains(Border::RIGHT | Border::TOP) {
            buf.set_symbol(area.width - 1, 0, Line::TopRight.get());
        }
        if self.borders.contains(Border::BOTTOM | Border::LEFT) {
            buf.set_symbol(0, area.height - 1, Line::BottomLeft.get());
        }
        if self.borders.contains(Border::BOTTOM | Border::RIGHT) {
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
