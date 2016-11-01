use super::Shape;
use style::Color;

/// # Examples
///
/// ```
/// use tui::style::Color;
/// use tui::widgets::canvas::{Shape, Rectangle};
///
/// let rectangle = Rectangle { x: 4.0, y: 4.0, width: 4.0, height: 4.0, color: Color::Red };
/// let points = rectangle.points().collect::<Vec<(f64, f64)>>();
/// assert_eq!(&points, &[(4.0, 4.0), (5.0, 4.0), (6.0, 4.0), (7.0, 4.0), (4.0, 5.0), (7.0, 5.0),
/// (4.0, 6.0), (7.0, 6.0), (4.0, 7.0), (5.0, 7.0), (6.0, 7.0), (7.0, 7.0)]);
/// ```
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: Color,
}

pub struct RectangleIterator<'a> {
    rect: &'a Rectangle,
    x: f64,
    y: f64,
    right: f64,
    bottom: f64,
}

impl<'a> Iterator for RectangleIterator<'a> {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.bottom {
            let pos = (self.x, self.y);
            let dx = if self.y == self.rect.y || self.y == self.bottom - 1.0 {
                1.0
            } else {
                self.rect.width - 1.0
            };
            self.x += dx;
            if self.x >= self.right {
                self.x = self.rect.x;
                self.y += 1.0;
            }
            Some(pos)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Rectangle {
    type Item = (f64, f64);
    type IntoIter = RectangleIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        let right = self.x + self.width;
        let bottom = self.y + self.height;
        RectangleIterator {
            rect: self,
            x: self.x,
            y: self.y,
            right: right,
            bottom: bottom,
        }
    }
}

impl<'a> Shape<'a> for Rectangle {
    fn color(&self) -> Color {
        self.color
    }
    fn points(&'a self) -> Box<Iterator<Item = (f64, f64)> + 'a> {
        Box::new(self.into_iter())
    }
}
