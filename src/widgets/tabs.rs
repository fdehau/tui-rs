use unicode_width::UnicodeWidthStr;

use widgets::{Block, Widget};
use buffer::Buffer;
use layout::Rect;
use style::Color;
use symbols::line;

pub struct Tabs<'a> {
    block: Option<Block<'a>>,
    titles: &'a [&'a str],
    selected: usize,
    highlight_color: Color,
    color: Color,
}

impl<'a> Default for Tabs<'a> {
    fn default() -> Tabs<'a> {
        Tabs {
            block: None,
            titles: &[],
            selected: 0,
            highlight_color: Color::Reset,
            color: Color::Reset,
        }
    }
}

impl<'a> Tabs<'a> {
    pub fn block(&mut self, block: Block<'a>) -> &mut Tabs<'a> {
        self.block = Some(block);
        self
    }

    pub fn titles(&mut self, titles: &'a [&'a str]) -> &mut Tabs<'a> {
        self.titles = titles;
        self
    }

    pub fn select(&mut self, selected: usize) -> &mut Tabs<'a> {
        self.selected = selected;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Tabs<'a> {
        self.color = color;
        self
    }

    pub fn highlight_color(&mut self, color: Color) -> &mut Tabs<'a> {
        self.highlight_color = color;
        self
    }
}

impl<'a> Widget for Tabs<'a> {
    fn buffer(&self, area: &Rect, buf: &mut Buffer) {
        let tabs_area = match self.block {
            Some(b) => {
                b.buffer(area, buf);
                b.inner(area)
            }
            None => *area,
        };
        if tabs_area.height < 1 {
            return;
        }

        let mut x = tabs_area.left();
        for (title, color) in self.titles.iter().enumerate().map(|(i, t)| if i == self.selected {
            (t, self.highlight_color)
        } else {
            (t, self.color)
        }) {
            x += 1;
            if x > tabs_area.right() {
                break;
            } else {
                buf.set_string(x, tabs_area.top(), title, color, Color::Reset);
                x += title.width() as u16 + 1;
                if x > tabs_area.right() {
                    break;
                } else {
                    buf.set_cell(x,
                                 tabs_area.top(),
                                 line::VERTICAL,
                                 Color::Reset,
                                 Color::Reset);
                    x += 1;
                }
            }
        }
    }
}
