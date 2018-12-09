use unicode_width::UnicodeWidthStr;

use crate::buffer::Buffer;
use crate::layout::{Corner, Rect};
use crate::style::Style;
use crate::widgets::{Block, Text, Widget};

pub struct List<'a> {
    block: Option<Block<'a>>,
    items: Vec<Text<'a>>,
    style: Style,
    start_corner: Corner,
    /// Index of the one selected
    selected: Option<usize>,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'a str>,
}

impl<'a> List<'a> {
    pub fn new(items: Vec<Text<'a>>) -> List<'a> {
        List {
            block: None,
            items: items.into(),
            style: Default::default(),
            start_corner: Corner::TopLeft,
            selected: None,
            highlight_style: Default::default(),
            highlight_symbol: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> List<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> List<'a> {
        self.style = style;
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> List<'a> {
        self.start_corner = corner;
        self
    }

    pub fn select(mut self, index: Option<usize>) -> List<'a> {
        self.selected = index;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> List<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, highlight_style: Style) -> List<'a> {
        self.highlight_style = highlight_style;
        self
    }
}

impl<'a> Widget for List<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let list_area = match self.block {
            Some(ref mut b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        self.background(list_area, buf, self.style.bg);

        // Use highlight_style only if something is selected
        let (selected, highlight_style) = match self.selected {
            Some(i) => (Some(i), self.highlight_style),
            None => (None, self.style),
        };
        let (x_offset, highlight_symbol) = if selected.is_some() {
            if let Some(symbol) = self.highlight_symbol {
                (symbol.width() as u16, symbol)
            } else {
                (0, "")
            }
        } else {
            (0, "")
        };

        // Make sure the list show the selected item
        let offset = if let Some(selected) = selected {
            // In order to show the selected item, the content will be shifted
            // from a certain height. This height is the total height of all
            // items that do not fit in the current list height, including
            // the selected items
            let mut height = 0;
            let mut offset_height = 0;
            for (i, item) in self.items.iter().enumerate() {
                let item_height = item.height();
                if height + item_height > list_area.height {
                    offset_height = item_height + height - list_area.height;
                }
                height += item_height;
                if i == selected {
                    break;
                }
            }

            // Go through as much items as needed until their total height is
            // greater than the previous found offset_height
            let mut offset = 0;
            height = 0;
            for item in &self.items {
                if height >= offset_height {
                    break;
                }
                let item_height = item.height();
                height += item_height;
                offset += 1;
            }
            offset
        } else {
            0
        };

        let mut height = 0;
        for (i, item) in self.items.iter().enumerate().skip(offset) {
            let item_height = item.height();
            if height + item_height > list_area.height {
                break;
            }
            let (x, y) = match self.start_corner {
                Corner::TopLeft => (list_area.left(), list_area.top() + height),
                Corner::BottomLeft => (
                    list_area.left(),
                    list_area.bottom() - (height + item_height),
                ),
                // Not supported
                _ => (list_area.left(), list_area.top() + height as u16),
            };
            height += item_height as u16;
            let mut style = self.style;
            if let Some(selected) = self.selected {
                if i == selected {
                    buf.set_stringn(x, y, highlight_symbol, x_offset as usize, highlight_style);
                    style = highlight_style;
                }
            }
            let mut lx = x + x_offset;
            let mut ly = y;
            for (g, style) in item.styled_graphemes(style) {
                if g == "\n" {
                    ly += 1;
                    lx = x + x_offset;
                    continue;
                }
                if lx >= list_area.right() {
                    continue;
                }
                buf.get_mut(lx, ly).set_symbol(g).set_style(style);
                lx += g.width() as u16;
            }
        }
    }
}
