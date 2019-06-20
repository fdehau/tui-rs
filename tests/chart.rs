use tui::backend::TestBackend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Marker, Widget};
use tui::Terminal;

#[test]
fn zero_axes_ok() {
    let backend = TestBackend::new(100, 100);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|mut f| {
        Chart::default()
            .block(Block::default().title("Plot").borders(Borders::ALL))
            .x_axis(
                Axis::default()
                    .bounds([ 0.0, 0.0, ])
                    .labels(&["0.0", "1.0"])
            )
            .y_axis(
                Axis::default()
                    .bounds([ 0.0, 1.0, ])
                    .labels(&["0.0", "1.0"])
            )
            .datasets(&[Dataset::default()
                .marker(Marker::Braille)
                .style(Style::default().fg(Color::Magenta))
                .data(&[(0.0, 0.0)])])
            .render(&mut f, Rect {
                x: 0,
                y: 0,
                width: 100,
                height: 100,
            });
        }).unwrap();
}
