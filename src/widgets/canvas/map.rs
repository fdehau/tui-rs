use crate::{
    style::Color,
    widgets::canvas::{
        world::{WORLD_HIGH_RESOLUTION, WORLD_LOW_RESOLUTION},
        Painter, Shape,
    },
};

#[derive(Debug, Clone, Copy)]
pub enum MapResolution {
    Low,
    High,
}

impl Default for MapResolution {
    fn default() -> Self {
        Self::Low
    }
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
#[derive(Debug, Default, Clone)]
pub struct Map {
    pub resolution: MapResolution,
    pub color: Color,
}

impl Shape for Map {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        for (x, y) in self.resolution.data() {
            if let Some((x, y)) = painter.get_point(*x, *y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
