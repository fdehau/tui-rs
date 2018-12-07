extern crate failure;
extern crate termion;
extern crate tui;

#[allow(dead_code)]
mod util;

use std::io;
use std::time::Duration;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

use util::event::{Config, Event, Events};

struct App {
    x: f64,
    y: f64,
    ball: Rect,
    playground: Rect,
    vx: u16,
    vy: u16,
    dir_x: bool,
    dir_y: bool,
}

impl App {
    fn new() -> App {
        App {
            x: 0.0,
            y: 0.0,
            ball: Rect::new(10, 30, 10, 10),
            playground: Rect::new(10, 10, 100, 100),
            vx: 1,
            vy: 1,
            dir_x: true,
            dir_y: true,
        }
    }

    fn update(&mut self) {
        if self.ball.left() < self.playground.left() || self.ball.right() > self.playground.right()
        {
            self.dir_x = !self.dir_x;
        }
        if self.ball.top() < self.playground.top() || self.ball.bottom() > self.playground.bottom()
        {
            self.dir_y = !self.dir_y;
        }

        if self.dir_x {
            self.ball.x += self.vx;
        } else {
            self.ball.x -= self.vx;
        }

        if self.dir_y {
            self.ball.y += self.vy;
        } else {
            self.ball.y -= self.vy
        }
    }
}

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup event handlers
    let config = Config {
        tick_rate: Duration::from_millis(100),
        ..Default::default()
    };
    let events = Events::with_config(config);

    // App
    let mut app = App::new();

    loop {
        let size = terminal.size()?;

        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);
            Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("World"))
                .paint(|ctx| {
                    ctx.draw(&Map {
                        color: Color::White,
                        resolution: MapResolution::High,
                    });
                    ctx.print(app.x, -app.y, "You are here", Color::Yellow);
                })
                .x_bounds([-180.0, 180.0])
                .y_bounds([-90.0, 90.0])
                .render(&mut f, chunks[0]);
            Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Pong"))
                .paint(|ctx| {
                    ctx.draw(&Line {
                        x1: f64::from(app.ball.left()),
                        y1: f64::from(app.ball.top()),
                        x2: f64::from(app.ball.right()),
                        y2: f64::from(app.ball.top()),
                        color: Color::Yellow,
                    });
                    ctx.draw(&Line {
                        x1: f64::from(app.ball.right()),
                        y1: f64::from(app.ball.top()),
                        x2: f64::from(app.ball.right()),
                        y2: f64::from(app.ball.bottom()),
                        color: Color::Yellow,
                    });
                    ctx.draw(&Line {
                        x1: f64::from(app.ball.right()),
                        y1: f64::from(app.ball.bottom()),
                        x2: f64::from(app.ball.left()),
                        y2: f64::from(app.ball.bottom()),
                        color: Color::Yellow,
                    });
                    ctx.draw(&Line {
                        x1: f64::from(app.ball.left()),
                        y1: f64::from(app.ball.bottom()),
                        x2: f64::from(app.ball.left()),
                        y2: f64::from(app.ball.top()),
                        color: Color::Yellow,
                    });
                })
                .x_bounds([10.0, 110.0])
                .y_bounds([10.0, 110.0])
                .render(&mut f, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Down => {
                    app.y += 1.0;
                }
                Key::Up => {
                    app.y -= 1.0;
                }
                Key::Right => {
                    app.x += 1.0;
                }
                Key::Left => {
                    app.x -= 1.0;
                }

                _ => {}
            },
            Event::Tick => {
                app.update();
            }
        }
    }

    Ok(())
}
