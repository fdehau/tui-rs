use std::iter;
use std::fmt::Display;
use std::iter::Iterator;

use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use widgets::{Widget, Block};
use layout::Rect;
use style::Style;

pub enum Item<'a, D: 'a> {
    Data(D),
    StyledData(D, &'a Style),
}

pub struct List<'a, L, D: 'a>
    where L: Iterator<Item = Item<'a, D>>
{
    block: Option<Block<'a>>,
    items: L,
    style: Style,
}

impl<'a, L, D> Default for List<'a, L, D>
    where L: Iterator<Item = Item<'a, D>> + Default
{
    fn default() -> List<'a, L, D> {
        List {
            block: None,
            items: L::default(),
            style: Default::default(),
        }
    }
}

impl<'a, L, D> List<'a, L, D>
    where L: Iterator<Item = Item<'a, D>>
{
    pub fn new(items: L) -> List<'a, L, D> {
        List {
            block: None,
            items: items,
            style: Default::default(),
        }
    }

    pub fn block(&'a mut self, block: Block<'a>) -> &mut List<'a, L, D> {
        self.block = Some(block);
        self
    }

    pub fn items<I: IntoIterator<Item = Item<'a, D>>>(&'a mut self,
                                                      items: L)
                                                      -> &mut List<'a, L, D> {
        self.items = items;
        self
    }

    pub fn style(&'a mut self, style: Style) -> &mut List<'a, L, D> {
        self.style = style;
        self
    }
}

impl<'a, L, D> Widget for List<'a, L, D>
    where L: Iterator<Item = Item<'a, D>>,
          D: Display
{
    fn draw(&mut self, area: &Rect, buf: &mut Buffer) {
        let list_area = match self.block {
            Some(ref mut b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        self.background(&list_area, buf, self.style.bg);

        for (i, item) in self.items
                .by_ref()
                .enumerate()
                .take(list_area.height as usize) {
            match item {
                Item::Data(ref v) => {
                    buf.set_stringn(list_area.left(),
                                    list_area.top() + i as u16,
                                    &format!("{}", v),
                                    list_area.width as usize,
                                    &Style::default());
                }
                Item::StyledData(ref v, ref s) => {
                    buf.set_stringn(list_area.left(),
                                    list_area.top() + i as u16,
                                    &format!("{}", v),
                                    list_area.width as usize,
                                    s);
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
/// # use tui::widgets::{Block, border, SelectableList};
/// # use tui::style::{Style, Color, Modifier};
/// # fn main() {
/// SelectableList::default()
///     .block(Block::default().title("SelectableList").borders(border::ALL))
///     .items(&["Item 1", "Item 2", "Item 3"])
///     .select(1)
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().modifier(Modifier::Italic))
///     .highlight_symbol(">>");
/// # }
/// ```
pub struct SelectableList<'a> {
    block: Option<Block<'a>>,
    /// Items to be displayed
    items: Vec<&'a str>,
    /// Index of the one selected
    selected: Option<usize>,
    /// Base style of the widget
    style: Style,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'a str>,
}

impl<'a> Default for SelectableList<'a> {
    fn default() -> SelectableList<'a> {
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

impl<'a> SelectableList<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut SelectableList<'a> {
        self.block = Some(block);
        self
    }

    pub fn items<I>(&'a mut self, items: &'a [I]) -> &mut SelectableList<'a>
        where I: AsRef<str> + 'a
    {
        self.items = items.iter().map(|i| i.as_ref()).collect::<Vec<&str>>();
        self
    }

    pub fn style(&'a mut self, style: Style) -> &mut SelectableList<'a> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(&'a mut self, highlight_symbol: &'a str) -> &mut SelectableList<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(&'a mut self, highlight_style: Style) -> &mut SelectableList<'a> {
        self.highlight_style = highlight_style;
        self
    }

    pub fn select(&'a mut self, index: usize) -> &'a mut SelectableList<'a> {
        self.selected = Some(index);
        self
    }
}

impl<'a> Widget for SelectableList<'a> {
    fn draw(&mut self, area: &Rect, buf: &mut Buffer) {

        let list_area = match self.block {
            Some(ref mut b) => b.inner(area),
            None => *area,
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
        List::new(self.items
                      .iter()
                      .enumerate()
                      .map(|(i, item)| if i == selected {
                               Item::StyledData(format!("{} {}", highlight_symbol, item),
                                                highlight_style)
                           } else {
                               Item::StyledData(format!("{} {}", blank_symbol, item), &self.style)
                           })
                      .skip(offset as usize))
                .block(self.block.unwrap_or_default())
                .draw(area, buf);
    }
}
