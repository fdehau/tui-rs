use tui::{backend::TestBackend, buffer::Buffer, layout::Rect, widgets::SevenSegment, Terminal};

#[test]
fn widgets_seven_segment_full() {
    let backend = TestBackend::new(60, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let display = SevenSegment::new("12:34:56.78, 90").unwrap();
            f.render_widget(
                display,
                Rect {
                    x: 1,
                    y: 1,
                    width: 58,
                    height: 3,
                },
            );
        })
        .unwrap();
    let expected = Buffer::with_lines(vec![
        "                                                            ",
        "    ╷ ╶──┐ ╶──┐ ╷  ╷ ┌──╴ ┌──╴ ╶──┐ ┌──┐      ┌──┐ ┌──┐     ",
        "    │ ┌──┘: ──┤ └──┤:└──┐ ├──┐    │ ├──┤      └──┤ │  │     ",
        "    ╵ └──╴ ╶──┘    ╵ ╶──┘ └──┘.   ╵ └──┘,     ╶──┘ └──┘     ",
        "                                                            ",
    ]);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_seven_segment_half() {
    let backend = TestBackend::new(28, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let display = SevenSegment::new("12:34:56.78, 90").unwrap();
            f.render_widget(
                display,
                Rect {
                    x: 1,
                    y: 1,
                    width: 27,
                    height: 3,
                },
            );
        })
        .unwrap();
    let expected = Buffer::with_lines(vec![
        "                            ",
        "    ╷ ╶──┐ ╶──┐ ╷  ╷ ┌──╴ ┌─",
        "    │ ┌──┘: ──┤ └──┤:└──┐ ├─",
        "    ╵ └──╴ ╶──┘    ╵ ╶──┘ └─",
        "                            ",
    ]);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_seven_segment_trim() {
    let backend = TestBackend::new(54, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let display = SevenSegment::new("12:34:56.78, 90").unwrap();
            f.render_widget(
                display,
                Rect {
                    x: 0,
                    y: 0,
                    width: 54,
                    height: 3,
                },
            );
        })
        .unwrap();
    let expected = Buffer::with_lines(vec![
        "   ╷ ╶──┐ ╶──┐ ╷  ╷ ┌──╴ ┌──╴ ╶──┐ ┌──┐      ┌──┐ ┌──┐",
        "   │ ┌──┘: ──┤ └──┤:└──┐ ├──┐    │ ├──┤      └──┤ │  │",
        "   ╵ └──╴ ╶──┘    ╵ ╶──┘ └──┘.   ╵ └──┘,     ╶──┘ └──┘",
    ]);
    terminal.backend().assert_buffer(&expected);
}
