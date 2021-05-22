use tui::{
    backend::TestBackend,
    border,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders},
    Terminal,
};

#[test]
fn border_macro_test() {
    let empty = Borders::empty();
    let all = Borders::all();
    let left_right = Borders::from_bits(Borders::LEFT.bits() | Borders::RIGHT.bits());
    assert_eq!(empty, border!());
    assert_eq!(all, border!(ALL));
    assert_eq!(all, border!(TOP, BOTTOM, LEFT, RIGHT));
    assert_eq!(left_right, Some(border!(RIGHT, LEFT)));
}
