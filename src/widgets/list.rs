use std::fmt::Display;
use std::iter;
use std::iter::Iterator;

use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use layout::{Corner, Rect};
use style::Style;
use widgets::{Block, Widget};

pub enum Item<'i, D: 'i> {
    Data(D),
    StyledData(D, &'i Style),
}

pub struct List<'b, 'i, L, D: 'i>
where
    L: Iterator<Item = Item<'i, D>>,
{
    block: Option<Block<'b>>,
    items: L,
    style: Style,
    start_corner: Corner,
}

impl<'b, 'i, L, D> Default for List<'b, 'i, L, D>
where
    L: Iterator<Item = Item<'i, D>> + Default,
{
    fn default() -> List<'b, 'i, L, D> {
        List {
            block: None,
            items: L::default(),
            style: Default::default(),
            start_corner: Corner::TopLeft,
        }
    }
}

impl<'b, 'i, L, D> List<'b, 'i, L, D>
where
    L: Iterator<Item = Item<'i, D>>,
{
    pub fn new(items: L) -> List<'b, 'i, L, D> {
        List {
            block: None,
            items,
            style: Default::default(),
            start_corner: Corner::TopLeft,
        }
    }

    pub fn block(&'b mut self, block: Block<'b>) -> &mut List<'b, 'i, L, D> {
        self.block = Some(block);
        self
    }

    pub fn items<I>(&'b mut self, items: I) -> &mut List<'b, 'i, L, D>
    where
        I: IntoIterator<Item = Item<'i, D>, IntoIter = L>,
    {
        self.items = items.into_iter();
        self
    }

    pub fn style(&'b mut self, style: Style) -> &mut List<'b, 'i, L, D> {
        self.style = style;
        self
    }

    pub fn start_corner(&'b mut self, corner: Corner) -> &mut List<'b, 'i, L, D> {
        self.start_corner = corner;
        self
    }
}

impl<'b, 'i, L, D> Widget for List<'b, 'i, L, D>
where
    L: Iterator<Item = Item<'i, D>>,
    D: Display,
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

        self.background(&list_area, buf, self.style.bg);

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
                Item::Data(ref v) => {
                    buf.set_stringn(
                        x,
                        y,
                        &format!("{}", v),
                        list_area.width as usize,
                        &Style::default(),
                    );
                }
                Item::StyledData(ref v, s) => {
                    buf.set_stringn(x, y, &format!("{}", v), list_area.width as usize, s);
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
/// # extern crate tui;
/// # use tui::widgets::{Block, Borders, SelectableList};
/// # use tui::style::{Style, Color, Modifier};
/// # fn main() {
/// SelectableList::default()
///     .block(Block::default().title("SelectableList").borders(Borders::ALL))
///     .items(&["Item 1", "Item 2", "Item 3"])
///     .select(1)
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().modifier(Modifier::Italic))
///     .highlight_symbol(">>");
/// # }
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
    pub fn block(&'b mut self, block: Block<'b>) -> &mut SelectableList<'b> {
        self.block = Some(block);
        self
    }

    pub fn items<I>(&'b mut self, items: &'b [I]) -> &mut SelectableList<'b>
    where
        I: AsRef<str> + 'b,
    {
        self.items = items.iter().map(|i| i.as_ref()).collect::<Vec<&str>>();
        self
    }

    pub fn style(&'b mut self, style: Style) -> &mut SelectableList<'b> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(&'b mut self, highlight_symbol: &'b str) -> &mut SelectableList<'b> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(&'b mut self, highlight_style: Style) -> &mut SelectableList<'b> {
        self.highlight_style = highlight_style;
        self
    }

    pub fn select(&'b mut self, index: usize) -> &'b mut SelectableList<'b> {
        self.selected = Some(index);
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
            Some(i) => (i, &self.highlight_style),
            None => (0, &self.style),
        };
        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = iter::repeat(" ")
            .take(highlight_symbol.width())
            .collect::<String>();
        // Make sure the list show the selected item
        let offset = if selected >= list_height {
            selected - list_height + 1
        } else {
            0
        };

        // Render items
        let items = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                if i == selected {
                    Item::StyledData(format!("{} {}", highlight_symbol, item), highlight_style)
                } else {
                    Item::StyledData(format!("{} {}", blank_symbol, item), &self.style)
                }
            })
            .skip(offset as usize);
        List::new(items)
            .block(self.block.unwrap_or_default())
            .style(self.style)
            .draw(area, buf);
    }
}
