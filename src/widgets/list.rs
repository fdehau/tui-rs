use std::convert::AsRef;
use std::iter::{self, Iterator};

use unicode_width::UnicodeWidthStr;

use crate::buffer::Buffer;
use crate::layout::{Corner, Rect};
use crate::style::Style;
use crate::widgets::{Block, Text, Widget};

pub struct List<'b, L>
where
    L: Iterator<Item = Text<'b>>,
{
    block: Option<Block<'b>>,
    items: L,
    style: Style,
    start_corner: Corner,
}

impl<'b, L> Default for List<'b, L>
where
    L: Iterator<Item = Text<'b>> + Default,
{
    fn default() -> List<'b, L> {
        List {
            block: None,
            items: L::default(),
            style: Default::default(),
            start_corner: Corner::TopLeft,
        }
    }
}

impl<'b, L> List<'b, L>
where
    L: Iterator<Item = Text<'b>>,
{
    pub fn new(items: L) -> List<'b, L> {
        List {
            block: None,
            items,
            style: Default::default(),
            start_corner: Corner::TopLeft,
        }
    }

    pub fn block(mut self, block: Block<'b>) -> List<'b, L> {
        self.block = Some(block);
        self
    }

    pub fn items<I>(mut self, items: I) -> List<'b, L>
    where
        I: IntoIterator<Item = Text<'b>, IntoIter = L>,
    {
        self.items = items.into_iter();
        self
    }

    pub fn style(mut self, style: Style) -> List<'b, L> {
        self.style = style;
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> List<'b, L> {
        self.start_corner = corner;
        self
    }
}

impl<'b, L> Widget for List<'b, L>
where
    L: Iterator<Item = Text<'b>>,
{
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

        for (i, item) in self
            .items
            .by_ref()
            .enumerate()
            .take(list_area.height as usize)
        {
            let (x, y) = match self.start_corner {
                Corner::TopLeft => (list_area.left(), list_area.top() + i as u16),
                Corner::BottomLeft => (list_area.left(), list_area.bottom() - (i + 1) as u16),
                // Not supported
                _ => (list_area.left(), list_area.top() + i as u16),
            };
            match item {
                Text::Raw(ref v) => {
                    buf.set_stringn(x, y, v, list_area.width as usize, Style::default());
                }
                Text::Styled(ref v, s) => {
                    buf.set_stringn(x, y, v, list_area.width as usize, s);
                }
            };
        }
    }
}

/// A widget to display several items among which one can be selected (optional)
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, SelectableList};
/// # use tui::style::{Style, Color, Modifier};
/// SelectableList::default()
///     .block(Block::default().title("SelectableList").borders(Borders::ALL))
///     .items(&["Item 1", "Item 2", "Item 3"])
///     .select(Some(1))
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().modifier(Modifier::ITALIC))
///     .highlight_symbol(">>");
/// ```
pub struct SelectableList<'b> {
    block: Option<Block<'b>>,
    /// Items to be displayed
    items: Vec<&'b str>,
    /// Index of the one selected
    selected: Option<usize>,
    /// Base style of the widget
    style: Style,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'b str>,
}

impl<'b> Default for SelectableList<'b> {
    fn default() -> SelectableList<'b> {
        SelectableList {
            block: None,
            items: Vec::new(),
            selected: None,
            style: Default::default(),
            highlight_style: Default::default(),
            highlight_symbol: None,
        }
    }
}

impl<'b> SelectableList<'b> {
    pub fn block(mut self, block: Block<'b>) -> SelectableList<'b> {
        self.block = Some(block);
        self
    }

    pub fn items<I>(mut self, items: &'b [I]) -> SelectableList<'b>
    where
        I: AsRef<str> + 'b,
    {
        self.items = items.iter().map(AsRef::as_ref).collect::<Vec<&str>>();
        self
    }

    pub fn style(mut self, style: Style) -> SelectableList<'b> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'b str) -> SelectableList<'b> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, highlight_style: Style) -> SelectableList<'b> {
        self.highlight_style = highlight_style;
        self
    }

    pub fn select(mut self, index: Option<usize>) -> SelectableList<'b> {
        self.selected = index;
        self
    }
}

impl<'b> Widget for SelectableList<'b> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let list_area = match self.block {
            Some(ref mut b) => b.inner(area),
            None => area,
        };

        let list_height = list_area.height as usize;

        // Use highlight_style only if something is selected
        let (selected, highlight_style) = match self.selected {
            Some(i) => (Some(i), self.highlight_style),
            None => (None, self.style),
        };
        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = iter::repeat(" ")
            .take(highlight_symbol.width())
            .collect::<String>();
        // Make sure the list show the selected item
        let offset = if let Some(selected) = selected {
            if selected >= list_height {
                selected - list_height + 1
            } else {
                0
            }
        } else {
            0
        };

        // Render items
        let items = self
            .items
            .iter()
            .enumerate()
            .map(|(i, &item)| {
                if let Some(s) = selected {
                    if i == s {
                        Text::styled(format!("{} {}", highlight_symbol, item), highlight_style)
                    } else {
                        Text::styled(format!("{} {}", blank_symbol, item), self.style)
                    }
                } else {
                    Text::styled(item, self.style)
                }
            })
            .skip(offset as usize);
        List::new(items)
            .block(self.block.unwrap_or_default())
            .style(self.style)
            .draw(area, buf);
    }
}
