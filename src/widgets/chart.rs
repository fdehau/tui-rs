use std::cmp::max;

use unicode_width::UnicodeWidthStr;

use widgets::{Widget, Block};
use buffer::Buffer;
use layout::Rect;
use style::Color;
use symbols;

pub struct Axis<'a> {
    title: Option<&'a str>,
    title_color: Color,
    bounds: [f64; 2],
    labels: Option<&'a [&'a str]>,
    labels_color: Color,
    color: Color,
}

impl<'a> Default for Axis<'a> {
    fn default() -> Axis<'a> {
        Axis {
            title: None,
            title_color: Color::Reset,
            bounds: [0.0, 0.0],
            labels: None,
            labels_color: Color::Reset,
            color: Color::Reset,
        }
    }
}

impl<'a> Axis<'a> {
    pub fn title(mut self, title: &'a str) -> Axis<'a> {
        self.title = Some(title);
        self
    }

    pub fn title_color(mut self, color: Color) -> Axis<'a> {
        self.title_color = color;
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

    pub fn labels_color(mut self, color: Color) -> Axis<'a> {
        self.labels_color = color;
        self
    }

    pub fn color(mut self, color: Color) -> Axis<'a> {
        self.color = color;
        self
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
            color: Color::Reset,
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
            bg: Color::Reset,
            datasets: &[],
        }
    }
}

#[derive(Debug)]
struct ChartLayout {
    legend_x: Option<(u16, u16)>,
    legend_y: Option<(u16, u16)>,
    label_x: Option<u16>,
    label_y: Option<u16>,
    axis_x: Option<u16>,
    axis_y: Option<u16>,
    graph_area: Rect,
}

impl Default for ChartLayout {
    fn default() -> ChartLayout {
        ChartLayout {
            legend_x: None,
            legend_y: None,
            label_x: None,
            label_y: None,
            axis_x: None,
            axis_y: None,
            graph_area: Rect::default(),
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

    fn layout(&self, area: &Rect) -> ChartLayout {
        let mut layout = ChartLayout::default();
        if area.height == 0 || area.width == 0 {
            return layout;
        }
        let mut x = area.left();
        let mut y = area.bottom() - 1;

        if self.x_axis.labels.is_some() && y > area.top() {
            layout.label_x = Some(y);
            y -= 1;
        }

        if let Some(labels) = self.y_axis.labels {
            let max_width = labels.iter().fold(0, |acc, l| max(l.width(), acc)) as u16;
            if x + max_width < area.right() {
                layout.label_y = Some(x);
                x += max_width;
            }
        }

        if self.x_axis.labels.is_some() && y > area.top() {
            layout.axis_x = Some(y);
            y -= 1;
        }

        if self.y_axis.labels.is_some() && x + 1 < area.right() {
            layout.axis_y = Some(x);
            x += 1;
        }

        if x < area.right() && y > 1 {
            layout.graph_area = Rect::new(x, area.top(), area.right() - x, y - area.top() + 1);
        }

        if let Some(title) = self.x_axis.title {
            let w = title.width() as u16;
            if w < layout.graph_area.width && layout.graph_area.height > 2 {
                layout.legend_x = Some((x + layout.graph_area.width - w, y));
            }
        }

        if let Some(title) = self.y_axis.title {
            let w = title.width() as u16;
            if w + 1 < layout.graph_area.width && layout.graph_area.height > 2 {
                layout.legend_y = Some((x + 1, area.top()));
            }
        }
        layout
    }
}

impl<'a> Widget for Chart<'a> {
    fn buffer(&self, area: &Rect, buf: &mut Buffer) {
        let chart_area = match self.block {
            Some(ref b) => {
                b.buffer(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        let layout = self.layout(&chart_area);
        let graph_area = layout.graph_area;
        if graph_area.width == 0 || graph_area.height == 0 {
            return;
        }

        if let Some((x, y)) = layout.legend_x {
            let title = self.x_axis.title.unwrap();
            buf.set_string(x, y, title, self.x_axis.title_color, self.bg);
        }

        if let Some((x, y)) = layout.legend_y {
            let title = self.y_axis.title.unwrap();
            buf.set_string(x, y, title, self.y_axis.title_color, self.bg);
        }

        if let Some(y) = layout.label_x {
            let labels = self.x_axis.labels.unwrap();
            let total_width = labels.iter().fold(0, |acc, l| l.width() + acc) as u16;
            let labels_len = labels.len() as u16;
            if total_width < graph_area.width && labels_len > 1 {
                for (i, label) in labels.iter().enumerate() {
                    buf.set_string(graph_area.left() +
                                   i as u16 * (graph_area.width - 1) / (labels_len - 1) -
                                   label.width() as u16,
                                   y,
                                   label,
                                   self.x_axis.labels_color,
                                   self.bg);
                }
            }
        }

        if let Some(x) = layout.label_y {
            let labels = self.y_axis.labels.unwrap();
            let labels_len = labels.len() as u16;
            if labels_len > 1 {
                for (i, label) in labels.iter().enumerate() {
                    buf.set_string(x,
                                   graph_area.bottom() -
                                   i as u16 * (graph_area.height - 1) / (labels_len - 1),
                                   label,
                                   self.y_axis.labels_color,
                                   self.bg);
                }
            }
        }

        if let Some(y) = layout.axis_x {
            for x in graph_area.left()..graph_area.right() {
                buf.update_cell(x, y, symbols::line::HORIZONTAL, self.x_axis.color, self.bg);
            }
        }

        if let Some(x) = layout.axis_y {
            for y in graph_area.top()..graph_area.bottom() {
                buf.update_cell(x, y, symbols::line::VERTICAL, self.y_axis.color, self.bg);
            }
        }

        if let Some(y) = layout.axis_x {
            if let Some(x) = layout.axis_y {
                buf.update_cell(x, y, symbols::line::BOTTOM_LEFT, self.x_axis.color, self.bg);
            }
        }

        for dataset in self.datasets {
            for &(x, y) in dataset.data.iter() {
                if x < self.x_axis.bounds[0] || x > self.x_axis.bounds[1] ||
                   y < self.y_axis.bounds[0] || y > self.y_axis.bounds[1] {
                    continue;
                }
                let dy = (self.y_axis.bounds[1] - y) * graph_area.height as f64 /
                         (self.y_axis.bounds[1] - self.y_axis.bounds[0]);
                let dx = (self.x_axis.bounds[1] - x) * graph_area.width as f64 /
                         (self.x_axis.bounds[1] - self.x_axis.bounds[0]);
                buf.update_cell(dx as u16 + graph_area.left(),
                                dy as u16 + graph_area.top(),
                                symbols::BLACK_CIRCLE,
                                dataset.color,
                                self.bg);
            }
        }
    }
}
