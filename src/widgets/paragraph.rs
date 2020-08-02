use crate::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{StyledGrapheme, Text},
    widgets::{
        reflow::{LineComposer, LineTruncator, WordWrapper},
        Block, Widget,
    },
};
use std::iter;
use unicode_width::UnicodeWidthStr;

fn get_line_offset(line_width: u16, text_area_width: u16, alignment: Alignment) -> u16 {
    match alignment {
        Alignment::Center => (text_area_width / 2).saturating_sub(line_width / 2),
        Alignment::Right => text_area_width.saturating_sub(line_width),
        Alignment::Left => 0,
    }
}

/// A widget to display some text.
///
/// # Examples
///
/// ```
/// # use tui::text::{Text, Spans, Span};
/// # use tui::widgets::{Block, Borders, Paragraph, Wrap};
/// # use tui::style::{Style, Color, Modifier};
/// # use tui::layout::{Alignment};
/// let text = vec![
///     Spans::from(vec![
///         Span::raw("First"),
///         Span::styled("line",Style::default().add_modifier(Modifier::ITALIC)),
///         Span::raw("."),
///     ]),
///     Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
/// ];
/// Paragraph::new(text)
///     .block(Block::default().title("Paragraph").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White).bg(Color::Black))
///     .alignment(Alignment::Center)
///     .wrap(Wrap::default());
/// ```
pub struct Paragraph<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Widget style
    style: Style,
    /// How to wrap the text
    wrap: Option<Wrap>,
    /// The text to display
    text: Text<'a>,
    /// Scroll
    scroll: (u16, u16),
    /// Alignment of the text
    alignment: Alignment,
}

/// Describes how to wrap text across lines.
///
/// ## Examples
///
/// ```
/// # use tui::widgets::{Paragraph, Wrap};
/// # use tui::text::Text;
/// let bullet_points = Text::from(r#"Some indented points:
///     - First thing goes here and is long so that it wraps
///     - Here is another point that is long enough to wrap"#);
///
/// // With leading spaces trimmed (window width of 30 chars):
/// Paragraph::new(bullet_points.clone()).wrap(Wrap::default());
/// // Some indented points:
/// // - First thing goes here and is
/// // long so that it wraps
/// // - Here is another point that
/// // is long enough to wrap
///
/// // But without trimming, indentation is preserved:
/// Paragraph::new(bullet_points).wrap(Wrap { trim: false, ..Wrap::default() });
/// // Some indented points:
/// //     - First thing goes here
/// // and is long so that it wraps
/// //     - Here is another point
/// // that is long enough to wrap
/// ```
pub struct Wrap {
    /// Should leading whitespace be trimmed
    pub trim: bool,
    pub scroll_callback: Option<Box<ScrollCallback>>,
}

impl Default for Wrap {
    fn default() -> Wrap {
        Wrap {
            trim: true,
            scroll_callback: None,
        }
    }
}

pub type ScrollCallback = dyn FnOnce(Rect, &[(Vec<StyledGrapheme<'_>>, u16)]) -> (u16, u16);

impl<'a> Paragraph<'a> {
    pub fn new<T>(text: T) -> Paragraph<'a>
    where
        T: Into<Text<'a>>,
    {
        Paragraph {
            block: None,
            style: Default::default(),
            wrap: None,
            text: text.into(),
            scroll: (0, 0),
            alignment: Alignment::Left,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Paragraph<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Paragraph<'a> {
        self.style = style;
        self
    }

    pub fn wrap(mut self, wrap: Wrap) -> Paragraph<'a> {
        self.wrap = Some(wrap);
        self
    }

    pub fn scroll(mut self, offset: (u16, u16)) -> Paragraph<'a> {
        self.scroll = offset;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Paragraph<'a> {
        self.alignment = alignment;
        self
    }

    fn draw_lines<'b, T>(
        &self,
        text_area: Rect,
        buf: &mut Buffer,
        mut line_composer: T,
        scroll: (u16, u16),
    ) where
        T: LineComposer<'b>,
    {
        let mut y = 0;
        let mut i = 0;
        while let Some((current_line, current_line_width)) = line_composer.next_line() {
            if i >= scroll.0 {
                let cell_y = text_area.top().saturating_add(y);
                let mut x = get_line_offset(current_line_width, text_area.width, self.alignment);
                for StyledGrapheme { symbol, style } in current_line {
                    buf.get_mut(text_area.left() + x, cell_y)
                        .set_symbol(if symbol.is_empty() {
                            // If the symbol is empty, the last char which rendered last time will
                            // leave on the line. It's a quick fix.
                            " "
                        } else {
                            symbol
                        })
                        .set_style(*style);
                    x += symbol.width() as u16;
                }
                y += 1;
            }
            i += 1;
            if y >= text_area.height {
                break;
            }
        }
    }
}

impl<'a> Widget for Paragraph<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let text_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if text_area.height < 1 {
            return;
        }

        let style = self.style;
        let mut styled = self.text.lines.iter().flat_map(|spans| {
            spans
                .0
                .iter()
                .flat_map(|span| span.styled_graphemes(style))
                // Required given the way composers work but might be refactored out if we change
                // composers to operate on lines instead of a stream of graphemes.
                .chain(iter::once(StyledGrapheme {
                    symbol: "\n",
                    style,
                }))
        });

        match self.wrap {
            None => {
                let mut line_composer = LineTruncator::new(&mut styled, text_area.width);
                if let Alignment::Left = self.alignment {
                    line_composer.set_horizontal_offset(self.scroll.1);
                }
                self.draw_lines(text_area, buf, line_composer, self.scroll);
            }
            Some(Wrap {
                trim,
                scroll_callback: None,
            }) => {
                let line_composer = WordWrapper::new(&mut styled, text_area.width, trim);
                self.draw_lines(text_area, buf, line_composer, self.scroll);
            }
            Some(Wrap {
                trim,
                ref mut scroll_callback,
            }) => {
                let mut line_composer = WordWrapper::new(&mut styled, text_area.width, trim);
                let mut lines = Vec::new();
                while let Some((current_line, current_line_width)) = line_composer.next_line() {
                    lines.push((Vec::from(current_line), current_line_width));
                }
                let f = scroll_callback.take().unwrap();
                let scroll = f(text_area, lines.as_ref());
                self.draw_lines(text_area, buf, WrappedLines { lines, index: 0 }, scroll);
            }
        };
    }
}

struct WrappedLines<'a> {
    lines: Vec<(Vec<StyledGrapheme<'a>>, u16)>,
    index: usize,
}

impl<'a> LineComposer<'a> for WrappedLines<'a> {
    fn next_line(&mut self) -> Option<(&[StyledGrapheme<'a>], u16)> {
        if self.index >= self.lines.len() {
            return None;
        }
        let (line, width) = &self.lines[self.index];
        self.index += 1;
        Some((&line, *width))
    }
}
