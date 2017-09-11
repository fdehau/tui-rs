extern crate tui;
extern crate termion;

use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Widget, Block, border};
use tui::widgets::canvas::{Canvas, Map, MapResolution, Line};
use tui::layout::{Group, Rect, Direction, Size};
use tui::style::Color;

struct App {
    size: Rect,
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
            size: Default::default(),
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

    fn advance(&mut self) {
        if self.ball.left() < self.playground.left() ||
            self.ball.right() > self.playground.right()
        {
            self.dir_x = !self.dir_x;
        }
        if self.ball.top() < self.playground.top() ||
            self.ball.bottom() > self.playground.bottom()
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

enum Event {
    Input(event::Key),
    Tick,
}

fn main() {
    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();
    let clock_tx = tx.clone();

    // Input
    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    // Tick
    thread::spawn(move || loop {
        clock_tx.send(Event::Tick).unwrap();
        thread::sleep(time::Duration::from_millis(500));
    });

    // App
    let mut app = App::new();

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    draw(&mut terminal, &app);

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }

        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                match input {
                    event::Key::Char('q') => {
                        break;
                    }
                    event::Key::Down => {
                        app.y += 1.0;
                    }
                    event::Key::Up => {
                        app.y -= 1.0;
                    }
                    event::Key::Right => {
                        app.x += 1.0;
                    }
                    event::Key::Left => {
                        app.x -= 1.0;
                    }

                    _ => {}
                }
            }
            Event::Tick => {
                app.advance();
            }
        }
        draw(&mut terminal, &app);
    }

    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<MouseBackend>, app: &App) {


    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, &app.size, |t, chunks| {
            Canvas::default()
                .block(Block::default().borders(border::ALL).title("World"))
                .paint(|ctx| {
                    ctx.draw(&Map {
                        color: Color::White,
                        resolution: MapResolution::High,
                    });
                    ctx.print(app.x, -app.y, "You are here", Color::Yellow);
                })
                .x_bounds([-180.0, 180.0])
                .y_bounds([-90.0, 90.0])
                .render(t, &chunks[0]);
            Canvas::default()
                .block(Block::default().borders(border::ALL).title("List"))
                .paint(|ctx| {
                    ctx.draw(&Line {
                        x1: app.ball.left() as f64,
                        y1: app.ball.top() as f64,
                        x2: app.ball.right() as f64,
                        y2: app.ball.top() as f64,
                        color: Color::Yellow,
                    });
                    ctx.draw(&Line {
                        x1: app.ball.right() as f64,
                        y1: app.ball.top() as f64,
                        x2: app.ball.right() as f64,
                        y2: app.ball.bottom() as f64,
                        color: Color::Yellow,
                    });
                    ctx.draw(&Line {
                        x1: app.ball.right() as f64,
                        y1: app.ball.bottom() as f64,
                        x2: app.ball.left() as f64,
                        y2: app.ball.bottom() as f64,
                        color: Color::Yellow,
                    });
                    ctx.draw(&Line {
                        x1: app.ball.left() as f64,
                        y1: app.ball.bottom() as f64,
                        x2: app.ball.left() as f64,
                        y2: app.ball.top() as f64,
                        color: Color::Yellow,
                    });
                })
                .x_bounds([10.0, 110.0])
                .y_bounds([10.0, 110.0])
                .render(t, &chunks[1]);
        });

    t.draw().unwrap();
}
