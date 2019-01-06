use crate::style::Color;
use crate::widgets::canvas::points::PointsIterator;
use crate::widgets::canvas::world::{WORLD_HIGH_RESOLUTION, WORLD_LOW_RESOLUTION};
use crate::widgets::canvas::Shape;

#[derive(Clone, Copy)]
pub enum MapResolution {
    Low,
    High,
}

impl MapResolution {
    fn data(self) -> &'static [(f64, f64)] {
        match self {
            MapResolution::Low => &WORLD_LOW_RESOLUTION,
            MapResolution::High => &WORLD_HIGH_RESOLUTION,
        }
    }
}

/// Shape to draw a world map with the given resolution and color
pub struct Map {
    pub resolution: MapResolution,
    pub color: Color,
}

impl Default for Map {
    fn default() -> Map {
        Map {
            resolution: MapResolution::Low,
            color: Color::Reset,
        }
    }
}

impl<'a> Shape<'a> for Map {
    fn color(&self) -> Color {
        self.color
    }
    fn points(&'a self) -> Box<Iterator<Item = (f64, f64)> + 'a> {
        Box::new(self.into_iter())
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = (f64, f64);
    type IntoIter = PointsIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        PointsIterator::from(self.resolution.data())
    }
}
