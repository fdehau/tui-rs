use std::cmp::max;

use unicode_width::UnicodeWidthStr;

use widgets::{Widget, Block, border};
use widgets::canvas::{Canvas, Shape, Points};
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

pub enum Marker {
    Dot,
    Braille,
}

pub struct Dataset<'a> {
    name: &'a str,
    data: &'a [(f64, f64)],
    marker: Marker,
    color: Color,
}

impl<'a> Default for Dataset<'a> {
    fn default() -> Dataset<'a> {
        Dataset {
            name: "",
            data: &[],
            marker: Marker::Dot,
            color: Color::Reset,
        }
    }
}

impl<'a> Dataset<'a> {
    pub fn name(mut self, name: &'a str) -> Dataset<'a> {
        self.name = name;
        self
    }

    pub fn data(mut self, data: &'a [(f64, f64)]) -> Dataset<'a> {
        self.data = data;
        self
    }

    pub fn marker(mut self, marker: Marker) -> Dataset<'a> {
        self.marker = marker;
        self
    }

    pub fn color(mut self, color: Color) -> Dataset<'a> {
        self.color = color;
        self
    }
}

#[derive(Debug)]
struct ChartLayout {
    title_x: Option<(u16, u16)>,
    title_y: Option<(u16, u16)>,
    label_x: Option<u16>,
    label_y: Option<u16>,
    axis_x: Option<u16>,
    axis_y: Option<u16>,
    legend_area: Option<Rect>,
    graph_area: Rect,
}

impl Default for ChartLayout {
    fn default() -> ChartLayout {
        ChartLayout {
            title_x: None,
            title_y: None,
            label_x: None,
            label_y: None,
            axis_x: None,
            axis_y: None,
            legend_area: None,
            graph_area: Rect::default(),
        }
    }
}

pub struct Chart<'a> {
    block: Option<Block<'a>>,
    x_axis: Axis<'a>,
    y_axis: Axis<'a>,
    datasets: &'a [Dataset<'a>],
    background_color: Color,
}

impl<'a> Default for Chart<'a> {
    fn default() -> Chart<'a> {
        Chart {
            block: None,
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            background_color: Color::Reset,
            datasets: &[],
        }
    }
}

impl<'a> Chart<'a> {
    pub fn block(&'a mut self, block: Block<'a>) -> &mut Chart<'a> {
        self.block = Some(block);
        self
    }

    pub fn background_color(&mut self, background_color: Color) -> &mut Chart<'a> {
        self.background_color = background_color;
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
                layout.title_x = Some((x + layout.graph_area.width - w, y));
            }
        }

        if let Some(title) = self.y_axis.title {
            let w = title.width() as u16;
            if w + 1 < layout.graph_area.width && layout.graph_area.height > 2 {
                layout.title_y = Some((x + 1, area.top()));
            }
        }

        if let Some(inner_width) = self.datasets.iter().map(|d| d.name.width() as u16).max() {
            let legend_width = inner_width + 2;
            let legend_height = self.datasets.len() as u16 + 2;
            if legend_width < layout.graph_area.width / 3 &&
               legend_height < layout.graph_area.height / 3 {
                layout.legend_area = Some(Rect::new(layout.graph_area.right() - legend_width,
                                                    layout.graph_area.top(),
                                                    legend_width,
                                                    legend_height));
            }
        }
        layout
    }
}

impl<'a> Widget for Chart<'a> {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {
        let chart_area = match self.block {
            Some(ref b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => *area,
        };

        let layout = self.layout(&chart_area);
        let graph_area = layout.graph_area;
        if graph_area.width < 1 || graph_area.height < 1 {
            return;
        }

        if self.background_color != Color::Reset {
            self.background(&chart_area, buf, self.background_color);
        }

        if let Some((x, y)) = layout.title_x {
            let title = self.x_axis.title.unwrap();
            buf.set_string(x, y, title, self.x_axis.title_color, self.background_color);
        }

        if let Some((x, y)) = layout.title_y {
            let title = self.y_axis.title.unwrap();
            buf.set_string(x, y, title, self.y_axis.title_color, self.background_color);
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
                                   self.background_color);
                }
            }
        }

        if let Some(x) = layout.label_y {
            let labels = self.y_axis.labels.unwrap();
            let labels_len = labels.len() as u16;
            for (i, label) in labels.iter().enumerate() {
                let dy = i as u16 * (graph_area.height - 1) / (labels_len - 1);
                if dy < graph_area.bottom() {
                    buf.set_string(x,
                                   graph_area.bottom() - 1 - dy,
                                   label,
                                   self.y_axis.labels_color,
                                   self.background_color);
                }
            }
        }

        if let Some(y) = layout.axis_x {
            for x in graph_area.left()..graph_area.right() {
                buf.set_cell(x,
                             y,
                             symbols::line::HORIZONTAL,
                             self.x_axis.color,
                             self.background_color);
            }
        }

        if let Some(x) = layout.axis_y {
            for y in graph_area.top()..graph_area.bottom() {
                buf.set_cell(x,
                             y,
                             symbols::line::VERTICAL,
                             self.y_axis.color,
                             self.background_color);
            }
        }

        if let Some(y) = layout.axis_x {
            if let Some(x) = layout.axis_y {
                buf.set_cell(x,
                             y,
                             symbols::line::BOTTOM_LEFT,
                             self.x_axis.color,
                             self.background_color);
            }
        }

        for dataset in self.datasets {
            match dataset.marker {
                Marker::Dot => {
                    for &(x, y) in dataset.data.iter().filter(|&&(x, y)| {
                        !(x < self.x_axis.bounds[0] || x > self.x_axis.bounds[1] ||
                          y < self.y_axis.bounds[0] ||
                          y > self.y_axis.bounds[1])
                    }) {
                        let dy = ((self.y_axis.bounds[1] - y) * (graph_area.height - 1) as f64 /
                                  (self.y_axis.bounds[1] -
                                   self.y_axis.bounds[0])) as u16;
                        let dx = ((x - self.x_axis.bounds[0]) * (graph_area.width - 1) as f64 /
                                  (self.x_axis.bounds[1] -
                                   self.x_axis.bounds[0])) as u16;

                        buf.set_cell(graph_area.left() + dx,
                                     graph_area.top() + dy,
                                     symbols::DOT,
                                     dataset.color,
                                     self.background_color);
                    }
                }
                Marker::Braille => {
                    Canvas::default()
                        .background_color(self.background_color)
                        .x_bounds(self.x_axis.bounds)
                        .y_bounds(self.y_axis.bounds)
                        .layer([&Points {
                                    coords: dataset.data,
                                    color: dataset.color,
                                } as &Shape]
                            .as_ref())
                        .draw(&graph_area, buf);
                }
            }
        }

        if let Some(legend_area) = layout.legend_area {
            Block::default()
                .borders(border::ALL)
                .draw(&legend_area, buf);
            for (i, dataset) in self.datasets.iter().enumerate() {
                buf.set_string(legend_area.x + 1,
                               legend_area.y + 1 + i as u16,
                               dataset.name,
                               dataset.color,
                               self.background_color);
            }
        }
    }
}
