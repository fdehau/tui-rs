use std::cmp::min;

use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use widgets::{Widget, Block};
use layout::Rect;
use style::Style;

/// A widget to display several items among which one can be selected (optional)
///
/// # Examples
///
/// ```
/// # extern crate tui;
/// # use tui::widgets::{Block, border, List};
/// # use tui::style::Color;
/// # fn main() {
/// List::default()
///     .block(Block::default().title("List").borders(border::ALL))
///     .items(&["Item 1", "Item 2", "Item 3"])
///     .select(1)
///     .color(Color::White)
///     .highlight_color(Color::Yellow)
///     .highlight_symbol(">>");
/// # }
/// ```
pub struct List<'a> {
    block: Option<Block<'a>>,
    /// Items to be displayed
    items: &'a [&'a str],
    /// Index of the one selected
    selected: Option<usize>,
    /// Base style of the widget
    style: Style,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'a str>,
}

impl<'a> Default for List<'a> {
    fn default() -> List<'a> {
        List {
            block: None,
            items: &[],
            selected: None,
            style: Default::default(),
            highlight_style: Default::default(),
            highlight_symbol: None,
        }
    }
}

impl<'a> List<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut List<'a> {
        self.block = Some(block);
        self
    }

    pub fn items(&'a mut self, items: &'a [&'a str]) -> &mut List<'a> {
        self.items = items;
        self
    }

    pub fn style(&'a mut self, style: Style) -> &mut List<'a> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(&'a mut self, highlight_symbol: &'a str) -> &mut List<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(&'a mut self, highlight_style: Style) -> &mut List<'a> {
        self.highlight_style = highlight_style;
        self
    }

    pub fn select(&'a mut self, index: usize) -> &'a mut List<'a> {
        self.selected = Some(index);
        self
    }
}

impl<'a> Widget for List<'a> {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {

        let list_area = match self.block {
            Some(ref b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        self.background(&list_area, buf, self.style.bg);

        let list_length = self.items.len();
        let list_height = list_area.height as usize;

        // Use highlight_style only if something is selected
        let (selected, highlight_style) = match self.selected {
            Some(i) => (i, &self.highlight_style),
            None => (0, &self.style),
        };

        // Make sure the list show the selected item
        let offset = if selected >= list_height {
            selected - list_height + 1
        } else {
            0
        };

        // Move items to the right if a highlight symbol was provided
        let x = match self.highlight_symbol {
            Some(s) => (s.width() + 1) as u16 + list_area.left(),
            None => list_area.left(),
        };

        // Render items
        if x < list_area.right() {
            let width = (list_area.right() - x) as usize;
            let max_index = min(list_height, list_length);
            for i in 0..max_index {
                let index = i + offset;
                let item = self.items[index];
                let style = if index == selected {
                    highlight_style
                } else {
                    &self.style
                };
                buf.set_stringn(x, list_area.top() + i as u16, item, width, style);
            }

            if let Some(s) = self.highlight_symbol {
                buf.set_string(list_area.left(),
                               list_area.top() + (selected - offset) as u16,
                               s,
                               &self.highlight_style);
            }
        }
    }
}
