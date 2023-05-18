use std::error::Error;
use tui::buffer::Cell;
use tui::style::{Color, Modifier};
use tui::{
    backend::{Backend, TestBackend},
    layout::Rect,
    widgets::Paragraph,
    Frame, Terminal,
};

#[test]
fn terminal_buffer_size_should_be_limited() {
    let backend = TestBackend::new(400, 400);
    let terminal = Terminal::new(backend).unwrap();
    let size = terminal.backend().size().unwrap();
    assert_eq!(size.width, 255);
    assert_eq!(size.height, 255);
}

#[test]
fn terminal_draw_returns_the_completed_frame() -> Result<(), Box<dyn Error>> {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend)?;
    let frame = terminal.draw(|f| {
        let paragrah = Paragraph::new("Test");
        f.render_widget(paragrah, f.size());
    })?;
    assert_eq!(frame.buffer.get(0, 0).symbol, "T");
    assert_eq!(frame.area, Rect::new(0, 0, 10, 10));
    terminal.backend_mut().resize(8, 8);
    let frame = terminal.draw(|f| {
        let paragrah = Paragraph::new("test");
        f.render_widget(paragrah, f.size());
    })?;
    assert_eq!(frame.buffer.get(0, 0).symbol, "t");
    assert_eq!(frame.area, Rect::new(0, 0, 8, 8));
    Ok(())
}

#[test]
fn terminal_clear_wipes_terminal_and_does_full_redraw() -> Result<(), Box<dyn Error>> {
    let backend = TestBackend::new(5, 5);
    let mut terminal = Terminal::new(backend)?;
    let draw_fun = |f: &mut Frame<TestBackend>| {
        let paragrah = Paragraph::new("Tests\n".repeat(5));
        f.render_widget(paragrah, f.size());
    };
    terminal.draw(draw_fun)?;
    terminal.clear()?;
    assert!(terminal
        .backend()
        .buffer()
        .content
        .iter()
        .find(|cell| cell.symbol.ne(" "))
        .is_none());
    terminal.draw(draw_fun)?;
    assert_eq!(
        terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|cell| cell.symbol.clone())
            .collect::<Vec<_>>()
            .join(""),
        "Tests".repeat(5)
    );
    Ok(())
}

#[test]
fn terminal_mark_dirty_does_full_redraw() -> Result<(), Box<dyn Error>> {
    let backend = TestBackend::new(5, 5);
    let mut terminal = Terminal::new(backend)?;
    let draw_fun = |f: &mut Frame<TestBackend>| {
        let paragrah = Paragraph::new("Tests\n".repeat(5));
        f.render_widget(paragrah, f.size());
    };
    terminal.draw(draw_fun)?;
    terminal.mark_dirty();
    let mut fill_cell = Cell {
        symbol: "#".to_string(),
        fg: Color::Gray,
        bg: Color::Gray,
        modifier: Modifier::all(),
    };
    for row in 0..5 {
        for col in 0..5 {
            terminal
                .backend_mut()
                .draw(std::iter::once((row, col, &fill_cell)))?;
        }
    }
    terminal.draw(draw_fun)?;
    assert_eq!(
        terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|cell| cell.symbol.clone())
            .collect::<Vec<_>>()
            .join(""),
        "Tests".repeat(5)
    );
    Ok(())
}
