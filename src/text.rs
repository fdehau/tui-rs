//! Primitives for styled text.
//!
//! A terminal UI is at its root a lot of strings. In order to make it accessible and stylish,
//! those strings may be associated to a set of styles. `tui` has three ways to represent them:
//! - A single line string where all graphemes have the same style is represented by a [`Span`].
//! - A single line string where each grapheme may have its own style is represented by [`Spans`].
//! - A multiple line string where each grapheme may have its own style is represented by a
//! [`Text`].
//!
//! These types form a hierarchy: [`Spans`] is a collection of [`Span`] and each line of [`Text`]
//! is a [`Spans`].
//!
//! Keep it mind that a lot of widgets will use those types to advertise what kind of string is
//! supported for their properties. Moreover, `tui` provides convenient `From` implementations so
//! that you can start by using simple `String` or `&str` and then promote them to the previous
//! primitives when you need additional styling capabilities.
//!
//! For example, for the [`crate::widgets::Block`] widget, all the following calls are valid to set
//! its `title` property (which is a [`Spans`] under the hood):
//!
//! ```rust
//! # use tui::widgets::Block;
//! # use tui::text::{Span, Spans};
//! # use tui::style::{Color, Style};
//! // A simple string with no styling.
//! // Converted to Spans(vec![
//! //   Span { content: Cow::Borrowed("My title"), style: Style { .. } }
//! // ])
//! let block = Block::default().title("My title");
//!
//! // A simple string with a unique style.
//! // Converted to Spans(vec![
//! //   Span { content: Cow::Borrowed("My title"), style: Style { fg: Some(Color::Yellow), .. }
//! // ])
//! let block = Block::default().title(
//!     Span::styled("My title", Style::default().fg(Color::Yellow))
//! );
//!
//! // A string with multiple styles.
//! // Converted to Spans(vec![
//! //   Span { content: Cow::Borrowed("My"), style: Style { fg: Some(Color::Yellow), .. } },
//! //   Span { content: Cow::Borrowed(" title"), .. }
//! // ])
//! let block = Block::default().title(vec![
//!     Span::styled("My", Style::default().fg(Color::Yellow)),
//!     Span::raw(" title"),
//! ]);
//! ```
use crate::style::Style;
use std::{borrow::Cow, cmp::max};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// A grapheme associated to a style.
#[derive(Debug, Clone, PartialEq)]
pub struct StyledGrapheme<'a> {
    pub symbol: &'a str,
    pub style: Style,
}

/// A string where all graphemes have the same style.
#[derive(Debug, Clone, PartialEq)]
pub struct Span<'a> {
    pub content: Cow<'a, str>,
    pub style: Style,
}

impl<'a> Span<'a> {
    /// Create a span with no style.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::text::Span;
    /// Span::raw("My text");
    /// Span::raw(String::from("My text"));
    /// ```
    pub fn raw<T>(content: T) -> Span<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Span {
            content: content.into(),
            style: Style::default(),
        }
    }

    /// Create a span with a style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tui::text::Span;
    /// # use tui::style::{Color, Modifier, Style};
    /// let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    /// Span::styled("My text", style);
    /// Span::styled(String::from("My text"), style);
    /// ```
    pub fn styled<T>(content: T, style: Style) -> Span<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Span {
            content: content.into(),
            style,
        }
    }

    /// Returns the width of the content held by this span.
    pub fn width(&self) -> usize {
        self.content.width()
    }

    /// Returns an iterator over the graphemes held by this span.
    ///
    /// `base_style` is the [`Style`] that will be patched with each grapheme [`Style`] to get
    /// the resulting [`Style`].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::text::{Span, StyledGrapheme};
    /// # use tui::style::{Color, Modifier, Style};
    /// # use std::iter::Iterator;
    /// let style = Style::default().fg(Color::Yellow);
    /// let span = Span::styled("Text", style);
    /// let style = Style::default().fg(Color::Green).bg(Color::Black);
    /// let styled_graphemes = span.styled_graphemes(style);
    /// assert_eq!(
    ///     vec![
    ///         StyledGrapheme {
    ///             symbol: "T",
    ///             style: Style {
    ///                 fg: Some(Color::Yellow),
    ///                 bg: Some(Color::Black),
    ///                 add_modifier: Modifier::empty(),
    ///                 sub_modifier: Modifier::empty(),
    ///             },
    ///         },
    ///         StyledGrapheme {
    ///             symbol: "e",
    ///             style: Style {
    ///                 fg: Some(Color::Yellow),
    ///                 bg: Some(Color::Black),
    ///                 add_modifier: Modifier::empty(),
    ///                 sub_modifier: Modifier::empty(),
    ///             },
    ///         },
    ///         StyledGrapheme {
    ///             symbol: "x",
    ///             style: Style {
    ///                 fg: Some(Color::Yellow),
    ///                 bg: Some(Color::Black),
    ///                 add_modifier: Modifier::empty(),
    ///                 sub_modifier: Modifier::empty(),
    ///             },
    ///         },
    ///         StyledGrapheme {
    ///             symbol: "t",
    ///             style: Style {
    ///                 fg: Some(Color::Yellow),
    ///                 bg: Some(Color::Black),
    ///                 add_modifier: Modifier::empty(),
    ///                 sub_modifier: Modifier::empty(),
    ///             },
    ///         },
    ///     ],
    ///     styled_graphemes.collect::<Vec<StyledGrapheme>>()
    /// );
    /// ```
    pub fn styled_graphemes(
        &'a self,
        base_style: Style,
    ) -> impl Iterator<Item = StyledGrapheme<'a>> {
        UnicodeSegmentation::graphemes(self.content.as_ref(), true)
            .map(move |g| StyledGrapheme {
                symbol: g,
                style: base_style.patch(self.style),
            })
            .filter(|s| s.symbol != "\n")
    }
}

impl<'a> From<String> for Span<'a> {
    fn from(s: String) -> Span<'a> {
        Span::raw(s)
    }
}

impl<'a> From<&'a str> for Span<'a> {
    fn from(s: &'a str) -> Span<'a> {
        Span::raw(s)
    }
}

/// A string composed of clusters of graphemes, each with their own style.
#[derive(Debug, Clone, PartialEq)]
pub struct Spans<'a>(pub Vec<Span<'a>>);

impl<'a> Default for Spans<'a> {
    fn default() -> Spans<'a> {
        Spans(Vec::new())
    }
}

impl<'a> Spans<'a> {
    /// Returns the width of the underlying string.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::text::{Span, Spans};
    /// # use tui::style::{Color, Style};
    /// let spans = Spans::from(vec![
    ///     Span::styled("My", Style::default().fg(Color::Yellow)),
    ///     Span::raw(" text"),
    /// ]);
    /// assert_eq!(7, spans.width());
    /// ```
    pub fn width(&self) -> usize {
        self.0.iter().fold(0, |acc, s| acc + s.width())
    }
}

impl<'a> From<String> for Spans<'a> {
    fn from(s: String) -> Spans<'a> {
        Spans(vec![Span::from(s)])
    }
}

impl<'a> From<&'a str> for Spans<'a> {
    fn from(s: &'a str) -> Spans<'a> {
        Spans(vec![Span::from(s)])
    }
}

impl<'a> From<Vec<Span<'a>>> for Spans<'a> {
    fn from(spans: Vec<Span<'a>>) -> Spans<'a> {
        Spans(spans)
    }
}

impl<'a> From<Span<'a>> for Spans<'a> {
    fn from(span: Span<'a>) -> Spans<'a> {
        Spans(vec![span])
    }
}

impl<'a> From<Spans<'a>> for String {
    fn from(line: Spans<'a>) -> String {
        line.0.iter().fold(String::new(), |mut acc, s| {
            acc.push_str(s.content.as_ref());
            acc
        })
    }
}

/// A string split over multiple lines where each line is composed of several clusters, each with
/// their own style.
#[derive(Debug, Clone, PartialEq)]
pub struct Text<'a> {
    pub lines: Vec<Spans<'a>>,
}

impl<'a> Default for Text<'a> {
    fn default() -> Text<'a> {
        Text { lines: Vec::new() }
    }
}

impl<'a> Text<'a> {
    /// Returns the max width of all the lines.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use tui::text::Text;
    /// let text = Text::from("The first line\nThe second line");
    /// assert_eq!(15, text.width());
    /// ```
    pub fn width(&self) -> usize {
        self.lines.iter().fold(0, |acc, l| max(acc, l.width()))
    }

    /// Returns the height.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use tui::text::Text;
    /// let text = Text::from("The first line\nThe second line");
    /// assert_eq!(2, text.height());
    /// ```
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn patch_style(&mut self, style: Style) {
        for line in &mut self.lines {
            for span in &mut line.0 {
                span.style = span.style.patch(style);
            }
        }
    }
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(s: &'a str) -> Text<'a> {
        Text {
            lines: s.lines().map(Spans::from).collect(),
        }
    }
}

impl<'a> From<Span<'a>> for Text<'a> {
    fn from(span: Span<'a>) -> Text<'a> {
        Text {
            lines: vec![Spans::from(span)],
        }
    }
}

impl<'a> From<Spans<'a>> for Text<'a> {
    fn from(spans: Spans<'a>) -> Text<'a> {
        Text { lines: vec![spans] }
    }
}

impl<'a> From<Vec<Spans<'a>>> for Text<'a> {
    fn from(lines: Vec<Spans<'a>>) -> Text<'a> {
        Text { lines }
    }
}
