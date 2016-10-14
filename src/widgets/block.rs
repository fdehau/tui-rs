
use buffer::Buffer;
use layout::Rect;
use style::Color;
use widgets::{Widget, WidgetType, border, Line, vline, hline};

#[derive(Hash, Clone, Copy)]
pub struct Block<'a> {
    title: Option<&'a str>,
    title_fg: Color,
    title_bg: Color,
    borders: border::Flags,
    border_fg: Color,
    border_bg: Color,
}

impl<'a> Default for Block<'a> {
    fn default() -> Block<'a> {
        Block {
            title: None,
            title_fg: Color::White,
            title_bg: Color::Black,
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

    pub fn title_fg(&mut self, color: Color) -> &mut Block<'a> {
        self.title_fg = color;
        self
    }

    pub fn title_bg(&mut self, color: Color) -> &mut Block<'a> {
        self.title_bg = color;
        self
    }

    pub fn border_fg(&mut self, color: Color) -> &mut Block<'a> {
        self.border_fg = color;
        self
    }

    pub fn border_bg(&mut self, color: Color) -> &mut Block<'a> {
        self.border_bg = color;
        self
    }


    pub fn borders(&mut self, flag: border::Flags) -> &mut Block<'a> {
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

impl<'a> Widget for Block<'a> {
    fn buffer(&self, area: &Rect) -> Buffer {

        let mut buf = Buffer::empty(*area);

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
        if let Some(title) = self.title {
            let (margin_x, string) = if self.borders.intersects(border::LEFT) {
                (1, format!(" {} ", title))
            } else {
                (0, String::from(title))
            };
            buf.set_string(margin_x, 0, &string, self.title_fg, self.title_bg);
        }
        buf
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Block
    }
}
