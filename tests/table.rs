use tui::backend::TestBackend;
use tui::buffer::Buffer;
use tui::layout::Constraint;
use tui::widgets::{Block, Borders, Row, Table};
use tui::Terminal;

#[test]
fn table_column_spacing() {
    let render = |column_spacing| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let table = Table::new(
                    ["Head1", "Head2", "Head3"].iter(),
                    vec![
                        Row::Data(["Row11", "Row12", "Row13"].iter()),
                        Row::Data(["Row21", "Row22", "Row23"].iter()),
                        Row::Data(["Row31", "Row32", "Row33"].iter()),
                        Row::Data(["Row41", "Row42", "Row43"].iter()),
                    ]
                    .into_iter(),
                )
                .block(Block::default().borders(Borders::ALL))
                .widths(&[
                    Constraint::Length(5),
                    Constraint::Length(5),
                    Constraint::Length(5),
                ])
                .column_spacing(column_spacing);
                f.render_widget(table, size);
            })
            .unwrap();
        terminal.backend().buffer().clone()
    };

    // no space between columns
    assert_eq!(
        render(0),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1Head2Head3             │",
            "│                            │",
            "│Row11Row12Row13             │",
            "│Row21Row22Row23             │",
            "│Row31Row32Row33             │",
            "│Row41Row42Row43             │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );

    // one space between columns
    assert_eq!(
        render(1),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1 Head2 Head3           │",
            "│                            │",
            "│Row11 Row12 Row13           │",
            "│Row21 Row22 Row23           │",
            "│Row31 Row32 Row33           │",
            "│Row41 Row42 Row43           │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );

    // enough space to just not hide the third column
    assert_eq!(
        render(6),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1      Head2      Head3 │",
            "│                            │",
            "│Row11      Row12      Row13 │",
            "│Row21      Row22      Row23 │",
            "│Row31      Row32      Row33 │",
            "│Row41      Row42      Row43 │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );

    // enough space to hide part of the third column
    assert_eq!(
        render(7),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1       Head2       Head│",
            "│                            │",
            "│Row11       Row12       Row1│",
            "│Row21       Row22       Row2│",
            "│Row31       Row32       Row3│",
            "│Row41       Row42       Row4│",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );
}

#[test]
fn table_widths() {
    let render = |widths| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let table = Table::new(
                    ["Head1", "Head2", "Head3"].iter(),
                    vec![
                        Row::Data(["Row11", "Row12", "Row13"].iter()),
                        Row::Data(["Row21", "Row22", "Row23"].iter()),
                        Row::Data(["Row31", "Row32", "Row33"].iter()),
                        Row::Data(["Row41", "Row42", "Row43"].iter()),
                    ]
                    .into_iter(),
                )
                .block(Block::default().borders(Borders::ALL))
                .widths(widths);
                f.render_widget(table, size);
            })
            .unwrap();
        terminal.backend().buffer().clone()
    };

    // columns of zero width show nothing
    assert_eq!(
        render(&[
            Constraint::Length(0),
            Constraint::Length(0),
            Constraint::Length(0)
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );

    // columns of 1 width trim
    assert_eq!(
        render(&[
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1)
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│H H H                       │",
            "│                            │",
            "│R R R                       │",
            "│R R R                       │",
            "│R R R                       │",
            "│R R R                       │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );

    // columns of large width just before pushing a column off
    assert_eq!(
        render(&[
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8)
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1    Head2    Head3     │",
            "│                            │",
            "│Row11    Row12    Row13     │",
            "│Row21    Row22    Row23     │",
            "│Row31    Row32    Row33     │",
            "│Row41    Row42    Row43     │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );
}

#[test]
fn table_percentage_widths() {
    let render = |widths| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let table = Table::new(
                    ["Head1", "Head2", "Head3"].iter(),
                    vec![
                        Row::Data(["Row11", "Row12", "Row13"].iter()),
                        Row::Data(["Row21", "Row22", "Row23"].iter()),
                        Row::Data(["Row31", "Row32", "Row33"].iter()),
                        Row::Data(["Row41", "Row42", "Row43"].iter()),
                    ]
                    .into_iter(),
                )
                .block(Block::default().borders(Borders::ALL))
                .widths(widths)
                .column_spacing(0);
                f.render_widget(table, size);
            })
            .unwrap();
        terminal.backend().buffer().clone()
    };

    // columns of zero width show nothing
    assert_eq!(
        render(&[
            Constraint::Percentage(0),
            Constraint::Percentage(0),
            Constraint::Percentage(0)
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );

    // columns of not enough width trims the data
    assert_eq!(
        render(&[
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10)
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│HeaHeaHea                   │",
            "│                            │",
            "│RowRowRow                   │",
            "│RowRowRow                   │",
            "│RowRowRow                   │",
            "│RowRowRow                   │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );

    // columns of large width just before pushing a column off
    assert_eq!(
        render(&[
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30)
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1    Head2    Head3     │",
            "│                            │",
            "│Row11    Row12    Row13     │",
            "│Row21    Row22    Row23     │",
            "│Row31    Row32    Row33     │",
            "│Row41    Row42    Row43     │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );

    // percentages summing to 100 should give equal widths
    assert_eq!(
        render(&[Constraint::Percentage(50), Constraint::Percentage(50)]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1          Head2        │",
            "│                            │",
            "│Row11          Row12        │",
            "│Row21          Row22        │",
            "│Row31          Row32        │",
            "│Row41          Row42        │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );
}

#[test]
fn table_mixed_widths() {
    let render = |widths| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let table = Table::new(
                    ["Head1", "Head2", "Head3"].iter(),
                    vec![
                        Row::Data(["Row11", "Row12", "Row13"].iter()),
                        Row::Data(["Row21", "Row22", "Row23"].iter()),
                        Row::Data(["Row31", "Row32", "Row33"].iter()),
                        Row::Data(["Row41", "Row42", "Row43"].iter()),
                    ]
                    .into_iter(),
                )
                .block(Block::default().borders(Borders::ALL))
                .widths(widths);
                f.render_widget(table, size);
            })
            .unwrap();
        terminal.backend().buffer().clone()
    };

    // columns of zero width show nothing
    assert_eq!(
        render(&[
            Constraint::Percentage(0),
            Constraint::Length(0),
            Constraint::Percentage(0)
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );

    // columns of not enough width trims the data
    assert_eq!(
        render(&[
            Constraint::Percentage(10),
            Constraint::Length(20),
            Constraint::Percentage(10)
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Hea Head2                Hea│",
            "│                            │",
            "│Row Row12                Row│",
            "│Row Row22                Row│",
            "│Row Row32                Row│",
            "│Row Row42                Row│",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );

    // columns of large width just before pushing a column off
    assert_eq!(
        render(&[
            Constraint::Percentage(30),
            Constraint::Length(10),
            Constraint::Percentage(30)
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1     Head2      Head3  │",
            "│                            │",
            "│Row11     Row12      Row13  │",
            "│Row21     Row22      Row23  │",
            "│Row31     Row32      Row33  │",
            "│Row41     Row42      Row43  │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );

    // columns of large size (>100% total) hide the last column
    assert_eq!(
        render(&[
            Constraint::Percentage(60),
            Constraint::Length(10),
            Constraint::Percentage(60)
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1            Head2      │",
            "│                            │",
            "│Row11            Row12      │",
            "│Row21            Row22      │",
            "│Row31            Row32      │",
            "│Row41            Row42      │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ])
    );
}
