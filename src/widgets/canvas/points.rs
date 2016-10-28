use super::Shape;
use style::Color;

pub struct Points<'a> {
    pub coords: &'a [(f64, f64)],
    pub color: Color,
}

impl<'a> Shape<'a> for Points<'a> {
    fn color(&self) -> Color {
        self.color
    }
    fn points(&'a self) -> Box<Iterator<Item = (f64, f64)> + 'a> {
        Box::new(self.into_iter())
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

impl<'a> IntoIterator for &'a Points<'a> {
    type Item = (f64, f64);
    type IntoIter = PointsIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        PointsIterator {
            coords: self.coords,
            index: 0,
        }
    }
}

pub struct PointsIterator<'a> {
    coords: &'a [(f64, f64)],
    index: usize,
}

impl<'a> Iterator for PointsIterator<'a> {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = if self.index < self.coords.len() {
            Some(self.coords[self.index])
        } else {
            None
        };
        self.index += 1;
        point
    }
}
