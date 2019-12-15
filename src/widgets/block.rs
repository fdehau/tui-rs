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
            borders: Borders::NONE,
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

    pub fn borders(mut self, flag: Borders) -> Block<'a> {
        self.borders = flag;
        self
    }

    pub fn rounded(self) -> RoundedBlock<'a> {
        RoundedBlock::from(self)
    }

    pub fn double_border(self) -> DoubleBlock<'a> {
        DoubleBlock::from(self)
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
                    .set_symbol(line::VERTICAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::TOP) {
            for x in area.left()..area.right() {
                buf.get_mut(x, area.top())
                    .set_symbol(line::HORIZONTAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(line::VERTICAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(line::HORIZONTAL)
                    .set_style(self.border_style);
            }
        }

        // Corners
        if self.borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(area.left(), area.top())
                .set_symbol(line::TOP_LEFT)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(area.right() - 1, area.top())
                .set_symbol(line::TOP_RIGHT)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf.get_mut(area.left(), area.bottom() - 1)
                .set_symbol(line::BOTTOM_LEFT)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
                .set_symbol(line::BOTTOM_RIGHT)
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

#[derive(Clone, Copy)]
pub struct RoundedBlock<'a> {
    /// Optional title place on the upper left of the block
    title: Option<&'a str>,
    /// Title style
    title_style: Style,
    /// Visible borders
    borders: Borders,
    /// Border style
    border_style: Style,
    /// Widget style
    style: Style,
}

impl<'a> From<Block<'a>> for RoundedBlock<'a> {
    fn from(block: Block<'a>) -> RoundedBlock<'a> {
        RoundedBlock {
            title: block.title,
            title_style: block.title_style,
            borders: block.borders,
            border_style: block.border_style,
            style: block.style,
        }
    }
}

impl<'a> Default for RoundedBlock<'a> {
    fn default() -> RoundedBlock<'a> {
        RoundedBlock {
            title: None,
            title_style: Default::default(),
            borders: Borders::NONE,
            border_style: Default::default(),
            style: Default::default(),
        }
    }
}

impl<'a> RoundedBlock<'a> {
    pub fn title(mut self, title: &'a str) -> RoundedBlock<'a> {
        self.title = Some(title);
        self
    }

    pub fn title_style(mut self, style: Style) -> RoundedBlock<'a> {
        self.title_style = style;
        self
    }

    pub fn border_style(mut self, style: Style) -> RoundedBlock<'a> {
        self.border_style = style;
        self
    }

    pub fn style(mut self, style: Style) -> RoundedBlock<'a> {
        self.style = style;
        self
    }

    pub fn borders(mut self, flag: Borders) -> RoundedBlock<'a> {
        self.borders = flag;
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

impl<'a> Widget for RoundedBlock<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        if area.width < 2 || area.height < 2 {
            return;
        }

        self.background(area, buf, self.style.bg);

        // Sides
        if self.borders.intersects(Borders::LEFT) {
            for y in area.top()..area.bottom() {
                buf.get_mut(area.left(), y)
                    .set_symbol(line::VERTICAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::TOP) {
            for x in area.left()..area.right() {
                buf.get_mut(x, area.top())
                    .set_symbol(line::HORIZONTAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(line::VERTICAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(line::HORIZONTAL)
                    .set_style(self.border_style);
            }
        }

        // Corners
        if self.borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(area.left(), area.top())
                .set_symbol(rounded::TOP_LEFT)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(area.right() - 1, area.top())
                .set_symbol(rounded::TOP_RIGHT)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf.get_mut(area.left(), area.bottom() - 1)
                .set_symbol(rounded::BOTTOM_LEFT)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
                .set_symbol(rounded::BOTTOM_RIGHT)
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

#[derive(Clone, Copy)]
pub struct DoubleBlock<'a> {
    /// Optional title place on the upper left of the block
    title: Option<&'a str>,
    /// Title style
    title_style: Style,
    /// Visible borders
    borders: Borders,
    /// Border style
    border_style: Style,
    /// Widget style
    style: Style,
}

impl<'a> From<Block<'a>> for DoubleBlock<'a> {
    fn from(block: Block<'a>) -> DoubleBlock<'a> {
        DoubleBlock {
            title: block.title,
            title_style: block.title_style,
            borders: block.borders,
            border_style: block.border_style,
            style: block.style,
        }
    }
}
impl<'a> Default for DoubleBlock<'a> {
    fn default() -> DoubleBlock<'a> {
        DoubleBlock {
            title: None,
            title_style: Default::default(),
            borders: Borders::NONE,
            border_style: Default::default(),
            style: Default::default(),
        }
    }
}

impl<'a> DoubleBlock<'a> {
    pub fn title(mut self, title: &'a str) -> DoubleBlock<'a> {
        self.title = Some(title);
        self
    }

    pub fn title_style(mut self, style: Style) -> DoubleBlock<'a> {
        self.title_style = style;
        self
    }

    pub fn border_style(mut self, style: Style) -> DoubleBlock<'a> {
        self.border_style = style;
        self
    }

    pub fn style(mut self, style: Style) -> DoubleBlock<'a> {
        self.style = style;
        self
    }

    pub fn borders(mut self, flag: Borders) -> DoubleBlock<'a> {
        self.borders = flag;
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

impl<'a> Widget for DoubleBlock<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        if area.width < 2 || area.height < 2 {
            return;
        }

        self.background(area, buf, self.style.bg);

        // Sides
        if self.borders.intersects(Borders::LEFT) {
            for y in area.top()..area.bottom() {
                buf.get_mut(area.left(), y)
                    .set_symbol(double::VERTICAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::TOP) {
            for x in area.left()..area.right() {
                buf.get_mut(x, area.top())
                    .set_symbol(double::HORIZONTAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(double::VERTICAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(double::HORIZONTAL)
                    .set_style(self.border_style);
            }
        }

        // Corners
        if self.borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(area.left(), area.top())
                .set_symbol(double::TOP_LEFT)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(area.right() - 1, area.top())
                .set_symbol(double::TOP_RIGHT)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf.get_mut(area.left(), area.bottom() - 1)
                .set_symbol(double::BOTTOM_LEFT)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
                .set_symbol(double::BOTTOM_RIGHT)
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
