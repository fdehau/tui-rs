use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::style::Style;
use crate::symbols::{double, line, rounded};
use crate::widgets::{Borders, Widget};

/// Base widget to be used with all upper level ones. It may be used to display a box border around
/// the widget and/or add a title.
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders};
/// # use tui::style::{Style, Color};
/// # fn main() {
/// Block::default()
///     .title("Block")
///     .title_style(Style::default().fg(Color::Red))
///     .borders(Borders::LEFT | Borders::RIGHT)
///     .border_style(Style::default().fg(Color::White))
///     .style(Style::default().bg(Color::Black));
/// # }
/// ```
#[derive(Clone, Copy)]
pub struct Block<'a> {
    /// Optional title place on the upper left of the block
    title: Option<&'a str>,
    /// Title style
    title_style: Style,
    /// Visible borders
    borders: Borders,
    /// Border style (meaning colors)
    border_style: Style,
    /// Border type (meaning one of single lines w/sharp corners, single lines w/rounded corners,
    /// or double lines w/sharp corners)
    border_type: BorderType,
    /// Widget style
    style: Style,
}

impl<'a> Default for Block<'a> {
    fn default() -> Block<'a> {
        Block {
            title: None,
            title_style: Default::default(),
            borders: Borders::NONE,
            border_style: Default::default(),
            border_type: BorderType::Plain,
            style: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BorderType {
    Plain,
    Rounded,
    Double,
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

    pub fn borders(mut self, flag: Borders) -> Block<'a> {
        self.borders = flag;
        self
    }

    pub fn set_border_type(mut self, border_type: BorderType) -> Block<'a> {
        self.border_type = border_type;
        self
    }

    /// Compute the inner area of a block based on its border visibility rules.
    pub fn inner(&self, area: Rect) -> Rect {
        if area.width < 2 || area.height < 2 {
            return Rect::default();
        }
        let mut inner = area;
        if self.borders.intersects(Borders::LEFT) {
            inner.x += 1;
            inner.width -= 1;
        }
        if self.borders.intersects(Borders::TOP) || self.title.is_some() {
            inner.y += 1;
            inner.height -= 1;
        }
        if self.borders.intersects(Borders::RIGHT) {
            inner.width -= 1;
        }
        if self.borders.intersects(Borders::BOTTOM) {
            inner.height -= 1;
        }
        inner
    }
}

impl<'a> Widget for Block<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        if area.width < 2 || area.height < 2 {
            return;
        }

        self.background(area, buf, self.style.bg);

        // Sides
        if self.borders.intersects(Borders::LEFT) {
            for y in area.top()..area.bottom() {
                buf.get_mut(area.left(), y)
                    .set_symbol({
                        match self.border_type {
                            BorderType::Double => double::VERTICAL,
                            _ => line::VERTICAL,
                        }
                    })
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::TOP) {
            for x in area.left()..area.right() {
                buf.get_mut(x, area.top())
                    .set_symbol({
                        match self.border_type {
                            BorderType::Double => double::HORIZONTAL,
                            _ => line::HORIZONTAL,
                        }
                    })
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol({
                        match self.border_type {
                            BorderType::Double => double::VERTICAL,
                            _ => line::VERTICAL,
                        }
                    })
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol({
                        match self.border_type {
                            BorderType::Double => double::HORIZONTAL,
                            _ => line::HORIZONTAL,
                        }
                    })
                    .set_style(self.border_style);
            }
        }

        // Corners
        if self.borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(area.left(), area.top())
                .set_symbol({
                    match self.border_type {
                        BorderType::Double => double::TOP_LEFT,
                        BorderType::Rounded => rounded::TOP_LEFT,
                        _ => line::TOP_LEFT,
                    }
                })
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(area.right() - 1, area.top())
                .set_symbol({
                    match self.border_type {
                        BorderType::Double => double::TOP_RIGHT,
                        BorderType::Rounded => rounded::TOP_RIGHT,
                        _ => line::TOP_RIGHT,
                    }
                })
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf.get_mut(area.left(), area.bottom() - 1)
                .set_symbol({
                    match self.border_type {
                        BorderType::Double => double::BOTTOM_LEFT,
                        BorderType::Rounded => rounded::BOTTOM_LEFT,
                        _ => line::BOTTOM_LEFT,
                    }
                })
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
                .set_symbol({
                    match self.border_type {
                        BorderType::Double => double::BOTTOM_RIGHT,
                        BorderType::Rounded => rounded::BOTTOM_RIGHT,
                        _ => line::BOTTOM_RIGHT,
                    }
                })
                .set_style(self.border_style);
        }

        if area.width > 2 {
            if let Some(title) = self.title {
                let lx = if self.borders.intersects(Borders::LEFT) {
                    1
                } else {
                    0
                };
                let rx = if self.borders.intersects(Borders::RIGHT) {
                    1
                } else {
                    0
                };
                let width = area.width - lx - rx;
                buf.set_stringn(
                    area.left() + lx,
                    area.top(),
                    title,
                    width as usize,
                    self.title_style,
                );
            }
        }
    }
}
