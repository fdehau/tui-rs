use crate::widgets::{RenderContext, Widget};

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
///     let block = Block::default().title("Block").borders(Borders::ALL);
///     f.render_widget(Clear, area); // <- this will clear/reset the area first
///     f.render_widget(block, area); // now render the block widget
/// }
/// ```
///
/// # Popup Example
///
/// For a more complete example how to utilize `Clear` to realize popups see
/// the example `examples/popup.rs`
#[derive(Debug, Clone)]
pub struct Clear;

impl Widget for Clear {
    type State = ();

    fn render(self, ctx: &mut RenderContext<Self::State>) {
        for x in ctx.area.left()..ctx.area.right() {
            for y in ctx.area.top()..ctx.area.bottom() {
                ctx.buffer.get_mut(x, y).reset();
            }
        }
    }
}
