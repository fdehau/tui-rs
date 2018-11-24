extern crate tui;

use tui::backend::{Backend, TestBackend};
use tui::Terminal;

#[test]
fn buffer_size_limited() {
    let backend = TestBackend::new(400, 400);
    let terminal = Terminal::new(backend).unwrap();
    let size = terminal.backend().size().unwrap();
    assert_eq!(size.width, 255);
    assert_eq!(size.height, 255);
}
