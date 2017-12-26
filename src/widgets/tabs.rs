use unicode_width::UnicodeWidthStr;

use widgets::{Block, Widget};
use buffer::Buffer;
use layout::Rect;
use style::Style;
use symbols::line;

/// A widget to display available tabs in a multiple panels context.
///
/// # Examples
///
/// ```
/// # extern crate tui;
/// # use tui::widgets::{Block, border, Tabs};
/// # use tui::style::{Style, Color};
/// # fn main() {
/// Tabs::default()
///     .block(Block::default().title("Tabs").borders(border::ALL))
///     .titles(&["Tab1", "Tab2", "Tab3", "Tab4"])
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().fg(Color::Yellow));
/// # }
/// ```
pub struct Tabs<'a, T>
where
    T: AsRef<str> + 'a,
{
    /// A block to wrap this widget in if necessary
    block: Option<Block<'a>>,
    /// One title for each tab
    titles: &'a [T],
    /// The index of the selected tabs
    selected: usize,
    /// The style used to draw the text
    style: Style,
    /// The style used to display the selected item
    highlight_style: Style,
}

impl<'a, T> Default for Tabs<'a, T>
where
    T: AsRef<str>,
{
    fn default() -> Tabs<'a, T> {
        Tabs {
            block: None,
            titles: &[],
            selected: 0,
            style: Default::default(),
            highlight_style: Default::default(),
        }
    }
}

impl<'a, T> Tabs<'a, T>
where
    T: AsRef<str>,
{
    pub fn block(&mut self, block: Block<'a>) -> &mut Tabs<'a, T> {
        self.block = Some(block);
        self
    }

    pub fn titles(&mut self, titles: &'a [T]) -> &mut Tabs<'a, T> {
        self.titles = titles;
        self
    }

    pub fn select(&mut self, selected: usize) -> &mut Tabs<'a, T> {
        self.selected = selected;
        self
    }

    pub fn style(&mut self, style: Style) -> &mut Tabs<'a, T> {
        self.style = style;
        self
    }

    pub fn highlight_style(&mut self, style: Style) -> &mut Tabs<'a, T> {
        self.highlight_style = style;
        self
    }
}

impl<'a, T> Widget for Tabs<'a, T>
where
    T: AsRef<str>,
{
    fn draw(&mut self, area: &Rect, buf: &mut Buffer) {
        let tabs_area = match self.block {
            Some(ref mut b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        if tabs_area.height < 1 {
            return;
        }

        self.background(&tabs_area, buf, self.style.bg);

        let mut x = tabs_area.left();
        for (title, style) in self.titles.iter().enumerate().map(|(i, t)| {
            if i == self.selected {
                (t, &self.highlight_style)
            } else {
                (t, &self.style)
            }
        }) {
            x += 1;
            if x > tabs_area.right() {
                break;
            } else {
                buf.set_string(x, tabs_area.top(), title.as_ref(), style);
                x += title.as_ref().width() as u16 + 1;
                if x >= tabs_area.right() {
                    break;
                } else {
                    buf.get_mut(x, tabs_area.top())
                        .set_symbol(line::VERTICAL)
                        .set_fg(self.style.fg)
                        .set_bg(self.style.bg);
                    x += 1;
                }
            }
        }
    }
}
