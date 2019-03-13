#[allow(dead_code)]
mod util;

use std::io;
use std::time::Duration;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout};
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Points};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

use crate::util::event::{Config, Event, Events};
use image::RgbImage;
use std::collections::HashMap;
use std::path::Path;

pub fn open<P>(path: P) -> RgbImage
where
    P: AsRef<Path>,
{
    let img = image::open(path).unwrap();
    img.to_rgb()
}

pub fn group_by_color(img: RgbImage) -> HashMap<(u8, u8, u8), Vec<(f64, f64)>> {
    let mut result = HashMap::<(u8, u8, u8), Vec<(f64, f64)>>::new();
    let (_, height) = img.dimensions();
    let height = height as i32;
    for (x, y, color) in img.enumerate_pixels() {
        let x = f64::from(x);
        let y = f64::from(height - 1 - (y as i32));
        let key = (color.data[0], color.data[1], color.data[2]);
        if let Some(origin_value) = result.get(&key) {
            let mut value = origin_value.clone();
            value.push((x, y));
            result.insert(key, value);
        } else {
            let mut value = Vec::<(f64, f64)>::new();
            value.push((x, y));
            result.insert(key, value);
        }
    }
    result
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
        tick_rate: Duration::from_millis(1000),
        ..Default::default()
    };
    let events = Events::with_config(config);

    let img = open("assets/Hummingbird_by_Shu_Le.png");
    let width = img.width();
    let height = img.height();
    let img_data = group_by_color(img);

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            Canvas::default()
                .block(
                    Block::default()
                        .title(format!("{}x{}", width, height).as_str())
                        .borders(Borders::NONE),
                )
                .x_bounds([0.0, (width - 1) as f64])
                .y_bounds([0.0, (height - 1) as f64])
                .paint(|ctx| {
                    for color in img_data.keys() {
                        if let Some(points) = img_data.get(&color) {
                            ctx.draw(&Points {
                                coords: points,
                                color: Color::Rgb(color.0, color.1, color.2),
                            })
                        }
                    }
                })
                .render(&mut f, chunks[0]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }

                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
