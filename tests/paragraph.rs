extern crate failure;
extern crate termion;
extern crate tui;

use tui::backend::TestBackend;
use tui::buffer::Buffer;
use tui::layout::Alignment;
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;

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
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(alignment)
                    .wrap(true)
                    .render(&mut f, size);
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

    let s = "コンピュータ上で文字を扱う場合、典型的には文字による通信を行う場合にその両端点では、";
    terminal
        .draw(|mut f| {
            let size = f.size();
            let text = [Text::raw(s)];
            Paragraph::new(text.iter())
                .block(Block::default().borders(Borders::ALL))
                .wrap(true)
                .render(&mut f, size);
        })
        .unwrap();

    let expected = Buffer::with_lines(vec![
        // This is "OK" - these are double-width characters. In terminal each occupies 2 spaces,
        // which means the buffer contains a Cell with a full grapheme in it, followed by a vacant
        // one. Here however, we have plain text, so each character is visibly followed by a space.
        "┌────────┐",
        "│コ ン ピ ュ │",
        "│ー タ 上 で │",
        "│文 字 を 扱 │",
        "│う 場 合 、 │",
        "│典 型 的 に │",
        "│は 文 字 に │",
        "│よ る 通 信 │",
        "│を 行 う 場 │",
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
                .block(Block::default().borders(Borders::ALL))
                .wrap(true)
                .render(&mut f, size);
        })
        .unwrap();

    let expected = Buffer::with_lines(vec![
        // The internal width is 8 so only 4 slots for double-width characters.
        "┌────────┐",
        "│aコ ン ピ  │", // Here we have 1 latin character so only 3 double-width ones can fit.
        "│ュ ー タ 上 │",
        "│で 文 字 を │",
        "│扱 う 場 合 │",
        "│、       │",
        "└────────┘",
    ]);
    assert_eq!(&expected, terminal.backend().buffer());
}
