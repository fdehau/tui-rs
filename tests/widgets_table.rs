use tui::backend::TestBackend;
use tui::buffer::Buffer;
use tui::layout::Constraint;
use tui::style::{Color, Style, StyleDiff};
use tui::widgets::{Block, Borders, Row, Table, TableState};
use tui::Terminal;

#[test]
fn widgets_table_column_spacing_can_be_changed() {
    let test_case = |column_spacing, expected| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
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
        terminal.backend().assert_buffer(&expected);
    };

    // no space between columns
    test_case(
        0,
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
        ]),
    );

    // one space between columns
    test_case(
        1,
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
        ]),
    );

    // enough space to just not hide the third column
    test_case(
        6,
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
        ]),
    );

    // enough space to hide part of the third column
    test_case(
        7,
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
        ]),
    );
}

#[test]
fn widgets_table_columns_widths_can_use_fixed_length_constraints() {
    let test_case = |widths, expected| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
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
        terminal.backend().assert_buffer(&expected);
    };

    // columns of zero width show nothing
    test_case(
        &[
            Constraint::Length(0),
            Constraint::Length(0),
            Constraint::Length(0),
        ],
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
        ]),
    );

    // columns of 1 width trim
    test_case(
        &[
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ],
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
        ]),
    );

    // columns of large width just before pushing a column off
    test_case(
        &[
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
        ],
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
        ]),
    );
}

#[test]
fn widgets_table_columns_widths_can_use_percentage_constraints() {
    let test_case = |widths, expected| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
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
        terminal.backend().assert_buffer(&expected);
    };

    // columns of zero width show nothing
    test_case(
        &[
            Constraint::Percentage(0),
            Constraint::Percentage(0),
            Constraint::Percentage(0),
        ],
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
        ]),
    );

    // columns of not enough width trims the data
    test_case(
        &[
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ],
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
        ]),
    );

    // columns of large width just before pushing a column off
    test_case(
        &[
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ],
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
        ]),
    );

    // percentages summing to 100 should give equal widths
    test_case(
        &[Constraint::Percentage(50), Constraint::Percentage(50)],
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
        ]),
    );
}

#[test]
fn widgets_table_columns_widths_can_use_mixed_constraints() {
    let test_case = |widths, expected| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
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
        terminal.backend().assert_buffer(&expected);
    };

    // columns of zero width show nothing
    test_case(
        &[
            Constraint::Percentage(0),
            Constraint::Length(0),
            Constraint::Percentage(0),
        ],
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
        ]),
    );

    // columns of not enough width trims the data
    test_case(
        &[
            Constraint::Percentage(10),
            Constraint::Length(20),
            Constraint::Percentage(10),
        ],
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
        ]),
    );

    // columns of large width just before pushing a column off
    test_case(
        &[
            Constraint::Percentage(30),
            Constraint::Length(10),
            Constraint::Percentage(30),
        ],
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
        ]),
    );

    // columns of large size (>100% total) hide the last column
    test_case(
        &[
            Constraint::Percentage(60),
            Constraint::Length(10),
            Constraint::Percentage(60),
        ],
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
        ]),
    );
}

#[test]
fn widgets_table_can_be_styled() {
    let test_case = |bg: Buffer, fg: Buffer, style, row_style, highlight_style, header_style| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let table = Table::new(
                    ["Head1", "Head2", "Head3"].iter(),
                    vec![
                        Row::Data(["Row11", "Row12", "Row13"].iter()),
                        Row::StyledData(["Row21", "Row22", "Row23"].iter(), row_style),
                        Row::StyledData(["Row31", "Row32", "Row33"].iter(), row_style),
                        Row::Data(["Row41", "Row42", "Row43"].iter()),
                    ]
                    .into_iter(),
                )
                .block(Block::default().borders(Borders::ALL))
                .highlight_symbol(">> ")
                .style(style)
                .highlight_style_diff(highlight_style)
                .header_style_diff(header_style)
                .widths(&[
                    Constraint::Length(9),
                    Constraint::Length(6),
                    Constraint::Length(6),
                ]);

                let mut state = TableState::default();
                state.select(Some(1));
                f.render_stateful_widget(table, size, &mut state);
            })
            .unwrap();

        let mut expected = Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1     Head2  Head3      │",
            "│                            │",
            "│   Row11  Row12  Row13      │",
            "│>> Row21  Row22  Row23      │",
            "│   Row31  Row32  Row33      │",
            "│   Row41  Row42  Row43      │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]);
        for x in 0..30 {
            for y in 0..10 {
                match bg.get(x, y).symbol.as_str() {
                    "B" | "b" => {
                        expected.get_mut(x, y).set_bg(Color::Blue);
                    }
                    "R" | "r" => {
                        expected.get_mut(x, y).set_bg(Color::Red);
                    }
                    "G" | "g" => {
                        expected.get_mut(x, y).set_bg(Color::Green);
                    }
                    "Y" | "y" => {
                        expected.get_mut(x, y).set_bg(Color::Yellow);
                    }
                    _ => (),
                };
                match fg.get(x, y).symbol.as_str() {
                    "B" | "b" => {
                        expected.get_mut(x, y).set_fg(Color::Blue);
                    }
                    "R" | "r" => {
                        expected.get_mut(x, y).set_fg(Color::Red);
                    }
                    "G" | "g" => {
                        expected.get_mut(x, y).set_fg(Color::Green);
                    }
                    "Y" | "y" => {
                        expected.get_mut(x, y).set_fg(Color::Yellow);
                    }
                    _ => (),
                };
            }
        }

        terminal.backend().assert_buffer(&expected);
    };

    test_case(
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
        ]),
        Style::default(),
        Style::default(),
        Style::default().into(),
        Style::default().into(),
    );

    test_case(
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│YYYYYrrrrrYYYYYrrYYYYYrrrrrr│",
            "│rrrrrrrrrrrrrrrrrrrrrrrrrrrr│",
            "│rrrRRRRRrrRRRRRrrRRRRRrrrrrr│",
            "│GGGGGGGGrrGGGGGrrGGGGGrrrrrr│",
            "│rrrBBBBBrrBBBBBrrBBBBBrrrrrr│",
            "│rrrRRRRRrrRRRRRrrRRRRRrrrrrr│",
            "│rrrrrrrrrrrrrrrrrrrrrrrrrrrr│",
            "│rrrrrrrrrrrrrrrrrrrrrrrrrrrr│",
            "└────────────────────────────┘",
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│YYYYY     YYYYY  YYYYY      │",
            "│                            │",
            "│rrrRRRRR  RRRRR  RRRRR      │",
            "│GGGGGGGG  GGGGG  GGGGG      │",
            "│rrrBBBBB  BBBBB  BBBBB      │",
            "│rrrRRRRR  RRRRR  RRRRR      │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
        Style::default().fg(Color::Red).bg(Color::Red),
        Style::default().fg(Color::Blue).bg(Color::Blue),
        Style::default().fg(Color::Green).bg(Color::Green).into(),
        Style::default().fg(Color::Yellow).bg(Color::Yellow).into(),
    );

    test_case(
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│YYYYYrrrrrYYYYYrrYYYYYrrrrrr│",
            "│rrrrrrrrrrrrrrrrrrrrrrrrrrrr│",
            "│rrrRRRRRrrRRRRRrrRRRRRrrrrrr│",
            "│--------rr-----rr-----rrrrrr│",
            "│rrrBBBBBrrBBBBBrrBBBBBrrrrrr│",
            "│rrrRRRRRrrRRRRRrrRRRRRrrrrrr│",
            "│rrrrrrrrrrrrrrrrrrrrrrrrrrrr│",
            "│rrrrrrrrrrrrrrrrrrrrrrrrrrrr│",
            "└────────────────────────────┘",
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│YYYYY     YYYYY  YYYYY      │",
            "│                            │",
            "│rrrRRRRR  RRRRR  RRRRR      │",
            "│--------  -----  -----      │",
            "│rrrBBBBB  BBBBB  BBBBB      │",
            "│rrrRRRRR  RRRRR  RRRRR      │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
        Style::default().fg(Color::Red).bg(Color::Red),
        Style::default().fg(Color::Blue).bg(Color::Blue),
        Style::default().into(),
        Style::default().fg(Color::Yellow).bg(Color::Yellow).into(),
    );

    test_case(
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│RRRRRrrrrrRRRRRrrRRRRRrrrrrr│",
            "│rrrrrrrrrrrrrrrrrrrrrrrrrrrr│",
            "│rrrRRRRRrrRRRRRrrRRRRRrrrrrr│",
            "│GGGGGGGGrrGGGGGrrGGGGGrrrrrr│",
            "│rrrBBBBBrrBBBBBrrBBBBBrrrrrr│",
            "│rrrRRRRRrrRRRRRrrRRRRRrrrrrr│",
            "│rrrrrrrrrrrrrrrrrrrrrrrrrrrr│",
            "│rrrrrrrrrrrrrrrrrrrrrrrrrrrr│",
            "└────────────────────────────┘",
        ]),
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│YYYYY     YYYYY  YYYYY      │",
            "│                            │",
            "│rrrRRRRR  RRRRR  RRRRR      │",
            "│RRRBBBBB  BBBBB  BBBBB      │",
            "│rrrBBBBB  BBBBB  BBBBB      │",
            "│rrrRRRRR  RRRRR  RRRRR      │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
        Style::default().fg(Color::Red).bg(Color::Red),
        Style::default().fg(Color::Blue).bg(Color::Blue),
        StyleDiff::default().bg(Color::Green),
        StyleDiff::default().fg(Color::Yellow),
    );
}
