use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::HashMap;

use cassowary::strength::{REQUIRED, WEAK};
use cassowary::WeightedRelation::*;
use cassowary::{Constraint as CassowaryConstraint, Expression, Solver, Variable};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unit {
    Length(u16),
    Percentage(u16),
    Ratio(u32, u32),
}

impl From<u16> for Unit {
    fn from(v: u16) -> Unit {
        Unit::Length(v)
    }
}

impl Unit {
    pub(crate) fn apply(&self, length: u16) -> u16 {
        match *self {
            Unit::Percentage(p) => length * p / 100,
            Unit::Ratio(num, den) => {
                let r = num * u32::from(length) / den;
                r as u16
            }
            Unit::Length(l) => length.min(l),
        }
    }

    fn check(&self) {
        match *self {
            Unit::Percentage(p) => {
                assert!(
                    p <= 100,
                    "Percentages should be between 0 and 100 inclusively."
                );
            }
            Unit::Ratio(num, den) => {
                assert!(
                    num <= den,
                    "Ratio numerator should be less than or equalt to denominator."
                );
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Equal,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Constraint {
    operator: Operator,
    unit: Unit,
    weight: u8,
}

impl Constraint {
    pub fn gte<T>(u: T) -> Constraint
    where
        T: Into<Unit>,
    {
        let u = u.into();
        u.check();
        Constraint {
            operator: Operator::GreaterThanOrEqual,
            unit: u,
            weight: 0,
        }
    }

    pub fn lte<T>(u: T) -> Constraint
    where
        T: Into<Unit>,
    {
        let u = u.into();
        u.check();
        Constraint {
            operator: Operator::LessThanOrEqual,
            unit: u,
            weight: 0,
        }
    }

    pub fn eq<T>(u: T) -> Constraint
    where
        T: Into<Unit>,
    {
        let u = u.into();
        u.check();
        Constraint {
            operator: Operator::Equal,
            unit: u,
            weight: 0,
        }
    }

    pub fn weight(mut self, weight: u8) -> Constraint {
        self.weight = weight;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constraints(Vec<Constraint>);

impl From<Constraint> for Constraints {
    fn from(c: Constraint) -> Constraints {
        Constraints(vec![c])
    }
}

impl<T> std::iter::FromIterator<T> for Constraints
where
    T: Into<Constraint>,
{
    fn from_iter<I>(iter: I) -> Constraints
    where
        I: IntoIterator<Item = T>,
    {
        Constraints(iter.into_iter().map(|c| c.into()).collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Margin {
    pub vertical: u16,
    pub horizontal: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Layout {
    direction: Direction,
    margin: Margin,
    constraints: Vec<Constraints>,
}

thread_local! {
    static LAYOUT_CACHE: RefCell<HashMap<(Rect, Layout), Vec<Rect>>> = RefCell::new(HashMap::new());
}

impl Default for Layout {
    fn default() -> Layout {
        Layout {
            direction: Direction::Vertical,
            margin: Margin {
                horizontal: 0,
                vertical: 0,
            },
            constraints: Vec::new(),
        }
    }
}

impl Layout {
    pub fn constraints<I>(mut self, constraints: I) -> Layout
    where
        I: IntoIterator,
        I::Item: Into<Constraints>,
    {
        self.constraints = constraints.into_iter().map(|c| c.into()).collect();
        self
    }

    pub fn margin(mut self, margin: u16) -> Layout {
        self.margin = Margin {
            horizontal: margin,
            vertical: margin,
        };
        self
    }

    pub fn horizontal_margin(mut self, horizontal: u16) -> Layout {
        self.margin.horizontal = horizontal;
        self
    }

    pub fn vertical_margin(mut self, vertical: u16) -> Layout {
        self.margin.vertical = vertical;
        self
    }

    pub fn direction(mut self, direction: Direction) -> Layout {
        self.direction = direction;
        self
    }

    /// Wrapper function around the cassowary-rs solver to be able to split a given
    /// area into smaller ones based on the preferred widths or heights and the direction.
    ///
    /// # Examples
    /// ```
    /// # use tui::layout::{Rect, Constraint, Direction, Layout, Unit};
    /// let chunks = Layout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints([
    ///         Constraint::eq(5).weight(10),
    ///         Constraint::eq(Unit::Percentage(100)).weight(5)
    ///     ])
    ///     .split(Rect {
    ///         x: 2,
    ///         y: 2,
    ///         width: 10,
    ///         height: 10,
    ///     });
    /// assert_eq!(
    ///     chunks,
    ///     vec![
    ///         Rect {
    ///             x: 2,
    ///             y: 2,
    ///             width: 10,
    ///             height: 5
    ///         },
    ///         Rect {
    ///             x: 2,
    ///             y: 7,
    ///             width: 10,
    ///             height: 5
    ///         }
    ///     ]
    /// );
    ///
    /// let chunks = Layout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([Constraint::eq(Unit::Ratio(1, 3)), Constraint::eq(Unit::Ratio(2, 3))])
    ///     .split(Rect {
    ///         x: 0,
    ///         y: 0,
    ///         width: 9,
    ///         height: 2,
    ///     });
    /// assert_eq!(
    ///     chunks,
    ///     vec![
    ///         Rect {
    ///             x: 0,
    ///             y: 0,
    ///             width: 3,
    ///             height: 2
    ///         },
    ///         Rect {
    ///             x: 3,
    ///             y: 0,
    ///             width: 6,
    ///             height: 2
    ///         }
    ///     ]
    /// );
    /// ```
    pub fn split(&self, area: Rect) -> Vec<Rect> {
        // TODO: Maybe use a fixed size cache ?
        LAYOUT_CACHE.with(|c| {
            c.borrow_mut()
                .entry((area, self.clone()))
                .or_insert_with(|| split(area, self))
                .clone()
        })
    }
}

fn split(area: Rect, layout: &Layout) -> Vec<Rect> {
    let mut solver = Solver::new();
    let mut vars: HashMap<Variable, (usize, usize)> = HashMap::new();
    let elements = layout
        .constraints
        .iter()
        .map(|_| Element::new())
        .collect::<Vec<Element>>();
    let mut results = layout
        .constraints
        .iter()
        .map(|_| Rect::default())
        .collect::<Vec<Rect>>();

    let dest_area = area.inner(&layout.margin);
    for (i, e) in elements.iter().enumerate() {
        vars.insert(e.x, (i, 0));
        vars.insert(e.y, (i, 1));
        vars.insert(e.width, (i, 2));
        vars.insert(e.height, (i, 3));
    }
    let mut ccs: Vec<CassowaryConstraint> =
        Vec::with_capacity(elements.len() * 4 + layout.constraints.len() * 6);
    for elt in &elements {
        ccs.push(elt.width | GE(REQUIRED) | 0f64);
        ccs.push(elt.height | GE(REQUIRED) | 0f64);
        ccs.push(elt.left() | GE(REQUIRED) | f64::from(dest_area.left()));
        ccs.push(elt.top() | GE(REQUIRED) | f64::from(dest_area.top()));
        ccs.push(elt.right() | LE(REQUIRED) | f64::from(dest_area.right()));
        ccs.push(elt.bottom() | LE(REQUIRED) | f64::from(dest_area.bottom()));
    }
    if let Some(first) = elements.first() {
        ccs.push(match layout.direction {
            Direction::Horizontal => first.left() | EQ(REQUIRED) | f64::from(dest_area.left()),
            Direction::Vertical => first.top() | EQ(REQUIRED) | f64::from(dest_area.top()),
        });
    }
    match layout.direction {
        Direction::Horizontal => {
            for pair in elements.windows(2) {
                ccs.push((pair[0].x + pair[0].width) | EQ(REQUIRED) | pair[1].x);
            }
            for (i, c) in layout.constraints.iter().enumerate() {
                ccs.push(elements[i].y | EQ(REQUIRED) | f64::from(dest_area.y));
                ccs.push(elements[i].height | EQ(REQUIRED) | f64::from(dest_area.height));
                apply_constraints(&mut ccs, c, elements[i].width, dest_area.width);
            }
        }
        Direction::Vertical => {
            for pair in elements.windows(2) {
                ccs.push((pair[0].y + pair[0].height) | EQ(REQUIRED) | pair[1].y);
            }
            for (i, c) in layout.constraints.iter().enumerate() {
                ccs.push(elements[i].x | EQ(REQUIRED) | f64::from(dest_area.x));
                ccs.push(elements[i].width | EQ(REQUIRED) | f64::from(dest_area.width));
                apply_constraints(&mut ccs, c, elements[i].height, dest_area.height);
            }
        }
    }
    solver.add_constraints(&ccs).unwrap();
    for &(var, value) in solver.fetch_changes() {
        let (index, attr) = vars[&var];
        let value = if value.is_sign_negative() {
            0
        } else {
            value as u16
        };
        match attr {
            0 => {
                results[index].x = value;
            }
            1 => {
                results[index].y = value;
            }
            2 => {
                results[index].width = value;
            }
            3 => {
                results[index].height = value;
            }
            _ => {}
        }
    }
    results
}

fn apply_constraints(
    ccs: &mut Vec<CassowaryConstraint>,
    constraints: &Constraints,
    var: Variable,
    total_length: u16,
) {
    for c in &constraints.0 {
        let weight = WEAK + f64::from(c.weight);
        let value = match c.unit {
            Unit::Length(v) => f64::from(v),
            Unit::Percentage(v) => f64::from(v * total_length) / 100.0,
            Unit::Ratio(n, d) => f64::from(total_length) * f64::from(n) / f64::from(d),
        };
        let operator = match c.operator {
            Operator::GreaterThanOrEqual => GE(weight),
            Operator::Equal => EQ(weight),
            Operator::LessThanOrEqual => LE(weight),
        };
        ccs.push(var | operator | value);
    }
}

/// A container used by the solver inside split
struct Element {
    x: Variable,
    y: Variable,
    width: Variable,
    height: Variable,
}

impl Element {
    fn new() -> Element {
        Element {
            x: Variable::new(),
            y: Variable::new(),
            width: Variable::new(),
            height: Variable::new(),
        }
    }

    fn left(&self) -> Variable {
        self.x
    }

    fn top(&self) -> Variable {
        self.y
    }

    fn right(&self) -> Expression {
        self.x + self.width
    }

    fn bottom(&self) -> Expression {
        self.y + self.height
    }
}

/// A simple rectangle used in the computation of the layout and to give widgets an hint about the
/// area they are supposed to render to.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Default for Rect {
    fn default() -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}

impl Rect {
    /// Creates a new rect, with width and height limited to keep the area under max u16.
    /// If clipped, aspect ratio will be preserved.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Rect {
        let max_area = u16::max_value();
        let (clipped_width, clipped_height) =
            if u32::from(width) * u32::from(height) > u32::from(max_area) {
                let aspect_ratio = f64::from(width) / f64::from(height);
                let max_area_f = f64::from(max_area);
                let height_f = (max_area_f / aspect_ratio).sqrt();
                let width_f = height_f * aspect_ratio;
                (width_f as u16, height_f as u16)
            } else {
                (width, height)
            };
        Rect {
            x,
            y,
            width: clipped_width,
            height: clipped_height,
        }
    }

    pub fn area(self) -> u16 {
        self.width * self.height
    }

    pub fn left(self) -> u16 {
        self.x
    }

    pub fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    pub fn top(self) -> u16 {
        self.y
    }

    pub fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub fn inner(self, margin: &Margin) -> Rect {
        if self.width < 2 * margin.horizontal || self.height < 2 * margin.vertical {
            Rect::default()
        } else {
            Rect {
                x: self.x + margin.horizontal,
                y: self.y + margin.vertical,
                width: self.width - 2 * margin.horizontal,
                height: self.height - 2 * margin.vertical,
            }
        }
    }

    pub fn union(self, other: Rect) -> Rect {
        let x1 = min(self.x, other.x);
        let y1 = min(self.y, other.y);
        let x2 = max(self.x + self.width, other.x + other.width);
        let y2 = max(self.y + self.height, other.y + other.height);
        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub fn intersection(self, other: Rect) -> Rect {
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.x + self.width, other.x + other.width);
        let y2 = min(self.y + self.height, other.y + other.height);
        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub fn intersects(self, other: Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn constraint_invalid_percentages() {
        Constraint::eq(Unit::Percentage(110));
    }

    #[test]
    fn test_vertical_split_by_height() {
        let target = Rect {
            x: 2,
            y: 2,
            width: 10,
            height: 10,
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::eq(10),
                Constraint::eq(10),
                Constraint::lte(5),
                Constraint::gte(1),
            ])
            .split(target);

        assert_eq!(target.height, chunks.iter().map(|r| r.height).sum::<u16>());
        chunks.windows(2).for_each(|w| assert!(w[0].y <= w[1].y));
    }

    #[test]
    fn test_rect_size_truncation() {
        for width in 256u16..300u16 {
            for height in 256u16..300u16 {
                let rect = Rect::new(0, 0, width, height);
                rect.area(); // Should not panic.
                assert!(rect.width < width || rect.height < height);
                // The target dimensions are rounded down so the math will not be too precise
                // but let's make sure the ratios don't diverge crazily.
                assert!(
                    (f64::from(rect.width) / f64::from(rect.height)
                        - f64::from(width) / f64::from(height))
                    .abs()
                        < 1.0
                )
            }
        }

        // One dimension below 255, one above. Area above max u16.
        let width = 900;
        let height = 100;
        let rect = Rect::new(0, 0, width, height);
        assert_ne!(rect.width, 900);
        assert_ne!(rect.height, 100);
        assert!(rect.width < width || rect.height < height);
    }

    #[test]
    fn test_rect_size_preservation() {
        for width in 0..256u16 {
            for height in 0..256u16 {
                let rect = Rect::new(0, 0, width, height);
                rect.area(); // Should not panic.
                assert_eq!(rect.width, width);
                assert_eq!(rect.height, height);
            }
        }

        // One dimension below 255, one above. Area below max u16.
        let rect = Rect::new(0, 0, 300, 100);
        assert_eq!(rect.width, 300);
        assert_eq!(rect.height, 100);
    }
}
