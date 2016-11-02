mod points;
mod line;
mod rectangle;
mod map;
mod world;

pub use self::points::Points;
pub use self::line::Line;
pub use self::rectangle::Rectangle;
pub use self::map::{Map, MapResolution};

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

pub struct Canvas<'a> {
    block: Option<Block<'a>>,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    layers: &'a [&'a [&'a Shape<'a>]],
    background_color: Color,
}

impl<'a> Default for Canvas<'a> {
    fn default() -> Canvas<'a> {
        Canvas {
            block: None,
            x_bounds: [0.0, 0.0],
            y_bounds: [0.0, 0.0],
            layers: &[],
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
    pub fn layers(&mut self, layers: &'a [&'a [&'a Shape<'a>]]) -> &mut Canvas<'a> {
        self.layers = layers;
        self
    }

    pub fn background_color(&'a mut self, color: Color) -> &mut Canvas<'a> {
        self.background_color = color;
        self
    }
}

impl<'a> Widget for Canvas<'a> {
    fn buffer(&self, area: &Rect, buf: &mut Buffer) {
        let canvas_area = match self.block {
            Some(ref b) => {
                b.buffer(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        let width = canvas_area.width as usize;
        let height = canvas_area.height as usize;

        let mut x_bounds = self.x_bounds;
        x_bounds.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut y_bounds = self.y_bounds;
        y_bounds.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for layer in self.layers {

            let mut grid: Vec<u16> = vec![BRAILLE_OFFSET; width * height];
            let mut colors: Vec<Color> = vec![Color::Reset; width * height];

            for shape in layer.iter() {
                for (x, y) in shape.points().filter(|&(x, y)| {
                    !(x < x_bounds[0] || x > x_bounds[1] || y < y_bounds[0] || y > y_bounds[1])
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
    }
}
