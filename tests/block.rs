use itui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Terminal,
};

#[test]
fn it_draws_a_block() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|mut f| {
            Block::default()
                .title("Title")
                .borders(Borders::ALL)
                .title_style(Style::default().fg(Color::LightBlue))
                .area(Rect {
                    x: 0,
                    y: 0,
                    width: 8,
                    height: 8,
                })
                .render(&mut f);
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
    assert_eq!(&expected, terminal.backend().buffer());
}
