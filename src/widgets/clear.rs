use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::widgets::Widget;

/// A widget to to clear/reset a certain area to allow overdrawing (e.g. for popups)
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Clear, Block, Borders};
/// # use tui::layout::Rect;
/// # use tui::Frame;
/// # use tui::backend::Backend;
/// fn draw_on_clear<B: Backend>(f: &mut Frame<B>, area: Rect) {
///     let mut block = Block::default().title("Block").borders(Borders::ALL);
///     f.render(&mut Clear, area); // <- this will clear/reset the area first
///     f.render(&mut block, area); // now render the block widget
/// }
/// ```
///
/// # Popup Example
///
/// For a more complete example how to utilize `Clear` to realize popups see
/// the example `examples/popup.rs`
#[derive(Debug, Clone, Default)]
pub struct Clear;

impl Widget for Clear {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y).reset();
            }
        }
    }
}
