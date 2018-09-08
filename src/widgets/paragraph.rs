use std::borrow::Cow;

use either::Either;
use itertools::{multipeek, MultiPeek};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use buffer::Buffer;
use layout::{Alignment, Rect};
use style::Style;
use widgets::{Block, Widget};

/// A widget to display some text.
///
/// # Examples
///
/// ```
/// # extern crate tui;
/// # use tui::widgets::{Block, Borders, Paragraph, Text};
/// # use tui::style::{Style, Color};
/// # use tui::layout::{Alignment};
/// # fn main() {
/// let text = [
///     Text::raw("First line\n"),
///     Text::styled("Second line\n", Style::default().fg(Color::Red))
/// ];
/// Paragraph::new(text.iter())
///     .block(Block::default().title("Paragraph").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White).bg(Color::Black))
///     .alignment(Alignment::Center)
///     .wrap(true);
/// # }
/// ```
pub struct Paragraph<'a, 't, T>
where
    T: Iterator<Item = &'t Text<'t>>,
{
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Widget style
    style: Style,
    /// Wrap the text or not
    wrapping: bool,
    /// The text to display
    text: T,
    /// Should we parse the text for embedded commands
    raw: bool,
    /// Scroll
    scroll: u16,
    /// Aligenment of the text
    alignment: Alignment,
}

pub enum Text<'b> {
    Raw(Cow<'b, str>),
    Styled(Cow<'b, str>, Style),
}

impl<'b> Text<'b> {
    pub fn raw<D: Into<Cow<'b, str>>>(data: D) -> Text<'b> {
        Text::Raw(data.into())
    }

    pub fn styled<D: Into<Cow<'b, str>>>(data: D, style: Style) -> Text<'b> {
        Text::Styled(data.into(), style)
    }
}

impl<'a, 't, T> Paragraph<'a, 't, T>
where
    T: Iterator<Item = &'t Text<'t>>,
{
    pub fn new(text: T) -> Paragraph<'a, 't, T> {
        Paragraph {
            block: None,
            style: Default::default(),
            wrapping: false,
            raw: false,
            text,
            scroll: 0,
            alignment: Alignment::Left,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Paragraph<'a, 't, T> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Paragraph<'a, 't, T> {
        self.style = style;
        self
    }

    pub fn wrap(mut self, flag: bool) -> Paragraph<'a, 't, T> {
        self.wrapping = flag;
        self
    }

    pub fn raw(mut self, flag: bool) -> Paragraph<'a, 't, T> {
        self.raw = flag;
        self
    }

    pub fn scroll(mut self, offset: u16) -> Paragraph<'a, 't, T> {
        self.scroll = offset;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Paragraph<'a, 't, T> {
        self.alignment = alignment;
        self
    }
}

impl<'a, 't, T> Widget for Paragraph<'a, 't, T>
where
    T: Iterator<Item = &'t Text<'t>>,
{
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let text_area = match self.block {
            Some(ref mut b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => area,
        };

        if text_area.height < 1 {
            return;
        }

        self.background(&text_area, buf, self.style.bg);

        let style = self.style;
        let styled = self.text.by_ref().flat_map(|t| match *t {
            Text::Raw(ref d) => {
                let data: &'t str = d; // coerce to &str
                Either::Left(UnicodeSegmentation::graphemes(data, true).map(|g| (g, style)))
            }
            Text::Styled(ref d, s) => {
                let data: &'t str = d; // coerce to &str
                Either::Right(UnicodeSegmentation::graphemes(data, true).map(move |g| (g, s)))
            }
        });
        let mut styled = multipeek(styled);

        fn get_cur_line_len<'a, I: Iterator<Item = (&'a str, Style)>>(
            styled: &mut MultiPeek<I>,
        ) -> u16 {
            let mut line_len = 0;
            while match &styled.peek() {
                Some(&(x, _)) => x != "\n",
                None => false,
            } {
                line_len += 1;
            }
            line_len
        };

        let mut x = match self.alignment {
            Alignment::Center => {
                (text_area.width / 2).saturating_sub(get_cur_line_len(&mut styled) / 2)
            }
            Alignment::Right => (text_area.width).saturating_sub(get_cur_line_len(&mut styled)),
            Alignment::Left => 0,
        };
        let mut y = 0;

        let mut remove_leading_whitespaces = false;
        while let Some((string, style)) = styled.next() {
            if string == "\n" {
                x = match self.alignment {
                    Alignment::Center => {
                        (text_area.width / 2).saturating_sub(get_cur_line_len(&mut styled) / 2)
                    }

                    Alignment::Right => {
                        (text_area.width).saturating_sub(get_cur_line_len(&mut styled))
                    }
                    Alignment::Left => 0,
                };
                y += 1;
                continue;
            }
            if x >= text_area.width && self.wrapping {
                x = match self.alignment {
                    Alignment::Center => {
                        (text_area.width / 2).saturating_sub(get_cur_line_len(&mut styled) / 2)
                    }

                    Alignment::Right => {
                        (text_area.width).saturating_sub(get_cur_line_len(&mut styled) + 1)
                    }
                    Alignment::Left => 0,
                };
                y += 1;
                remove_leading_whitespaces = true
            }

            if remove_leading_whitespaces && string == " " {
                continue;
            }
            remove_leading_whitespaces = false;

            if y > text_area.height + self.scroll - 1 {
                break;
            }

            if y < self.scroll {
                continue;
            }

            buf.get_mut(text_area.left() + x, text_area.top() + y - self.scroll)
                .set_symbol(string)
                .set_style(style);
            x += string.width() as u16;
        }
    }
}
