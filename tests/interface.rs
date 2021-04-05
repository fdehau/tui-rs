use tui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders},
    Terminal,
};

struct Interface {
    left: Block<'static>,
    right: Block<'static>,
    up: Block<'static>,
    down: Block<'static>,
}

#[test]
fn interface() {
    let mut interface = Interface {
        left: Block::default().title("_Left").borders(Borders::ALL),
        right: Block::default().title("Right").borders(Borders::ALL),
        up: Block::default().title("Up").borders(Borders::ALL),
        down: Block::default().title("Down").borders(Borders::ALL),
    };
    for index in 0..5 {
        interface.left.retitle(format!("{}Left", index));
        let backend = TestBackend::new(24, 6);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                f.render(
                    &mut interface.left,
                    Rect {
                        x: 0,
                        y: 0,
                        width: 8,
                        height: 6,
                    },
                );
                f.render(
                    &mut interface.up,
                    Rect {
                        x: 8,
                        y: 0,
                        width: 8,
                        height: 3,
                    },
                );
                f.render(
                    &mut interface.down,
                    Rect {
                        x: 8,
                        y: 3,
                        width: 8,
                        height: 3,
                    },
                );
                f.render(
                    &mut interface.right,
                    Rect {
                        x: 16,
                        y: 0,
                        width: 8,
                        height: 6,
                    },
                );
            })
            .unwrap();
        let mut expected = Buffer::with_lines(vec![
            "┌_Left─┐┌Up────┐┌Right─┐",
            "│      ││      ││      │",
            "│      │└──────┘│      │",
            "│      │┌Down──┐│      │",
            "│      ││      ││      │",
            "└──────┘└──────┘└──────┘",
        ]);
        expected.get_mut(1, 0).symbol = format!("{}", index);
        terminal.backend().assert_buffer(&expected);
    }
}
