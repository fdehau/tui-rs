mod line;
mod map;
mod points;
mod world;

pub use self::line::Line;
pub use self::map::{Map, MapResolution};
pub use self::points::Points;

use buffer::Buffer;
use layout::Rect;
use style::{Color, Style};
use widgets::{Block, Widget};

pub const DOTS: [[u16; 2]; 4] = [
    [0x0001, 0x0008],
    [0x0002, 0x0010],
    [0x0004, 0x0020],
    [0x0040, 0x0080],
];
pub const BRAILLE_OFFSET: u16 = 0x2800;
pub const BRAILLE_BLANK: char = '⠀';

/// Interface for all shapes that may be drawn on a Canvas widget.
pub trait Shape<'a> {
    /// Returns the color of the shape
    fn color(&self) -> Color;
    /// Returns an iterator over all points of the shape
    fn points(&'a self) -> Box<Iterator<Item = (f64, f64)> + 'a>;
}

/// Label to draw some text on the canvas
pub struct Label<'a> {
    pub x: f64,
    pub y: f64,
    pub text: &'a str,
    pub color: Color,
}

struct Layer {
    string: String,
    colors: Vec<Color>,
}

struct Grid {
    cells: Vec<u16>,
    colors: Vec<Color>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            cells: vec![BRAILLE_OFFSET; width * height],
            colors: vec![Color::Reset; width * height],
        }
    }

    fn save(&self) -> Layer {
        Layer {
            string: String::from_utf16(&self.cells).unwrap(),
            colors: self.colors.clone(),
        }
    }

    fn reset(&mut self) {
        for c in &mut self.cells {
            *c = BRAILLE_OFFSET;
        }
        for c in &mut self.colors {
            *c = Color::Reset;
        }
    }
}

/// Holds the state of the Canvas when painting to it.
pub struct Context<'a> {
    width: u16,
    height: u16,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    grid: Grid,
    dirty: bool,
    layers: Vec<Layer>,
    labels: Vec<Label<'a>>,
}

impl<'a> Context<'a> {
    /// Draw any object that may implement the Shape trait
    pub fn draw<'b, S>(&mut self, shape: &'b S)
    where
        S: Shape<'b>,
    {
        self.dirty = true;
        let left = self.x_bounds[0];
        let right = self.x_bounds[1];
        let bottom = self.y_bounds[0];
        let top = self.y_bounds[1];
        for (x, y) in shape
            .points()
            .filter(|&(x, y)| !(x < left || x > right || y < bottom || y > top))
        {
            let dy = ((top - y) * f64::from(self.height - 1) * 4.0 / (top - bottom)) as usize;
            let dx = ((x - left) * f64::from(self.width - 1) * 2.0 / (right - left)) as usize;
            let index = dy / 4 * self.width as usize + dx / 2;
            self.grid.cells[index] |= DOTS[dy % 4][dx % 2];
            self.grid.colors[index] = shape.color();
        }
    }

    /// Go one layer above in the canvas.
    pub fn layer(&mut self) {
        self.layers.push(self.grid.save());
        self.grid.reset();
        self.dirty = false;
    }

    /// Print a string on the canvas at the given position
    pub fn print(&mut self, x: f64, y: f64, text: &'a str, color: Color) {
        self.labels.push(Label {
            x: x,
            y: y,
            text: text,
            color: color,
        });
    }

    /// Push the last layer if necessary
    fn finish(&mut self) {
        if self.dirty {
            self.layer()
        }
    }
}

/// The Canvas widget may be used to draw more detailed figures using braille patterns (each
/// cell can have a braille character in 8 different positions).
/// # Examples
///
/// ```
/// # extern crate tui;
/// # use tui::widgets::{Block, Borders};
/// # use tui::widgets::canvas::{Canvas, Shape, Line, Map, MapResolution};
/// # use tui::style::Color;
/// # fn main() {
/// Canvas::default()
///     .block(Block::default().title("Canvas").borders(Borders::ALL))
///     .x_bounds([-180.0, 180.0])
///     .y_bounds([-90.0, 90.0])
///     .paint(|ctx| {
///         ctx.draw(&Map{
///             resolution: MapResolution::High,
///             color: Color::White
///         });
///         ctx.layer();
///         ctx.draw(&Line{
///             x1: 0.0,
///             y1: 10.0,
///             x2: 10.0,
///             y2: 10.0,
///             color: Color::White,
///         });
///         ctx.draw(&Line{
///             x1: 10.0,
///             y1: 10.0,
///             x2: 20.0,
///             y2: 20.0,
///             color: Color::Red
///         });
///     });
/// # }
/// ```
pub struct Canvas<'a, F>
where
    F: Fn(&mut Context),
{
    block: Option<Block<'a>>,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    painter: Option<F>,
    background_color: Color,
}

impl<'a, F> Default for Canvas<'a, F>
where
    F: Fn(&mut Context),
{
    fn default() -> Canvas<'a, F> {
        Canvas {
            block: None,
            x_bounds: [0.0, 0.0],
            y_bounds: [0.0, 0.0],
            painter: None,
            background_color: Color::Reset,
        }
    }
}

impl<'a, F> Canvas<'a, F>
where
    F: Fn(&mut Context),
{
    pub fn block(&mut self, block: Block<'a>) -> &mut Canvas<'a, F> {
        self.block = Some(block);
        self
    }
    pub fn x_bounds(&mut self, bounds: [f64; 2]) -> &mut Canvas<'a, F> {
        self.x_bounds = bounds;
        self
    }
    pub fn y_bounds(&mut self, bounds: [f64; 2]) -> &mut Canvas<'a, F> {
        self.y_bounds = bounds;
        self
    }

    /// Store the closure that will be used to draw to the Canvas
    pub fn paint(&mut self, f: F) -> &mut Canvas<'a, F> {
        self.painter = Some(f);
        self
    }

    pub fn background_color(&'a mut self, color: Color) -> &mut Canvas<'a, F> {
        self.background_color = color;
        self
    }
}

impl<'a, F> Widget for Canvas<'a, F>
where
    F: Fn(&mut Context),
{
    fn draw(&mut self, area: &Rect, buf: &mut Buffer) {
        let canvas_area = match self.block {
            Some(ref mut b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        let width = canvas_area.width as usize;
        let height = canvas_area.height as usize;

        if let Some(ref painter) = self.painter {
            // Create a blank context that match the size of the terminal
            let mut ctx = Context {
                x_bounds: self.x_bounds,
                y_bounds: self.y_bounds,
                width: canvas_area.width,
                height: canvas_area.height,
                grid: Grid::new(width, height),
                dirty: false,
                layers: Vec::new(),
                labels: Vec::new(),
            };
            // Paint to this context
            painter(&mut ctx);
            ctx.finish();

            // Retreive painted points for each layer
            for layer in ctx.layers {
                for (i, (ch, color)) in layer
                    .string
                    .chars()
                    .zip(layer.colors.into_iter())
                    .enumerate()
                {
                    if ch != BRAILLE_BLANK {
                        let (x, y) = (i % width, i / width);
                        buf.get_mut(x as u16 + canvas_area.left(), y as u16 + canvas_area.top())
                            .set_char(ch)
                            .set_fg(color)
                            .set_bg(self.background_color);
                    }
                }
            }

            // Finally draw the labels
            let style = Style::default().bg(self.background_color);
            for label in ctx.labels.iter().filter(|l| {
                !(l.x < self.x_bounds[0] || l.x > self.x_bounds[1] || l.y < self.y_bounds[0]
                    || l.y > self.y_bounds[1])
            }) {
                let dy = ((self.y_bounds[1] - label.y) * f64::from(canvas_area.height - 1)
                    / (self.y_bounds[1] - self.y_bounds[0])) as u16;
                let dx = ((label.x - self.x_bounds[0]) * f64::from(canvas_area.width - 1)
                    / (self.x_bounds[1] - self.x_bounds[0])) as u16;
                buf.set_string(
                    dx + canvas_area.left(),
                    dy + canvas_area.top(),
                    label.text,
                    &style.fg(label.color),
                );
            }
        }
    }
}
