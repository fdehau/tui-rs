use std::cmp::{min, max};

use unicode_width::UnicodeWidthStr;

use widgets::{Widget, Block};
use buffer::Buffer;
use layout::Rect;
use style::Color;
use symbols;

pub struct Axis<'a> {
    title: Option<&'a str>,
    bounds: [f64; 2],
    labels: Option<&'a [&'a str]>,
}

impl<'a> Default for Axis<'a> {
    fn default() -> Axis<'a> {
        Axis {
            title: None,
            bounds: [0.0, 0.0],
            labels: None,
        }
    }
}

impl<'a> Axis<'a> {
    pub fn title(mut self, title: &'a str) -> Axis<'a> {
        self.title = Some(title);
        self
    }

    pub fn bounds(mut self, bounds: [f64; 2]) -> Axis<'a> {
        self.bounds = bounds;
        self
    }

    pub fn labels(mut self, labels: &'a [&'a str]) -> Axis<'a> {
        self.labels = Some(labels);
        self
    }

    fn title_width(&self) -> u16 {
        match self.title {
            Some(title) => title.width() as u16,
            None => 0,
        }
    }

    fn max_label_width(&self) -> u16 {
        match self.labels {
            Some(labels) => labels.iter().fold(0, |acc, l| max(l.width(), acc)) as u16,
            None => 0,
        }
    }
}

pub struct Dataset<'a> {
    data: &'a [(f64, f64)],
    color: Color,
}

impl<'a> Default for Dataset<'a> {
    fn default() -> Dataset<'a> {
        Dataset {
            data: &[],
            color: Color::White,
        }
    }
}

impl<'a> Dataset<'a> {
    pub fn data(mut self, data: &'a [(f64, f64)]) -> Dataset<'a> {
        self.data = data;
        self
    }

    pub fn color(mut self, color: Color) -> Dataset<'a> {
        self.color = color;
        self
    }
}

pub struct Chart<'a> {
    block: Option<Block<'a>>,
    x_axis: Axis<'a>,
    y_axis: Axis<'a>,
    datasets: &'a [Dataset<'a>],
    bg: Color,
}

impl<'a> Default for Chart<'a> {
    fn default() -> Chart<'a> {
        Chart {
            block: None,
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            bg: Color::Black,
            datasets: &[],
        }
    }
}

impl<'a> Chart<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut Chart<'a> {
        self.block = Some(block);
        self
    }

    pub fn bg(&mut self, bg: Color) -> &mut Chart<'a> {
        self.bg = bg;
        self
    }

    pub fn x_axis(&mut self, axis: Axis<'a>) -> &mut Chart<'a> {
        self.x_axis = axis;
        self
    }

    pub fn y_axis(&mut self, axis: Axis<'a>) -> &mut Chart<'a> {
        self.y_axis = axis;
        self
    }

    pub fn datasets(&mut self, datasets: &'a [Dataset<'a>]) -> &mut Chart<'a> {
        self.datasets = datasets;
        self
    }
}

impl<'a> Widget<'a> for Chart<'a> {
    fn buffer(&'a self, area: &Rect) -> Buffer<'a> {
        let (mut buf, chart_area) = match self.block {
            Some(ref b) => (b.buffer(area), b.inner(*area)),
            None => (Buffer::empty(*area), *area),
        };

        let margin_x = chart_area.x - area.x;
        let margin_y = chart_area.y - area.y;

        for dataset in self.datasets {
            for &(x, y) in dataset.data.iter() {
                if x <= self.x_axis.bounds[0] || x > self.x_axis.bounds[1] ||
                   y <= self.y_axis.bounds[0] || y > self.y_axis.bounds[1] {
                    continue;
                }
                let dy = (self.y_axis.bounds[1] - y) * (chart_area.height - 1) as f64 /
                         (self.y_axis.bounds[1] - self.y_axis.bounds[0]);
                let dx = (self.x_axis.bounds[1] - x) * (chart_area.width - 1) as f64 /
                         (self.x_axis.bounds[1] - self.x_axis.bounds[0]);
                buf.update_cell(dx as u16 + margin_x, dy as u16 + margin_y, |c| {
                    c.symbol = symbols::DOT;
                    c.fg = dataset.color;
                    c.bg = self.bg;
                })
            }
        }
        buf
    }
}
