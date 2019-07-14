use itui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph, Text, Widget},
    Terminal,
};

const SAMPLE_STRING: &str =
    "The library is based on the principle of immediate rendering with \
     intermediate buffers. This means that at each new frame you should build all widgets that are \
     supposed to be part of the UI. While providing a great flexibility for rich and \
     interactive UI, this may introduce overhead for highly dynamic content.";

#[test]
fn paragraph_render_wrap() {
    let render = |alignment| {
        let backend = TestBackend::new(20, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let text = [Text::raw(SAMPLE_STRING)];
                Paragraph::new(text.iter())
                    .block(Block::default().borders(Borders::ALL).area(size))
                    .alignment(alignment)
                    .wrap(true)
                    .area(size)
                    .render(&mut f);
            })
            .unwrap();
        terminal.backend().buffer().clone()
    };

    assert_eq!(
        render(Alignment::Left),
        Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│The library is    │",
            "│based on the      │",
            "│principle of      │",
            "│immediate         │",
            "│rendering with    │",
            "│intermediate      │",
            "│buffers. This     │",
            "│means that at each│",
            "└──────────────────┘",
        ])
    );
    assert_eq!(
        render(Alignment::Right),
        Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│    The library is│",
            "│      based on the│",
            "│      principle of│",
            "│         immediate│",
            "│    rendering with│",
            "│      intermediate│",
            "│     buffers. This│",
            "│means that at each│",
            "└──────────────────┘",
        ])
    );
    assert_eq!(
        render(Alignment::Center),
        Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│  The library is  │",
            "│   based on the   │",
            "│   principle of   │",
            "│     immediate    │",
            "│  rendering with  │",
            "│   intermediate   │",
            "│   buffers. This  │",
            "│means that at each│",
            "└──────────────────┘",
        ])
    );
}

#[test]
fn paragraph_render_double_width() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let s =
        "コンピュータ上で文字を扱う場合、典型的には文字による通信を行う場合にその両端点では、";
    terminal
        .draw(|mut f| {
            let size = f.size();
            let text = [Text::raw(s)];
            Paragraph::new(text.iter())
                .block(Block::default().borders(Borders::ALL).area(size))
                .wrap(true)
                .area(size)
                .render(&mut f);
        })
        .unwrap();

    let expected = Buffer::with_lines(vec![
        "┌────────┐",
        "│コンピュ│",
        "│ータ上で│",
        "│文字を扱│",
        "│う場合、│",
        "│典型的に│",
        "│は文字に│",
        "│よる通信│",
        "│を行う場│",
        "└────────┘",
    ]);
    assert_eq!(&expected, terminal.backend().buffer());
}

#[test]
fn paragraph_render_mixed_width() {
    let backend = TestBackend::new(10, 7);
    let mut terminal = Terminal::new(backend).unwrap();

    let s = "aコンピュータ上で文字を扱う場合、";
    terminal
        .draw(|mut f| {
            let size = f.size();
            let text = [Text::raw(s)];
            Paragraph::new(text.iter())
                .block(Block::default().borders(Borders::ALL).area(size))
                .wrap(true)
                .area(size)
                .render(&mut f);
        })
        .unwrap();

    let expected = Buffer::with_lines(vec![
        // The internal width is 8 so only 4 slots for double-width characters.
        "┌────────┐",
        "│aコンピ │", // Here we have 1 latin character so only 3 double-width ones can fit.
        "│ュータ上│",
        "│で文字を│",
        "│扱う場合│",
        "│、      │",
        "└────────┘",
    ]);
    assert_eq!(&expected, terminal.backend().buffer());
}
