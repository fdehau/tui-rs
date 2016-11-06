use buffer::Buffer;
use layout::Rect;
use style::Style;
use widgets::{Widget, border};
use symbols::line;

/// Base widget to be used with all upper level ones. It may be used to display a box border around
/// the widget and/or add a title.
///
/// # Examples
///
/// ```
/// # extern crate tui;
/// # use tui::widgets::{Block, border};
/// # use tui::style::Color;
/// # fn main() {
/// Block::default()
///     .title("Block")
///     .title_color(Color::Red)
///     .borders(border::LEFT | border::RIGHT)
///     .border_color(Color::White)
///     .background_color(Color::Black);
/// # }
/// ```
#[derive(Clone, Copy)]
pub struct Block<'a> {
    /// Optional title place on the upper left of the block
    title: Option<&'a str>,
    /// Title style
    title_style: Style,
    /// Visible borders
    borders: border::Flags,
    /// Border style
    border_style: Style,
    /// Widget style
    style: Style,
}

impl<'a> Default for Block<'a> {
    fn default() -> Block<'a> {
        Block {
            title: None,
            title_style: Default::default(),
            borders: border::NONE,
            border_style: Default::default(),
            style: Default::default(),
        }
    }
}

impl<'a> Block<'a> {
    pub fn title(mut self, title: &'a str) -> Block<'a> {
        self.title = Some(title);
        self
    }

    pub fn title_style(mut self, style: Style) -> Block<'a> {
        self.title_style = style;
        self
    }

    pub fn border_style(mut self, style: Style) -> Block<'a> {
        self.border_style = style;
        self
    }

    pub fn style(mut self, style: Style) -> Block<'a> {
        self.style = style;
        self
    }

    pub fn borders(mut self, flag: border::Flags) -> Block<'a> {
        self.borders = flag;
        self
    }

    /// Compute the inner area of a block based on its border visibility rules.
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
    fn draw(&self, area: &Rect, buf: &mut Buffer) {

        if area.width < 2 || area.height < 2 {
            return;
        }

        self.background(area, buf, self.style.bg);

        // Sides
        if self.borders.intersects(border::LEFT) {
            for y in area.top()..area.bottom() {
                buf.get_mut(area.left(), y)
                    .set_symbol(line::VERTICAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(border::TOP) {
            for x in area.left()..area.right() {
                buf.get_mut(x, area.top())
                    .set_symbol(line::HORIZONTAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(border::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(line::VERTICAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(border::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(line::HORIZONTAL)
                    .set_style(self.border_style);
            }
        }

        // Corners
        if self.borders.contains(border::LEFT | border::TOP) {
            buf.get_mut(area.left(), area.top())
                .set_symbol(line::TOP_LEFT)
                .set_style(self.border_style);
        }
        if self.borders.contains(border::RIGHT | border::TOP) {
            buf.get_mut(area.right() - 1, area.top())
                .set_symbol(line::TOP_RIGHT)
                .set_style(self.border_style);
        }
        if self.borders.contains(border::LEFT | border::BOTTOM) {
            buf.get_mut(area.left(), area.bottom() - 1)
                .set_symbol(line::BOTTOM_LEFT)
                .set_style(self.border_style);
        }
        if self.borders.contains(border::RIGHT | border::BOTTOM) {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
                .set_symbol(line::BOTTOM_RIGHT)
                .set_style(self.border_style);
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
                                &self.title_style);
            }
        }
    }
}
