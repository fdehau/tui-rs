use crate::{
    style::Color,
    widgets::canvas::{Line, Painter, Shape},
};

/// Shape to draw a rectangle from a `Rect` with the given color
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: Color,
    pub fill: bool,
}

impl Shape for Rectangle {
    fn draw(&self, painter: &mut Painter) {
        let mut lines: Vec<Line> = Vec::new();

        if !self.fill {
            lines = [
                Line {
                    x1: self.x,
                    y1: self.y,
                    x2: self.x,
                    y2: self.y + self.height,
                    color: self.color,
                },
                Line {
                    x1: self.x,
                    y1: self.y + self.height,
                    x2: self.x + self.width,
                    y2: self.y + self.height,
                    color: self.color,
                },
                Line {
                    x1: self.x + self.width,
                    y1: self.y,
                    x2: self.x + self.width,
                    y2: self.y + self.height,
                    color: self.color,
                },
                Line {
                    x1: self.x,
                    y1: self.y,
                    x2: self.x + self.width,
                    y2: self.y,
                    color: self.color,
                },
            ]
            .to_vec();
        } else {
            let mut n: f64 = 0.0;

            while n < self.height {
                lines.push(Line {
                    x1: self.x,
                    y1: self.y + n,
                    x2: self.x + self.width,
                    y2: self.y + n,
                    color: self.color,
                });

                n += 0.5
            }
        }

        for line in &lines {
            line.draw(painter);
        }
    }
}
