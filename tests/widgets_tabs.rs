use tui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::{Margin, Rect},
    symbols,
    widgets::Tabs,
    Terminal,
};

#[test]
fn widgets_tabs_should_not_panic_on_narrow_areas() {
    let backend = TestBackend::new(1, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|mut f| {
            let tabs = Tabs::default().titles(&["Tab1", "Tab2"]).margin(Margin {
                horizontal: 0,
                vertical: 0,
            });
            f.render_widget(
                tabs,
                Rect {
                    x: 0,
                    y: 0,
                    width: 1,
                    height: 1,
                },
            );
        })
        .unwrap();

    let expected = Buffer::with_lines(vec!["T"]);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_tabs_should_truncate_the_last_item() {
    let backend = TestBackend::new(9, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|mut f| {
            let tabs = Tabs::default()
                .titles(&["Tab1", "Tab2"])
                .margin(Margin {
                    horizontal: 0,
                    vertical: 0,
                })
                .divider(symbols::line::VERTICAL);
            f.render_widget(
                tabs,
                Rect {
                    x: 0,
                    y: 0,
                    width: 9,
                    height: 1,
                },
            );
        })
        .unwrap();

    let expected = Buffer::with_lines(vec![format!("Tab1 {} Ta", symbols::line::VERTICAL)]);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_tabs_should_not_panic_on_narrow_areas_with_margin() {
    let backend = TestBackend::new(3, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|mut f| {
            let tabs = Tabs::default()
                .titles(&["Tab1", "Tab2"])
                .margin(Margin {
                    horizontal: 3,
                    vertical: 0,
                })
                .divider(symbols::line::VERTICAL);
            f.render_widget(
                tabs,
                Rect {
                    x: 0,
                    y: 0,
                    width: 3,
                    height: 1,
                },
            );
        })
        .unwrap();

    let expected = Buffer::with_lines(vec!["   "]);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_tabs_should_respect_left_margin() {
    let test_case = |margin, width, expected_buf| {
        let backend = TestBackend::new(width, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|mut f| {
                let tabs = Tabs::default()
                    .titles(&["Tab1", "Tab2"])
                    .margin(Margin {
                        horizontal: margin,
                        vertical: 0,
                    })
                    .divider(symbols::line::VERTICAL);
                f.render_widget(
                    tabs,
                    Rect {
                        x: 0,
                        y: 0,
                        width,
                        height: 1,
                    },
                );
            })
            .unwrap();

        let expected = Buffer::with_lines(vec![expected_buf]);
        terminal.backend().assert_buffer(&expected);
    };

    test_case(0, 11, format!("Tab1 {} Tab2", symbols::line::VERTICAL));
    test_case(1, 13, format!(" Tab1 {} Tab2 ", symbols::line::VERTICAL));
    test_case(2, 15, format!("  Tab1 {} Tab2  ", symbols::line::VERTICAL));
    test_case(3, 3, "   ".to_string());
}

/// Tests that truncation occurs in order to respect the right margin
/// when space would otherwise normally be available.
#[test]
fn widgets_tabs_should_respect_right_margin() {
    // TODO: Possibly test other number of margins
    let test_case = |margin, width, expected_buf| {
        let backend = TestBackend::new(width, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|mut f| {
                let tabs = Tabs::default()
                    .titles(&["Tab1", "Tab2"])
                    .margin(Margin {
                        horizontal: margin,
                        vertical: 0,
                    })
                    .divider(symbols::line::VERTICAL);
                f.render_widget(
                    tabs,
                    Rect {
                        x: 0,
                        y: 0,
                        width,
                        height: 1,
                    },
                );
            })
            .unwrap();

        let expected = Buffer::with_lines(vec![expected_buf]);
        terminal.backend().assert_buffer(&expected);
    };

    test_case(1, 12, format!(" Tab1 {} Tab ", symbols::line::VERTICAL));
    test_case(2, 13, format!("  Tab1 {} Ta  ", symbols::line::VERTICAL));
    test_case(2, 10, format!("  Tab1 {}  ", symbols::line::VERTICAL));
    test_case(3, 10, "   Tab1   ".to_string());
    test_case(3, 6, "      ".to_string());
}

#[test]
fn widgets_tabs_should_respect_vertical_margin() {
    let test_case = |margin, height, expected_buf: Vec<String>| {
        let backend = TestBackend::new(11, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|mut f| {
                let tabs = Tabs::default()
                    .titles(&["Tab1", "Tab2"])
                    .margin(Margin {
                        horizontal: 0,
                        vertical: margin,
                    })
                    .divider(symbols::line::VERTICAL);

                f.render_widget(
                    tabs,
                    Rect {
                        x: 0,
                        y: 0,
                        width: 11,
                        height,
                    },
                );
            })
            .unwrap();

        let expected = Buffer::with_lines(expected_buf);
        terminal.backend().assert_buffer(&expected);
    };

    test_case(0, 1, vec![format!("Tab1 {} Tab2", symbols::line::VERTICAL)]);
    test_case(
        1,
        3,
        vec![
            " ".repeat(11),
            format!("Tab1 {} Tab2", symbols::line::VERTICAL),
            " ".repeat(11),
        ],
    );
    test_case(
        2,
        5,
        vec![
            " ".repeat(11),
            " ".repeat(11),
            format!("Tab1 {} Tab2", symbols::line::VERTICAL),
            " ".repeat(11),
            " ".repeat(11),
        ],
    );
    test_case(1, 2, vec![" ".repeat(11), " ".repeat(11)]);
    test_case(
        2,
        4,
        vec![
            " ".repeat(11),
            " ".repeat(11),
            " ".repeat(11),
            " ".repeat(11),
        ],
    );
    test_case(2, 3, vec![" ".repeat(11), " ".repeat(11), " ".repeat(11)]);
}
