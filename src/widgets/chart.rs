use crate::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    symbols,
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders, Widget,
    },
};
use std::{borrow::Cow, cmp::max};
use unicode_width::UnicodeWidthStr;

/// An X or Y axis for the chart widget
#[derive(Debug, Default, Clone)]
pub struct Axis<'a> {
    /// Title displayed next to axis end.
    /// Cannot be modified directly, only with `retitle()` and `untitle()`.
    title:      Option<Spans<'a>>,
    /// Bounds for the axis (all data points outside these limits will not be represented)
    pub bounds: [f64; 2],
    /// A list of labels to put to the left or below the axis
    pub labels: Option<Vec<Span<'a>>>,
    /// The style used to draw the axis itself
    pub style:  Style,
}

impl<'a> Axis<'a> {
    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Spans<'a>>,
    {
        self.title = Some(title.into());
        self
    }

    pub fn retitle<T>(&mut self, title: T)
    where
        T: Into<Spans<'a>>,
    {
        self.title = Some(title.into());
    }

    pub fn untitle(&mut self) {
        self.title = None;
    }

    #[deprecated(
        since = "0.10.0",
        note = "You should use styling capabilities of `text::Spans` given as argument of the `title` method to apply styling to the title."
    )]
    pub fn title_style(mut self, style: Style) -> Self {
        if let Some(t) = self.title {
            let title = String::from(t);
            self.title = Some(Spans::from(Span::styled(title, style)));
        }
        self
    }

    pub fn bounds(mut self, bounds: [f64; 2]) -> Self {
        self.bounds = bounds;
        self
    }

    pub fn labels(mut self, labels: Vec<Span<'a>>) -> Self {
        self.labels = Some(labels);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

/// Used to determine which style of graphing to use
#[derive(Debug, Clone, Copy)]
pub enum GraphType {
    /// Draw each point
    Scatter,
    /// Draw each point and lines between each point using the same marker
    Line,
}

impl Default for GraphType {
    fn default() -> Self {
        Self::Scatter
    }
}

/// A group of data points
#[derive(Debug, Clone)]
pub struct Dataset<'a> {
    /// Name of the dataset (used in the legend if shown).
    /// Cannot be modified directly, only with `rename()`.
    name:           Cow<'a, str>,
    /// A reference to the actual data
    pub data:       &'a [(f64, f64)],
    /// Symbol used for each points of this dataset
    pub marker:     symbols::Marker,
    /// Determines graph type used for drawing points
    pub graph_type: GraphType,
    /// Style used to plot this dataset
    pub style:      Style,
}

impl<'a> Default for Dataset<'a> {
    fn default() -> Dataset<'a> {
        Dataset {
            name: Cow::from(""),
            data: &[],
            marker: symbols::Marker::Dot,
            graph_type: GraphType::Scatter,
            style: Style::default(),
        }
    }
}

impl<'a> Dataset<'a> {
    pub fn name<S>(mut self, name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.name = name.into();
        self
    }

    pub fn rename<S>(&mut self, name: S)
    where
        S: Into<Cow<'a, str>>,
    {
        self.name = name.into();
    }

    pub fn data(mut self, data: &'a [(f64, f64)]) -> Self {
        self.data = data;
        self
    }

    pub fn marker(mut self, marker: symbols::Marker) -> Self {
        self.marker = marker;
        self
    }

    pub fn graph_type(mut self, graph_type: GraphType) -> Self {
        self.graph_type = graph_type;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

/// A container that holds all the infos about where to display each elements of the chart (axis,
/// labels, legend, ...).
#[derive(Debug, Default, Clone, PartialEq)]
struct ChartLayout {
    /// Location of the title of the x axis
    title_x:      Option<(u16, u16)>,
    /// Location of the title of the y axis
    title_y:      Option<(u16, u16)>,
    /// Location of the first label of the x axis
    label_x:      Option<u16>,
    /// Location of the first label of the y axis
    label_y:      Option<u16>,
    /// Y coordinate of the horizontal axis
    axis_x:       Option<u16>,
    /// X coordinate of the vertical axis
    axis_y:       Option<u16>,
    /// Area of the legend
    legend_area:  Option<Rect>,
    /// Area of the graph
    graph_area:   Rect,
}

/// A widget to plot one or more dataset in a cartesian coordinate system
///
/// # Examples
///
/// ```
/// # use tui::symbols;
/// # use tui::widgets::{Block, Borders, Chart, Axis, Dataset, GraphType};
/// # use tui::style::{Style, Color};
/// # use tui::text::Span;
/// let datasets = vec![
///     Dataset::default()
///         .name("data1")
///         .marker(symbols::Marker::Dot)
///         .graph_type(GraphType::Scatter)
///         .style(Style::default().fg(Color::Cyan))
///         .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
///     Dataset::default()
///         .name("data2")
///         .marker(symbols::Marker::Braille)
///         .graph_type(GraphType::Line)
///         .style(Style::default().fg(Color::Magenta))
///         .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
/// ];
/// Chart::new(datasets)
///     .block(Block::default().title("Chart"))
///     .x_axis(Axis::default()
///         .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
///         .style(Style::default().fg(Color::White))
///         .bounds([0.0, 10.0])
///         .labels(["0.0", "5.0", "10.0"].iter().cloned().map(Span::from).collect()))
///     .y_axis(Axis::default()
///         .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
///         .style(Style::default().fg(Color::White))
///         .bounds([0.0, 10.0])
///         .labels(["0.0", "5.0", "10.0"].iter().cloned().map(Span::from).collect()));
/// ```
#[derive(Debug, Clone)]
pub struct Chart<'a> {
    /// A block to display around the widget eventually
    pub block:                      Option<Block<'a>>,
    /// The horizontal axis
    pub x_axis:                     Axis<'a>,
    /// The vertical axis
    pub y_axis:                     Axis<'a>,
    /// A reference to the datasets
    pub datasets:                   Vec<Dataset<'a>>,
    /// The widget base style
    pub style:                      Style,
    /// Constraints used to determine whether the legend should be shown or not
    pub hidden_legend_constraints:  (Constraint, Constraint),
}

impl<'a> Chart<'a> {
    pub fn new(datasets: Vec<Dataset<'a>>) -> Self {
        Self {
            block:  None,
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            style:  Default::default(),
            datasets,
            hidden_legend_constraints: (Constraint::Ratio(1, 4), Constraint::Ratio(1, 4)),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Style the `Chart`.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the abscissa of the `Chart`.
    pub fn x_axis(mut self, axis: Axis<'a>) -> Self {
        self.x_axis = axis;
        self
    }

    /// Set the ordinate of the `Chart`.
    pub fn y_axis(mut self, axis: Axis<'a>) -> Self {
        self.y_axis = axis;
        self
    }

    /// Set the constraints used to determine whether the legend should be shown or not.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tui::widgets::Chart;
    /// # use tui::layout::Constraint;
    /// let constraints = (
    ///     Constraint::Ratio(1, 3),
    ///     Constraint::Ratio(1, 4)
    /// );
    /// // Hide the legend when either its width is greater than 33% of the total widget width
    /// // or if its height is greater than 25% of the total widget height.
    /// let _chart: Chart = Chart::new(vec![])
    ///     .hidden_legend_constraints(constraints);
    /// ```
    pub fn hidden_legend_constraints(mut self, constraints: (Constraint, Constraint)) -> Self {
        self.hidden_legend_constraints = constraints;
        self
    }

    /// Compute the internal layout of the chart given the area. If the area is too small some
    /// elements may be automatically hidden
    fn layout(&self, area: Rect) -> ChartLayout {
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

        if let Some(ref y_labels) = self.y_axis.labels {
            let mut max_width = y_labels.iter().map(Span::width).max().unwrap_or_default() as u16;
            if let Some(ref x_labels) = self.x_axis.labels {
                if !x_labels.is_empty() {
                    max_width = max(max_width, x_labels[0].content.width() as u16);
                }
            }
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

        if let Some(ref title) = self.x_axis.title {
            let w = title.width() as u16;
            if w < layout.graph_area.width && layout.graph_area.height > 2 {
                layout.title_x = Some((x + layout.graph_area.width - w, y));
            }
        }

        if let Some(ref title) = self.y_axis.title {
            let w = title.width() as u16;
            if w + 1 < layout.graph_area.width && layout.graph_area.height > 2 {
                layout.title_y = Some((x, area.top()));
            }
        }

        if let Some(inner_width) = self.datasets.iter().map(|d| d.name.width() as u16).max() {
            let legend_width = inner_width + 2;
            let legend_height = self.datasets.len() as u16 + 2;
            let max_legend_width = self
                .hidden_legend_constraints
                .0
                .apply(layout.graph_area.width);
            let max_legend_height = self
                .hidden_legend_constraints
                .1
                .apply(layout.graph_area.height);
            if inner_width > 0
                && legend_width < max_legend_width
                && legend_height < max_legend_height
            {
                layout.legend_area = Some(Rect::new(
                    layout.graph_area.right() - legend_width,
                    layout.graph_area.top(),
                    legend_width,
                    legend_height,
                ));
            }
        }
        layout
    }
}

impl<'a> Widget for Chart<'a> {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }
        buf.set_style(area, self.style);
        // Sample the style of the entire widget. This sample will be used to reset the style of
        // the cells that are part of the components put on top of the grah area (i.e legend and
        // axis names).
        let original_style = buf.get(area.left(), area.top()).style();

        let chart_area = match self.block.take() {
            Some(mut b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let layout = self.layout(chart_area);
        let graph_area = layout.graph_area;
        if graph_area.width < 1 || graph_area.height < 1 {
            return;
        }

        if let Some(y) = layout.label_x {
            if let Some(labels) = &self.x_axis.labels {
                let total_width = labels.iter().map(Span::width).sum::<usize>() as u16;
                let labels_len = labels.len() as u16;
                if total_width < graph_area.width && labels_len > 1 {
                    for (i, label) in labels.iter().enumerate() {
                        buf.set_span(
                            graph_area.left() + i as u16 * (graph_area.width - 1) / (labels_len - 1)
                                - label.content.width() as u16,
                            y,
                            label,
                            label.width() as u16,
                        );
                    }
                }
            } else {
                panic!("x_axis_labels must be something!");
            }
        }

        if let Some(x) = layout.label_y {
            if let Some(labels) = &self.y_axis.labels {
                let labels_len = labels.len() as u16;
                for (i, label) in labels.iter().enumerate() {
                    let dy = i as u16 * (graph_area.height - 1) / (labels_len - 1);
                    if dy < graph_area.bottom() {
                        buf.set_span(x, graph_area.bottom() - 1 - dy, label, label.width() as u16);
                    }
                }
            } else {
                panic!("y_axis_labels must be something!");
            }
        }

        if let Some(y) = layout.axis_x {
            for x in graph_area.left()..graph_area.right() {
                buf.get_mut(x, y)
                    .set_symbol(symbols::line::HORIZONTAL)
                    .set_style(self.x_axis.style);
            }
        }

        if let Some(x) = layout.axis_y {
            for y in graph_area.top()..graph_area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(symbols::line::VERTICAL)
                    .set_style(self.y_axis.style);
            }
        }

        if let Some(y) = layout.axis_x {
            if let Some(x) = layout.axis_y {
                buf.get_mut(x, y)
                    .set_symbol(symbols::line::BOTTOM_LEFT)
                    .set_style(self.x_axis.style);
            }
        }

        for dataset in &self.datasets {
            Canvas::default()
                .background_color(self.style.bg.unwrap_or(Color::Reset))
                .x_bounds(self.x_axis.bounds)
                .y_bounds(self.y_axis.bounds)
                .marker(dataset.marker)
                .paint(|ctx| {
                    ctx.draw(&Points {
                        coords: dataset.data,
                        color: dataset.style.fg.unwrap_or(Color::Reset),
                    });
                    if let GraphType::Line = dataset.graph_type {
                        for data in dataset.data.windows(2) {
                            ctx.draw(&Line {
                                x1: data[0].0,
                                y1: data[0].1,
                                x2: data[1].0,
                                y2: data[1].1,
                                color: dataset.style.fg.unwrap_or(Color::Reset),
                            })
                        }
                    }
                })
                .render(graph_area, buf);
        }

        if let Some(legend_area) = layout.legend_area {
            buf.set_style(legend_area, original_style);
            Block::default()
                .borders(Borders::ALL)
                .render(legend_area, buf);
            for (i, dataset) in self.datasets.iter().enumerate() {
                buf.set_string(
                    legend_area.x + 1,
                    legend_area.y + 1 + i as u16,
                    &dataset.name,
                    dataset.style,
                );
            }
        }

        if let Some((x, y)) = layout.title_x {
            //  If title is Some(Span{content: Cow::Owned(…), style: …}),
            //    this allocates memory,
            //  otherwise this clone is only copy by value.
            let title = self.x_axis.title.clone().unwrap();
            let width = graph_area.right().saturating_sub(x);
            buf.set_style(
                Rect {
                    x,
                    y,
                    width,
                    height: 1,
                },
                original_style,
            );
            buf.set_spans(x, y, &title, width);
        }

        if let Some((x, y)) = layout.title_y {
            //  If title is Some(Span{content: Cow::Owned(…), style: …}),
            //    this allocates memory,
            //  otherwise this clone is only copy by value.
            let title = self.y_axis.title.clone().unwrap();
            let width = graph_area.right().saturating_sub(x);
            buf.set_style(
                Rect {
                    x,
                    y,
                    width,
                    height: 1,
                },
                original_style,
            );
            buf.set_spans(x, y, &title, width);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct LegendTestCase {
        chart_area:                 Rect,
        hidden_legend_constraints:  (Constraint, Constraint),
        legend_area:                Option<Rect>,
    }

    #[test]
    fn it_should_hide_the_legend() {
        let data = [(0.0, 5.0), (1.0, 6.0), (3.0, 7.0)];
        let cases = [
            LegendTestCase {
                chart_area: Rect::new(0, 0, 100, 100),
                hidden_legend_constraints: (Constraint::Ratio(1, 4), Constraint::Ratio(1, 4)),
                legend_area: Some(Rect::new(88, 0, 12, 12)),
            },
            LegendTestCase {
                chart_area: Rect::new(0, 0, 100, 100),
                hidden_legend_constraints: (Constraint::Ratio(1, 10), Constraint::Ratio(1, 4)),
                legend_area: None,
            },
        ];
        for case in &cases {
            let datasets = (0..10)
                .map(|i| {
                    let name = format!("Dataset #{}", i);
                    Dataset::default().name(name).data(&data)
                })
                .collect::<Vec<_>>();
            let chart = Chart::new(datasets)
                .x_axis(Axis::default().title("X axis"))
                .y_axis(Axis::default().title("Y axis"))
                .hidden_legend_constraints(case.hidden_legend_constraints);
            let layout = chart.layout(case.chart_area);
            assert_eq!(layout.legend_area, case.legend_area);
        }
    }
}
