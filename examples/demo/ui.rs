use std::io;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{
    Block, Borders,List, Paragraph, Row,
    SelectableList, Table, Tabs, Text, Widget,
};
use tui::{Frame, Terminal};

use crate::demo::App;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &App) -> Result<(), io::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.size());
        Tabs::default()
            .block(Block::default().borders(Borders::ALL).title(app.title))
            .titles(&app.tabs.titles)
            .style(Style::default().fg(Color::Green))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(app.tabs.index)
            .area(chunks[0])
            .render(&mut f);
        match app.tabs.index {
            0 => draw_first_tab(&mut f, &app, chunks[1]),
            1 => draw_second_tab(&mut f, &app, chunks[1]),
            _ => {}
        };
    })
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(7),
                Constraint::Min(7),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    draw_gauges(f, app, chunks[0]);
    draw_charts(f, app, chunks[1]);
    draw_text(f, chunks[2]);
}

fn draw_gauges<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(2), Constraint::Length(3)].as_ref())
        .margin(1)
        .split(area);
    Block::default()
        .borders(Borders::ALL)
        .title("Graphs")
        .area(area)
        .render(f);
}

fn draw_charts<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let constraints = if app.show_chart {
        vec![Constraint::Percentage(50), Constraint::Percentage(50)]
    } else {
        vec![Constraint::Percentage(100)]
    };
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);
    {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);
        {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[0]);
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("List"))
                .items(&app.tasks.items)
                .select(Some(app.tasks.selected))
                .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
                .highlight_symbol(">")
                .render(f);
            let info_style = Style::default().fg(Color::White);
            let warning_style = Style::default().fg(Color::Yellow);
            let error_style = Style::default().fg(Color::Magenta);
            let critical_style = Style::default().fg(Color::Red);
            let events = app.logs.items.iter().map(|&(evt, level)| {
                Text::styled(
                    format!("{}: {}", level, evt),
                    match level {
                        "ERROR" => error_style,
                        "CRITICAL" => critical_style,
                        "WARNING" => warning_style,
                        _ => info_style,
                    },
                )
            });
            List::new(events)
                .block(Block::default().borders(Borders::ALL).title("List").area(chunks[1]))
                .render(f);
        }
    }
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = [
        Text::raw("This is a paragraph with several lines. You can change style your text the way you want.\n\nFox example: "),
        Text::styled("under", Style::default().fg(Color::Red)),
        Text::raw(" "),
        Text::styled("the", Style::default().fg(Color::Green)),
        Text::raw(" "),
        Text::styled("rainbow", Style::default().fg(Color::Blue)),
        Text::raw(".\nOh and if you didn't "),
        Text::styled("notice", Style::default().modifier(Modifier::ITALIC)),
        Text::raw(" you can "),
        Text::styled("automatically", Style::default().modifier(Modifier::BOLD)),
        Text::raw(" "),
        Text::styled("wrap", Style::default().modifier(Modifier::REVERSED)),
        Text::raw(" your "),
        Text::styled("text", Style::default().modifier(Modifier::UNDERLINED)),
        Text::raw(".\nOne more thing is that it should display unicode characters: 10€")
    ];
    Paragraph::new(text.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Footer")
                .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD))
                .area(area)
        )
        .wrap(true)
        .render(f);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);
    let up_style = Style::default().fg(Color::Green);
    let failure_style = Style::default()
        .fg(Color::Red)
        .modifier(Modifier::RAPID_BLINK | Modifier::CROSSED_OUT);
    let header = ["Server", "Location", "Status"];
    let rows = app.servers.iter().map(|s| {
        let style = if s.status == "Up" {
            up_style
        } else {
            failure_style
        };
        Row::StyledData(vec![s.name, s.location, s.status].into_iter(), style)
    });
    Table::new(header.into_iter(), rows)
        .block(Block::default().title("Servers").borders(Borders::ALL))
        .header_style(Style::default().fg(Color::Yellow))
        .widths(&[15, 15, 10])
        .area(chunks[0])
        .render(f);

}
