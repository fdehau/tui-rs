use std::iter;
use std::cmp::min;

use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use widgets::{Widget, Block};
use layout::Rect;
use style::Style;


pub struct List<'a> {
    block: Option<Block<'a>>,
    items: Vec<(&'a str, &'a Style)>,
    style: Style,
}

impl<'a> Default for List<'a> {
    fn default() -> List<'a> {
        List {
            block: None,
            items: Vec::new(),
            style: Default::default(),
        }
    }
}

impl<'a> List<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut List<'a> {
        self.block = Some(block);
        self
    }

    pub fn items<I>(&'a mut self, items: &'a [(I, &'a Style)]) -> &mut List<'a>
        where I: AsRef<str> + 'a
    {
        self.items =
            items.iter().map(|&(ref i, s)| (i.as_ref(), s)).collect::<Vec<(&'a str, &'a Style)>>();
        self
    }

    pub fn style(&'a mut self, style: Style) -> &mut List<'a> {
        self.style = style;
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

        let max_index = min(self.items.len(), list_area.height as usize);
        for (i, &(ref item, style)) in self.items.iter().enumerate().take(max_index) {
            buf.set_stringn(list_area.left(),
                            list_area.top() + i as u16,
                            item.as_ref(),
                            list_area.width as usize,
                            style);
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
            .block(self.block.unwrap_or_default())
            .items(&items)
            .draw(area, buf);
    }
}
