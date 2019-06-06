#[allow(dead_code)]
mod util;

use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::{backend::TermionBackend, Terminal};

use crate::util::event::{Event, Events};
use tui::style::{Color, Style};
use tui::widgets::Text;

/// This list does not implement widget, but instead provides a render call taking properties.
/// On top of that, it can keep its own state, which can conveniently be accessible to the parent application
/// to control it. However, the component is able to write its own state and adjust it based on information
/// it only obtains when drawing.
///
/// If this seems a little like React, you might be right!
mod list {
    use tui::{
        buffer::Buffer,
        layout::Rect,
        widgets::{Block, Paragraph, Text, Widget},
    };

    /// Thanks to the state maintained in this list, it can scroll naturally.
    /// Compare this to the `List` `Widget` in TUI, which seems to 'stick to the bottom'.
    #[derive(Default)]
    pub struct StatefulList {
        /// The index at which the list last started. Used for scrolling
        offset: usize,
    }

    #[derive(Default)]
    pub struct StatefulListProps<'b> {
        pub block: Option<Block<'b>>,
        pub entry_in_view: Option<usize>,
    }

    impl StatefulList {
        fn list_offset_for(&self, entry_in_view: Option<usize>, height: usize) -> usize {
            match entry_in_view {
                Some(pos) => match height as usize {
                    h if self.offset + h - 1 < pos => pos - h + 1,
                    _ if self.offset > pos => pos,
                    _ => self.offset,
                },
                None => 0,
            }
        }
    }

    impl StatefulList {
        pub fn render<'a, 't>(
            &mut self,
            props: StatefulListProps<'a>,
            items: impl IntoIterator<Item = Vec<Text<'t>>>,
            area: Rect,
            buf: &mut Buffer,
        ) {
            let StatefulListProps {
                block,
                entry_in_view,
            } = props;

            let list_area = match block {
                Some(mut b) => {
                    b.draw(area, buf);
                    b.inner(area)
                }
                None => area,
            };
            // Here is the magic - we mutate our own state to automatically handle proper scrolling.
            // The same can be accomplished with stateless components, but then the caller has to know
            // and maintain all of its state somewhere.
            // Bringing the state to where it is 'owned' is very convenient.
            self.offset = self.list_offset_for(entry_in_view, list_area.height as usize);

            if list_area.width < 1 || list_area.height < 1 {
                return;
            }

            for (i, text_iterator) in items
                .into_iter()
                .skip(self.offset)
                .enumerate()
                .take(list_area.height as usize)
            {
                let (x, y) = (list_area.left(), list_area.top() + i as u16);
                Paragraph::new(text_iterator.iter()).draw(
                    Rect {
                        x,
                        y,
                        width: list_area.width,
                        height: 1,
                    },
                    buf,
                );
            }
        }
    }
}

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut list = list::StatefulList::default();
    let mut selected = 0;

    terminal.hide_cursor()?;
    let labels = [
        "j or <down> for going down",
        "k or <up> for going up",
        "Ctrl + u for going up fast",
        "Ctrl + d for going down fast",
        "foo",
        "bar",
        "baz",
        "yes",
        "no",
        "maybe",
        "great",
        "awesome",
        "fantastic!",
    ];
    const NUM_ENTRIES: usize = 200;
    loop {
        let area = terminal.pre_draw()?;
        let props = list::StatefulListProps {
            block: None,
            entry_in_view: Some(selected),
        };
        let entries = (0..NUM_ENTRIES).map(|idx| {
            let (fg, bg) = if idx == selected {
                (Color::Yellow, Color::Blue)
            } else {
                (Color::White, Color::Reset)
            };
            vec![
                Text::Styled(
                    format!(" {:>3}. ", idx + 1).into(),
                    Style {
                        fg: Color::Red,
                        bg,
                        ..Default::default()
                    },
                ),
                Text::Styled(
                    labels[idx % labels.len()].into(),
                    Style {
                        fg,
                        bg,
                        ..Default::default()
                    },
                ),
            ]
        });
        list.render(props, entries, area, terminal.current_buffer_mut());
        terminal.post_draw()?;

        use Key::*;
        match events.next()? {
            Event::Input(key) => {
                selected = match key {
                    Char('j') | Down => selected.saturating_add(1),
                    Ctrl('d') | PageDown => selected.saturating_add(10),
                    Char('k') | Up => selected.saturating_sub(1),
                    Ctrl('u') | PageUp => selected.saturating_sub(10),
                    Char('q') => break,
                    _ => selected,
                }
                .min(NUM_ENTRIES - 1)
            }
            _ => {}
        }
    }

    Ok(())
}
