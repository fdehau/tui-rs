use crate::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

/// A shape to draw a group of points with the given color
/// 
/// The X Axis represents the count of columns of the container.
/// 
/// The Y Axis represents the count of rows of the container.
/// 
/// The first value of the coordinate is the X axis, the second value is the Y axis.
#[derive(Debug, Clone)]
pub struct Points<'a> {
    pub coords: &'a [(f64, f64)],
    pub color: Color,
}

impl<'a> Shape for Points<'a> {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.coords {
            if let Some((x, y)) = painter.get_point(*x, *y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

impl<'a> Default for Points<'a> {
    fn default() -> Points<'a> {
        Points {
            coords: &[],
            color: Color::Reset,
        }
    }
}
