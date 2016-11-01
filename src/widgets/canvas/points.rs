use std::slice;

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
        PointsIterator { iter: self.coords.iter() }
    }
}

pub struct PointsIterator<'a> {
    iter: slice::Iter<'a, (f64, f64)>,
}

impl<'a> From<&'a [(f64, f64)]> for PointsIterator<'a> {
    fn from(data: &'a [(f64, f64)]) -> PointsIterator<'a> {
        PointsIterator { iter: data.iter() }
    }
}

impl<'a> Iterator for PointsIterator<'a> {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(p) => Some(*p),
            None => None,
        }
    }
}
