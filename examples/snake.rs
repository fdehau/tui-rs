#[allow(dead_code)]
mod util;

use crate::util::event::{Config, Event, Events};
use rand::Rng;
use std::{error::Error, io, time::Duration};
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Paragraph,
    },
    Terminal,
};

struct Snake {
    set_direction: bool,
    tail: Vec<Rectangle>,
    next_x: f64,
    next_y: f64,
}

struct Game {
    score: u32,
    apple: Rectangle,
    snake: Snake,
}

impl Game {
    fn new() -> Game {
        let mut tail = Vec::new();

        tail.push(Rectangle {
            x: 40.0,
            y: 40.0,
            width: 5.0,
            height: 5.0,
            color: Color::Green,
            fill: false,
        });

        Game {
            score: 0,
            apple: Rectangle {
                x: 50.0,
                y: 50.0,
                width: 5.0,
                height: 5.0,
                color: Color::Yellow,
                fill: true,
            },
            snake: Snake {
                set_direction: false,
                tail: tail,
                next_x: 0.0,
                next_y: 0.0,
            },
        }
    }

    fn update(&mut self) -> bool {
        self.snake.tail[0].x += self.snake.next_x;
        self.snake.tail[0].y += self.snake.next_y;

        let snake: Vec<Rectangle> = self.snake.tail.clone();

        for i in 0..self.snake.tail.len() {
            //Move snake tail
            if i != 0 {
                self.snake.tail[i].x = snake[i - 1].x;
                self.snake.tail[i].y = snake[i - 1].y;

                //snake bites it's tail?
                if snake[0].x == snake[i].x && snake[0].y == snake[i].y {
                    return false;
                }
            }

            //Handle when the snake goes out of the playground
            if self.snake.tail[i].x > 95.0 {
                self.snake.tail[i].x = 5.0
            } else if self.snake.tail[i].x < 10.0 {
                self.snake.tail[i].x = 95.0
            } else if self.snake.tail[i].y < 10.0 {
                self.snake.tail[i].y = 95.0
            } else if self.snake.tail[i].y > 95.0 {
                self.snake.tail[i].y = 5.0
            }
        }

        //The snake bites the apple?
        if self.snake.tail[0].x == self.apple.x && self.snake.tail[0].y == self.apple.y {
            self.snake.tail.push(Rectangle {
                x: self.snake.tail[snake.len() - 1].x,
                y: self.snake.tail[snake.len() - 1].y,
                width: 5.0,
                height: 5.0,
                color: Color::Green,
                fill: true,
            });

            let mut rng = rand::thread_rng();

            let apple_x: f64 = rng.gen_range(2.0, 19.0);
            let apple_y: f64 = rng.gen_range(2.0, 19.0);

            self.apple.x = apple_x.round() * 5.0;
            self.apple.y = apple_y.round() * 5.0;

            self.score += 100;
        };

        self.snake.set_direction = false;
        true
    }
}

fn main() {
    let snake = snake_game();

    println!("Awesome score: {}", snake.unwrap());
}

fn snake_game() -> Result<u32, Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    //Setup event handler
    let config = Config {
        tick_rate: Duration::from_millis(250),
        ..Default::default()
    };
    let events = Events::with_config(config);

    //Initialize the game
    let mut game = Game::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                .split(f.size());

            let score_bar = Paragraph::new(format!("{}", &game.score))
                .block(Block::default().borders(Borders::ALL).title("Score"));

            f.render_widget(score_bar, chunks[0]);

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("The Snake Game"),
                )
                .paint(|ctx| {
                    ctx.draw(&game.apple);

                    for i in 0..game.snake.tail.len() {
                        ctx.draw(&game.snake.tail[i])
                    }
                })
                .x_bounds([10.0, 100.0])
                .y_bounds([10.0, 100.0]);

            f.render_widget(canvas, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                //Handle snake movemente and prevent turn direction
                Key::Up | Key::Char('w') => {
                    if game.snake.next_y != -5.0 && !game.snake.set_direction {
                        game.snake.set_direction = true;
                        game.snake.next_x = 0.0;
                        game.snake.next_y = 5.0;
                    }
                }
                Key::Right | Key::Char('d') => {
                    if game.snake.next_x != -5.0 && !game.snake.set_direction {
                        game.snake.set_direction = true;
                        game.snake.next_x = 5.0;
                        game.snake.next_y = 0.0;
                    }
                }
                Key::Left | Key::Char('a') => {
                    if game.snake.next_x != 5.0 && !game.snake.set_direction {
                        game.snake.set_direction = true;
                        game.snake.next_x = -5.0;
                        game.snake.next_y = 0.0;
                    }
                }
                Key::Down | Key::Char('s') => {
                    if game.snake.next_y != 5.0 && !game.snake.set_direction {
                        game.snake.set_direction = true;
                        game.snake.next_x = 0.0;
                        game.snake.next_y = -5.0;
                    }
                }
                _ => {}
            },
            Event::Tick => {
                if !game.update() {
                    break;
                }
            }
        }
    }

    Ok(game.score)
}
