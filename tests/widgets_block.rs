use tui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::{Rect, Alignment},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders},
    Terminal,
};

#[test]
fn widgets_block_renders() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let block = Block::default()
                .title(Span::styled("Title", Style::default().fg(Color::LightBlue)))
                .borders(Borders::ALL);
            f.render_widget(
                block,
                Rect {
                    x: 0,
                    y: 0,
                    width: 8,
                    height: 8,
                },
            );
        })
        .unwrap();
    let mut expected = Buffer::with_lines(vec![
        "┌Title─┐  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "└──────┘  ",
        "          ",
        "          ",
    ]);
    for x in 1..=5 {
        expected.get_mut(x, 0).set_fg(Color::LightBlue);
    }
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_on_small_areas() {
    let test_case = |block, area: Rect, expected| {
        let backend = TestBackend::new(area.width, area.height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                f.render_widget(block, area);
            })
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    let one_cell_test_cases = [
        (Borders::NONE, "T"),
        (Borders::LEFT, "│"),
        (Borders::TOP, "T"),
        (Borders::RIGHT, "│"),
        (Borders::BOTTOM, "T"),
        (Borders::ALL, "┌"),
    ];
    for (borders, symbol) in one_cell_test_cases.iter().cloned() {
        test_case(
            Block::default().title("Test").borders(borders),
            Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            Buffer::empty(Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            }),
        );
        test_case(
            Block::default().title("Test").borders(borders),
            Rect {
                x: 0,
                y: 0,
                width: 1,
                height: 0,
            },
            Buffer::empty(Rect {
                x: 0,
                y: 0,
                width: 1,
                height: 0,
            }),
        );
        test_case(
            Block::default().title("Test").borders(borders),
            Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 1,
            },
            Buffer::empty(Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 1,
            }),
        );
        test_case(
            Block::default().title("Test").borders(borders),
            Rect {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
            },
            Buffer::with_lines(vec![symbol]),
        );
    }
    test_case(
        Block::default().title("Test").borders(Borders::LEFT),
        Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
        },
        Buffer::with_lines(vec!["│Tes"]),
    );
    test_case(
        Block::default().title("Test").borders(Borders::RIGHT),
        Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
        },
        Buffer::with_lines(vec!["Tes│"]),
    );
    test_case(
        Block::default().title("Test").borders(Borders::RIGHT),
        Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
        },
        Buffer::with_lines(vec!["Tes│"]),
    );
    test_case(
        Block::default()
            .title("Test")
            .borders(Borders::LEFT | Borders::RIGHT),
        Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
        },
        Buffer::with_lines(vec!["│Te│"]),
    );
    test_case(
        Block::default().title("Test").borders(Borders::TOP),
        Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
        },
        Buffer::with_lines(vec!["Test"]),
    );
    test_case(
        Block::default().title("Test").borders(Borders::TOP),
        Rect {
            x: 0,
            y: 0,
            width: 5,
            height: 1,
        },
        Buffer::with_lines(vec!["Test─"]),
    );
    test_case(
        Block::default()
            .title("Test")
            .borders(Borders::LEFT | Borders::TOP),
        Rect {
            x: 0,
            y: 0,
            width: 5,
            height: 1,
        },
        Buffer::with_lines(vec!["┌Test"]),
    );
    test_case(
        Block::default()
            .title("Test")
            .borders(Borders::LEFT | Borders::TOP),
        Rect {
            x: 0,
            y: 0,
            width: 6,
            height: 1,
        },
        Buffer::with_lines(vec!["┌Test─"]),
    );
}

#[test]
fn widgets_block_renders_title_top_left_all_borders() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let block = Block::default()
                .title(Span::styled("Title", Style::default()))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL);

    let area = Rect {
        x: 1,
        y: 1,
        width: 18,
        height: 8,
    };

    terminal.draw(|f| { f.render_widget(block, area); }).unwrap();

    let expected = Buffer::with_lines(vec![
        "                    ",
        " ┌Title───────────┐ ",
        " │                │ ",
        " │                │ ",
        " │                │ ",
        " │                │ ",
        " │                │ ",
        " │                │ ",
        " └────────────────┘ ",
        "                    ",
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_title_top_left_no_left_border() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let block = Block::default()
                .title(Span::styled("Title", Style::default()))
                .title_alignment(Alignment::Left)
                .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM);

    let area = Rect {
        x: 1,
        y: 1,
        width: 18,
        height: 8,
    };

    terminal.draw(|f| { f.render_widget(block, area); }).unwrap();

    let expected = Buffer::with_lines(vec![
        "                    ",
        " Title────────────┐ ",
        "                  │ ",
        "                  │ ",
        "                  │ ",
        "                  │ ",
        "                  │ ",
        "                  │ ",
        " ─────────────────┘ ",
        "                    ",
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_title_top_left_no_right_border() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let block = Block::default()
                .title(Span::styled("Title", Style::default()))
                .title_alignment(Alignment::Left)
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM);

    let area = Rect {
        x: 1,
        y: 1,
        width: 18,
        height: 8,
    };

    terminal.draw(|f| { f.render_widget(block, area); }).unwrap();

    let expected = Buffer::with_lines(vec![
        "                    ",
        " ┌Title──────────── ",
        " │                  ",
        " │                  ",
        " │                  ",
        " │                  ",
        " │                  ",
        " │                  ",
        " └───────────────── ",
        "                    ",
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_title_top_left_no_borders() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let block = Block::default()
                .title(Span::styled("Title", Style::default()))
                .title_alignment(Alignment::Left)
                .borders(Borders::NONE);

    let area = Rect {
        x: 1,
        y: 1,
        width: 18,
        height: 8,
    };

    terminal.draw(|f| { f.render_widget(block, area); }).unwrap();

    let expected = Buffer::with_lines(vec![
        "                    ",
        " Title              ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_title_top_center_all_borders() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let block = Block::default()
                .title(Span::styled("Title", Style::default()))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);

    let area = Rect {
        x: 1,
        y: 1,
        width: 18,
        height: 8,
    };

    terminal.draw(|f| { f.render_widget(block, area); }).unwrap();

    let expected = Buffer::with_lines(vec![
        "                    ",
        " ┌─────Title──────┐ ",
        " │                │ ",
        " │                │ ",
        " │                │ ",
        " │                │ ",
        " │                │ ",
        " │                │ ",
        " └────────────────┘ ",
        "                    ",
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_title_top_center_no_left_border() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let block = Block::default()
                .title(Span::styled("Title", Style::default()))
                .title_alignment(Alignment::Center)
                .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM);

    let area = Rect {
        x: 1,
        y: 1,
        width: 18,
        height: 8,
    };

    terminal.draw(|f| { f.render_widget(block, area); }).unwrap();

    let expected = Buffer::with_lines(vec![
        "                    ",
        " ──────Title──────┐ ",
        "                  │ ",
        "                  │ ",
        "                  │ ",
        "                  │ ",
        "                  │ ",
        "                  │ ",
        " ─────────────────┘ ",
        "                    ",
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_title_top_center_no_right_border() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let block = Block::default()
                .title(Span::styled("Title", Style::default()))
                .title_alignment(Alignment::Center)
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM);

    let area = Rect {
        x: 1,
        y: 1,
        width: 18,
        height: 8,
    };

    terminal.draw(|f| { f.render_widget(block, area); }).unwrap();

    let expected = Buffer::with_lines(vec![
        "                    ",
        " ┌──────Title────── ",
        " │                  ",
        " │                  ",
        " │                  ",
        " │                  ",
        " │                  ",
        " │                  ",
        " └───────────────── ",
        "                    ",
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_title_top_center_no_borders() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let block = Block::default()
                .title(Span::styled("Title", Style::default()))
                .title_alignment(Alignment::Center)
                .borders(Borders::NONE);

    let area = Rect {
        x: 1,
        y: 1,
        width: 18,
        height: 8,
    };

    terminal.draw(|f| { f.render_widget(block, area); }).unwrap();

    let expected = Buffer::with_lines(vec![
        "                    ",
        "       Title        ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_title_top_right_all_borders() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let block = Block::default()
                .title(Span::styled("Title", Style::default()))
                .title_alignment(Alignment::Right)
                .borders(Borders::ALL);

    let area = Rect {
        x: 1,
        y: 1,
        width: 18,
        height: 8,
    };

    terminal.draw(|f| { f.render_widget(block, area); }).unwrap();

    let expected = Buffer::with_lines(vec![
        "                    ",
        " ┌───────────Title┐ ",
        " │                │ ",
        " │                │ ",
        " │                │ ",
        " │                │ ",
        " │                │ ",
        " │                │ ",
        " └────────────────┘ ",
        "                    ",
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_title_top_right_no_left_border() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let block = Block::default()
                .title(Span::styled("Title", Style::default()))
                .title_alignment(Alignment::Right)
                .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM);

    let area = Rect {
        x: 1,
        y: 1,
        width: 18,
        height: 8,
    };

    terminal.draw(|f| { f.render_widget(block, area); }).unwrap();

    let expected = Buffer::with_lines(vec![
        "                    ",
        " ────────────Title┐ ",
        "                  │ ",
        "                  │ ",
        "                  │ ",
        "                  │ ",
        "                  │ ",
        "                  │ ",
        " ─────────────────┘ ",
        "                    ",
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_title_top_right_no_right_border() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let block = Block::default()
                .title(Span::styled("Title", Style::default()))
                .title_alignment(Alignment::Right)
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM);

    let area = Rect {
        x: 1,
        y: 1,
        width: 18,
        height: 8,
    };

    terminal.draw(|f| { f.render_widget(block, area); }).unwrap();

    let expected = Buffer::with_lines(vec![
        "                    ",
        " ┌────────────Title ",
        " │                  ",
        " │                  ",
        " │                  ",
        " │                  ",
        " │                  ",
        " │                  ",
        " └───────────────── ",
        "                    ",
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_title_top_right_no_borders() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let block = Block::default()
                .title(Span::styled("Title", Style::default()))
                .title_alignment(Alignment::Right)
                .borders(Borders::NONE);

    let area = Rect {
        x: 1,
        y: 1,
        width: 18,
        height: 8,
    };

    terminal.draw(|f| { f.render_widget(block, area); }).unwrap();

    let expected = Buffer::with_lines(vec![
        "                    ",
        "              Title ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
    ]);

    terminal.backend().assert_buffer(&expected);
}