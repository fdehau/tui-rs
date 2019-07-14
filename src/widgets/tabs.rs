use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::line,
    widgets::{Block, Widget},
};

/// A widget to display available tabs in a multiple panels context.
///
/// # Examples
///
/// ```
/// # use itui::widgets::{Block, Borders, Tabs};
/// # use itui::style::{Style, Color};
/// # use itui::symbols::{DOT};
/// # fn main() {
/// Tabs::default()
///     .block(Block::default().title("Tabs").borders(Borders::ALL))
///     .titles(&["Tab1", "Tab2", "Tab3", "Tab4"])
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().fg(Color::Yellow))
///     .divider(DOT);
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
    /// Tab divider
    divider: &'a str,
    /// area occupied by this tabs
    area: Rect,
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
            area: Default::default(),
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

    pub fn area(mut self, area: Rect) -> Self {
        self.area = area;
        self
    }
}

impl<'a, T> Widget for Tabs<'a, T>
where
    T: AsRef<str>,
{
    fn get_area(&self) -> Rect {
        self.area
    }
    fn draw(&mut self, buf: &mut Buffer) {
        let tabs_area = match self.block {
            Some(ref mut b) => {
                b.draw(buf);
                b.inner()
            }
            None => self.area,
        };

        if tabs_area.height < 1 {
            return;
        }

        self.background(buf, self.style.bg);

        let mut x = tabs_area.left();
        let titles_length = self.titles.len();
        let divider_width = self.divider.width() as u16;
        for (title, style, last_title) in self.titles.iter().enumerate().map(|(i, t)| {
            let lt = i + 1 == titles_length;
            if i == self.selected {
                (t, self.highlight_style, lt)
            } else {
                (t, self.style, lt)
            }
        }) {
            x += 1;
            if x > tabs_area.right() {
                break;
            } else {
                buf.set_string(x, tabs_area.top(), title.as_ref(), style);
                x += title.as_ref().width() as u16 + 1;
                if x >= tabs_area.right() || last_title {
                    break;
                } else {
                    buf.set_string(x, tabs_area.top(), self.divider, self.style);
                    x += divider_width;
                }
            }
        }
    }
}
