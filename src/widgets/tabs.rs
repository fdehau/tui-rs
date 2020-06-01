use unicode_width::UnicodeWidthStr;

use crate::buffer::Buffer;
use crate::layout::{Margin, Rect};
use crate::style::Style;
use crate::symbols::line;
use crate::widgets::{Block, Widget};

/// A widget to display available tabs in a multiple panels context.
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, Tabs};
/// # use tui::style::{Style, Color};
/// # use tui::symbols::{DOT};
/// Tabs::default()
///     .block(Block::default().title("Tabs").borders(Borders::ALL))
///     .titles(&["Tab1", "Tab2", "Tab3", "Tab4"])
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().fg(Color::Yellow))
///     .divider(DOT);
/// ```
#[derive(Debug, Clone)]
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
    /// Tab divider
    divider: &'a str,
    /// Margin width
    margin: Margin,
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
            divider: line::VERTICAL,
            margin: Margin {
                horizontal: 0,
                vertical: 0,
            },
        }
    }
}

impl<'a, T> Tabs<'a, T>
where
    T: AsRef<str>,
{
    pub fn block(mut self, block: Block<'a>) -> Tabs<'a, T> {
        self.block = Some(block);
        self
    }

    pub fn titles(mut self, titles: &'a [T]) -> Tabs<'a, T> {
        self.titles = titles;
        self
    }

    pub fn select(mut self, selected: usize) -> Tabs<'a, T> {
        self.selected = selected;
        self
    }

    pub fn style(mut self, style: Style) -> Tabs<'a, T> {
        self.style = style;
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Tabs<'a, T> {
        self.highlight_style = style;
        self
    }

    pub fn divider(mut self, divider: &'a str) -> Tabs<'a, T> {
        self.divider = divider;
        self
    }

    pub fn margin(mut self, margin: Margin) -> Tabs<'a, T> {
        self.margin = margin;
        self
    }
}

impl<'a, T> Widget for Tabs<'a, T>
where
    T: AsRef<str>,
{
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let tabs_area = match self.block {
            Some(ref mut b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        }
        .inner(&self.margin);

        println!("area: {:?}, tabs_area: {:?}", area, tabs_area);
        println!("tabs_area height: {}", tabs_area.height);

        if tabs_area.height < 1 {
            return;
        }

        println!("didn't return");

        buf.set_background(tabs_area, self.style.bg);

        let mut x = tabs_area.left();
        let titles_length = self.titles.len();

        // divider actually requires a space before it, so we add one
        let divider_width = self.divider.width() as u16 + 1;

        for (title, style, last_title) in self.titles.iter().enumerate().map(|(i, t)| {
            let lt = i + 1 == titles_length;
            if i == self.selected {
                (t, self.highlight_style, lt)
            } else {
                (t, self.style, lt)
            }
        }) {
            if x >= tabs_area.right() {
                break;
            }

            let mut space_remaining: isize = (tabs_area.right() as isize) - (x as isize);
            let title_width = title.as_ref().width() as u16;
            if title_width > space_remaining as u16 {
                buf.set_stringn(
                    x,
                    tabs_area.top(),
                    title.as_ref(),
                    space_remaining as usize,
                    style,
                );
                break;
            } else {
                buf.set_string(x, tabs_area.top(), title.as_ref(), style);
                x += title_width;
                space_remaining -= title_width as isize;

                if !last_title {
                    if space_remaining >= divider_width as isize {
                        buf.set_string(x + 1, tabs_area.top(), self.divider, self.style);
                        x += divider_width + 1; // add an additional space for the next one
                    } else {
                        break;
                    }
                }
            }
        }
    }
}
