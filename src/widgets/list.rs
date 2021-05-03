use crate::{
    layout::{Corner, Rect},
    style::Style,
    text::Text,
    widgets::{Block, RenderContext, Widget},
};
use std::iter::{self, Iterator};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub struct ListState {
    offset: usize,
}

impl Default for ListState {
    fn default() -> ListState {
        ListState { offset: 0 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem<'a> {
    content: Text<'a>,
    style: Style,
}

impl<'a> ListItem<'a> {
    pub fn new<T>(content: T) -> ListItem<'a>
    where
        T: Into<Text<'a>>,
    {
        ListItem {
            content: content.into(),
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> ListItem<'a> {
        self.style = style;
        self
    }

    pub fn height(&self) -> usize {
        self.content.height()
    }
}

/// A widget to display several items among which one can be selected (optional)
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, List, ListItem};
/// # use tui::style::{Style, Color, Modifier};
/// let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
/// List::new(items)
///     .block(Block::default().title("List").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
///     .highlight_symbol(">>");
/// ```
#[derive(Debug, Clone)]
pub struct List<'a> {
    block: Option<Block<'a>>,
    items: Vec<ListItem<'a>>,
    /// Style used as a base style for the widget
    style: Style,
    start_corner: Corner,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'a str>,
    selected: Option<usize>,
}

impl<'a> List<'a> {
    pub fn new<T>(items: T) -> List<'a>
    where
        T: Into<Vec<ListItem<'a>>>,
    {
        List {
            block: None,
            style: Style::default(),
            items: items.into(),
            start_corner: Corner::TopLeft,
            highlight_style: Style::default(),
            highlight_symbol: None,
            selected: None,
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

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> List<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> List<'a> {
        self.highlight_style = style;
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
}

impl<'a> Widget for List<'a> {
    type State = ListState;

    fn render(mut self, ctx: &mut RenderContext<Self::State>) {
        ctx.buffer.set_style(ctx.area, self.style);
        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(ctx.area);
                b.render(&mut RenderContext {
                    area: ctx.area,
                    buffer: ctx.buffer,
                    state: &mut (),
                });
                inner_area
            }
            None => ctx.area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        if self.items.is_empty() {
            return;
        }
        let list_height = list_area.height as usize;

        if self.selected.is_none() {
            ctx.state.offset = 0;
        }

        let mut start = ctx.state.offset;
        let mut end = ctx.state.offset;
        let mut height = 0;
        for item in self.items.iter().skip(ctx.state.offset) {
            if height + item.height() > list_height {
                break;
            }
            height += item.height();
            end += 1;
        }

        let selected = self.selected.unwrap_or(0).min(self.items.len() - 1);
        while selected >= end {
            height = height.saturating_add(self.items[end].height());
            end += 1;
            while height > list_height {
                height = height.saturating_sub(self.items[start].height());
                start += 1;
            }
        }
        while selected < start {
            start -= 1;
            height = height.saturating_add(self.items[start].height());
            while height > list_height {
                end -= 1;
                height = height.saturating_sub(self.items[end].height());
            }
        }
        ctx.state.offset = start;

        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = iter::repeat(" ")
            .take(highlight_symbol.width())
            .collect::<String>();

        let mut current_height = 0;
        let has_selection = self.selected.is_some();
        for (i, item) in self
            .items
            .iter_mut()
            .enumerate()
            .skip(ctx.state.offset)
            .take(end - start)
        {
            let (x, y) = match self.start_corner {
                Corner::BottomLeft => {
                    current_height += item.height() as u16;
                    (list_area.left(), list_area.bottom() - current_height)
                }
                _ => {
                    let pos = (list_area.left(), list_area.top() + current_height);
                    current_height += item.height() as u16;
                    pos
                }
            };
            let area = Rect {
                x,
                y,
                width: list_area.width,
                height: item.height() as u16,
            };
            let item_style = self.style.patch(item.style);
            ctx.buffer.set_style(area, item_style);

            let is_selected = self.selected.map(|s| s == i).unwrap_or(false);
            let elem_x = if has_selection {
                let symbol = if is_selected {
                    highlight_symbol
                } else {
                    &blank_symbol
                };
                let (x, _) =
                    ctx.buffer
                        .set_stringn(x, y, symbol, list_area.width as usize, item_style);
                x
            } else {
                x
            };
            let max_element_width = (list_area.width - (elem_x - x)) as usize;
            for (j, line) in item.content.lines.iter().enumerate() {
                ctx.buffer
                    .set_spans(elem_x, y + j as u16, line, max_element_width as u16);
            }
            if is_selected {
                ctx.buffer.set_style(area, self.highlight_style);
            }
        }
    }
}
