use unicode_width::UnicodeWidthStr;

use widgets::{Block, Widget};
use buffer::Buffer;
use layout::Rect;
use style::Color;
use symbols::line;

/// A widget to display available tabs in a multiple panels context.
///
/// # Examples
///
/// ```
/// # extern crate tui;
/// # use tui::widgets::{Block, border, Tabs};
/// # use tui::style::Color;
/// # fn main() {
/// Tabs::default()
///     .block(Block::default().title("Tabs").borders(border::ALL))
///     .titles(&["Tab1", "Tab2", "Tab3", "Tab4"])
///     .color(Color::White)
///     .highlight_color(Color::Yellow)
///     .background_color(Color::Black);
/// # }
/// ```
pub struct Tabs<'a> {
    /// A block to wrap this widget in if necessary
    block: Option<Block<'a>>,
    /// One title for each tab
    titles: &'a [&'a str],
    /// The index of the selected tabs
    selected: usize,
    /// The color used to draw the text
    color: Color,
    /// The background color of this widget
    background_color: Color,
    /// The color used to display the selected item
    highlight_color: Color,
}

impl<'a> Default for Tabs<'a> {
    fn default() -> Tabs<'a> {
        Tabs {
            block: None,
            titles: &[],
            selected: 0,
            color: Color::Reset,
            background_color: Color::Reset,
            highlight_color: Color::Reset,
        }
    }
}

impl<'a> Tabs<'a> {
    pub fn block(&mut self, block: Block<'a>) -> &mut Tabs<'a> {
        self.block = Some(block);
        self
    }

    pub fn titles(&mut self, titles: &'a [&'a str]) -> &mut Tabs<'a> {
        self.titles = titles;
        self
    }

    pub fn select(&mut self, selected: usize) -> &mut Tabs<'a> {
        self.selected = selected;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Tabs<'a> {
        self.color = color;
        self
    }

    pub fn background_color(&mut self, color: Color) -> &mut Tabs<'a> {
        self.background_color = color;
        self
    }

    pub fn highlight_color(&mut self, color: Color) -> &mut Tabs<'a> {
        self.highlight_color = color;
        self
    }
}

impl<'a> Widget for Tabs<'a> {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {

        let tabs_area = match self.block {
            Some(b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        if tabs_area.height < 1 {
            return;
        }

        if self.background_color != Color::Reset {
            self.background(&tabs_area, buf, self.background_color);
        }

        let mut x = tabs_area.left();
        for (title, color) in self.titles.iter().enumerate().map(|(i, t)| if i == self.selected {
            (t, self.highlight_color)
        } else {
            (t, self.color)
        }) {
            x += 1;
            if x > tabs_area.right() {
                break;
            } else {
                buf.set_string(x, tabs_area.top(), title, color, self.background_color);
                x += title.width() as u16 + 1;
                if x >= tabs_area.right() {
                    break;
                } else {
                    buf.set_cell(x,
                                 tabs_area.top(),
                                 line::VERTICAL,
                                 self.color,
                                 self.background_color);
                    x += 1;
                }
            }
        }
    }
}
