use std::cmp::min;
use std::f64::consts::PI;

use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::style::{Color, Style};
use crate::symbols::bar;
use crate::widgets::{Block, Widget};

const TAU: f64 = PI * 2.0;

#[derive(Debug, Clone)]
pub struct PieChart<'a> {
    block: Option<Block<'a>>,
    angles: Vec<((f64, f64), Color)>,
}

impl<'a> Default for PieChart<'a> {
    fn default() -> PieChart<'a> {
        PieChart {
            block: None,
            angles: Vec::new(),
        }
    }
}

impl<'a> PieChart<'a> {
    pub fn data(mut self, data: &'a [(f64, Color)]) -> PieChart<'a> {
        let total: f64 = data.iter().map(|(n, _)| n).sum();
        let ratio = TAU / total;
        let mut original_angle = 0.0;
        let angles: Vec<_> = data
            .iter()
            .map(|(n, c)| {
                let theta = n * ratio;
                let angle0 = original_angle;
                let angle1 = angle0 + theta;
                original_angle = angle1;
                ((angle0, angle1), *c)
            })
            .collect();
        self.angles = angles;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> PieChart<'a> {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for PieChart<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let chart_area = match self.block {
            Some(ref mut b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        let origin: (u16, u16) = (
            chart_area.x + chart_area.width / 2,
            chart_area.y + chart_area.height / 2,
        );
        let radius = min(chart_area.height, chart_area.width) / 2;
        if radius < 2 {
            return;
        }
        let radius: f64 = (radius - 2).into();

        let cell_count = buf.content.len();
        for i in 0..cell_count {
            let (xp, yp) = buf.pos_of(i);
            let color = self.angles.iter().find_map(|((angle0, angle1), color)| {
                let x_origin: f64 = origin.0.into();
                let y_origin: f64 = origin.1.into();
                let xp: f64 = xp.into();
                let yp: f64 = yp.into();
                let opposite = xp - x_origin;
                let adjacent = yp - y_origin;
                let distance_p = (opposite).powi(2) + (adjacent).powi(2);
                if distance_p > radius.powi(2) {
                    return None;
                }
                let mid_angle = {
                    let mid_angle = opposite.atan2(adjacent);
                    if mid_angle < 0.0 {
                        mid_angle + TAU
                    } else {
                        mid_angle
                    }
                };
                if (*angle0 <= mid_angle && mid_angle <= *angle1)
                    || (*angle1 <= mid_angle && mid_angle <= *angle0)
                {
                    return Some(*color);
                }
                None
            });
            if let Some(color) = color {
                buf.get_mut(xp, yp)
                    .set_symbol(bar::FULL)
                    .set_style(Style::default().fg(color));
            }
        }
    }
}
