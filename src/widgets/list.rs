use std::iter;
use std::cmp::min;

use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use widgets::{Widget, Block};
use layout::Rect;
use style::Style;


pub struct List<'a, T>
    where T: AsRef<str> + 'a
{
    block: Option<Block<'a>>,
    items: &'a [(T, &'a Style)],
    style: Style,
}

impl<'a, T> Default for List<'a, T>
    where T: AsRef<str> + 'a
{
    fn default() -> List<'a, T> {
        List {
            block: None,
            items: &[],
            style: Default::default(),
        }
    }
}

impl<'a, T> List<'a, T>
    where T: AsRef<str> + 'a
{
    pub fn block(&'a mut self, block: Block<'a>) -> &mut List<'a, T> {
        self.block = Some(block);
        self
    }

    pub fn items(&'a mut self, items: &'a [(T, &'a Style)]) -> &mut List<'a, T> {
        self.items = items;
        self
    }

    pub fn style(&'a mut self, style: Style) -> &mut List<'a, T> {
        self.style = style;
        self
    }
}

impl<'a, T> Widget for List<'a, T>
    where T: AsRef<str> + 'a
{
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

        let max_index = min(self.items.len(), list_area.height as usize);
        for (i, &(ref item, style)) in self.items.iter().enumerate().take(max_index) {
            buf.set_stringn(list_area.left(),
                            list_area.top() + i as u16,
                            item.as_ref(),
                            list_area.width as usize,
                            &style);
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

impl<'a> Default for SelectableList<'a> {
    fn default() -> SelectableList<'a> {
        SelectableList {
            block: None,
            items: &[],
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

    pub fn items(&'a mut self, items: &'a [&'a str]) -> &mut SelectableList<'a> {
        self.items = items;
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
    fn draw(&self, area: &Rect, buf: &mut Buffer) {

        let list_area = match self.block {
            Some(ref b) => b.inner(area),
            None => *area,
        };

        let list_height = list_area.height as usize;

        // Use highlight_style only if something is selected
        let (selected, highlight_style) = match self.selected {
            Some(i) => (i, &self.highlight_style),
            None => (0, &self.style),
        };
        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = iter::repeat(" ").take(highlight_symbol.width()).collect::<String>();
        // Make sure the list show the selected item
        let offset = if selected >= list_height {
            selected - list_height + 1
        } else {
            0
        };
        let items = self.items
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, item)| if i == selected {
                (format!("{} {}", highlight_symbol, item), highlight_style)
            } else {
                (format!("{} {}", blank_symbol, item), &self.style)
            })
            .skip(offset as usize)
            .collect::<Vec<(String, &Style)>>();

        // Render items
        List::default()
            .block(self.block.unwrap_or(Default::default()))
            .items(&items)
            .draw(area, buf);
    }
}
