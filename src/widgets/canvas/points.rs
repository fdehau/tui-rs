use crate::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

/// A shape to draw a group of points with the given color
#[derive(Debug, Clone)]
pub struct Points<'a> {
    pub coords: &'a [(f64, f64, bool)],
    pub color: Color,
}

impl<'a> Shape for Points<'a> {
    fn draw(&self, painter: &mut Painter) {
        for (x, y, drawed) in self.coords {
            if *drawed {
                if let Some((x, y)) = painter.get_point(*x, *y) {
                    painter.paint(x, y, self.color);
                }
            } else if let Some((x, y)) = painter.get_point(*x, *y) {
                //painter.paint(x, y, Color::Reset);
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
