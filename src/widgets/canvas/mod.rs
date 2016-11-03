mod points;
mod line;
mod map;
mod world;

pub use self::points::Points;
pub use self::line::Line;
pub use self::map::{Map, MapResolution};

use std::borrow::Cow;

use style::Color;
use buffer::Buffer;
use widgets::{Block, Widget};
use layout::Rect;

pub const DOTS: [[u16; 2]; 4] =
    [[0x0001, 0x0008], [0x0002, 0x0010], [0x0004, 0x0020], [0x0040, 0x0080]];
pub const BRAILLE_OFFSET: u16 = 0x2800;
pub const BRAILLE_BLANK: char = '⠀';

pub trait Shape<'a> {
    fn color(&self) -> Color;
    fn points(&'a self) -> Box<Iterator<Item = (f64, f64)> + 'a>;
}

pub struct Label<'a> {
    pub x: f64,
    pub y: f64,
    pub text: &'a str,
    pub color: Color,
}

pub struct Canvas<'a> {
    block: Option<Block<'a>>,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    layers: Vec<Cow<'a, [&'a Shape<'a>]>>,
    labels: Vec<Label<'a>>,
    background_color: Color,
}

impl<'a> Default for Canvas<'a> {
    fn default() -> Canvas<'a> {
        Canvas {
            block: None,
            x_bounds: [0.0, 0.0],
            y_bounds: [0.0, 0.0],
            layers: Vec::new(),
            labels: Vec::new(),
            background_color: Color::Reset,
        }
    }
}

impl<'a> Canvas<'a> {
    pub fn block(&mut self, block: Block<'a>) -> &mut Canvas<'a> {
        self.block = Some(block);
        self
    }
    pub fn x_bounds(&mut self, bounds: [f64; 2]) -> &mut Canvas<'a> {
        self.x_bounds = bounds;
        self
    }
    pub fn y_bounds(&mut self, bounds: [f64; 2]) -> &mut Canvas<'a> {
        self.y_bounds = bounds;
        self
    }
    pub fn layers<L>(&mut self, layers: Vec<L>) -> &mut Canvas<'a>
        where L: Into<Cow<'a, [&'a Shape<'a>]>>
    {
        self.layers =
            layers.into_iter().map(|l| l.into()).collect::<Vec<Cow<'a, [&'a Shape<'a>]>>>();
        self
    }

    pub fn layer<L>(&mut self, layer: L) -> &mut Canvas<'a>
        where L: Into<Cow<'a, [&'a Shape<'a>]>>
    {
        self.layers.push(layer.into());
        self
    }

    pub fn labels<L>(&mut self, labels: L) -> &mut Canvas<'a>
        where L: Into<Vec<Label<'a>>>
    {
        self.labels = labels.into();
        self
    }

    pub fn background_color(&'a mut self, color: Color) -> &mut Canvas<'a> {
        self.background_color = color;
        self
    }
}

impl<'a> Widget for Canvas<'a> {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {
        let canvas_area = match self.block {
            Some(ref b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        let width = canvas_area.width as usize;
        let height = canvas_area.height as usize;

        for layer in &self.layers {

            let mut grid: Vec<u16> = vec![BRAILLE_OFFSET; width * height];
            let mut colors: Vec<Color> = vec![Color::Reset; width * height];

            for shape in layer.iter() {
                for (x, y) in shape.points().filter(|&(x, y)| {
                    !(x < self.x_bounds[0] || x > self.x_bounds[1] || y < self.y_bounds[0] ||
                      y > self.y_bounds[1])
                }) {
                    let dy = ((self.y_bounds[1] - y) * (canvas_area.height - 1) as f64 * 4.0 /
                              (self.y_bounds[1] -
                               self.y_bounds[0])) as usize;
                    let dx = ((x - self.x_bounds[0]) * (canvas_area.width - 1) as f64 * 2.0 /
                              (self.x_bounds[1] -
                               self.x_bounds[0])) as usize;
                    let index = dy / 4 * width + dx / 2;
                    grid[index] |= DOTS[dy % 4][dx % 2];
                    colors[index] = shape.color();
                }
            }

            let string = String::from_utf16(&grid).unwrap();
            for (i, (ch, color)) in string.chars().zip(colors.into_iter()).enumerate() {
                if ch != BRAILLE_BLANK {
                    let (x, y) = (i % width, i / width);
                    buf.update_cell(x as u16 + canvas_area.left(),
                                    y as u16 + canvas_area.top(),
                                    |c| {
                                        c.symbol.clear();
                                        c.symbol.push(ch);
                                        c.fg = color;
                                        c.bg = self.background_color;
                                    });
                }
            }
        }

        for label in self.labels.iter().filter(|l| {
            !(l.x < self.x_bounds[0] || l.x > self.x_bounds[1] || l.y < self.y_bounds[0] ||
              l.y > self.y_bounds[1])
        }) {
            let dy = ((self.y_bounds[1] - label.y) * (canvas_area.height - 1) as f64 /
                      (self.y_bounds[1] - self.y_bounds[0])) as u16;
            let dx = ((label.x - self.x_bounds[0]) * (canvas_area.width - 1) as f64 /
                      (self.x_bounds[1] - self.x_bounds[0])) as u16;
            buf.set_string(dx + canvas_area.left(),
                           dy + canvas_area.top(),
                           label.text,
                           label.color,
                           self.background_color);
        }
    }
}
