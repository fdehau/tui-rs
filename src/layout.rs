use std::cmp::{max, min};
use std::collections::HashMap;

use cassowary::{Constraint, Expression, Solver, Variable};
use cassowary::WeightedRelation::*;
use cassowary::strength::{REQUIRED, WEAK};

use terminal::Terminal;
use backend::Backend;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    Vertical,
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
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    pub fn area(&self) -> u16 {
        self.width * self.height
    }

    pub fn left(&self) -> u16 {
        self.x
    }

    pub fn right(&self) -> u16 {
        self.x + self.width
    }

    pub fn top(&self) -> u16 {
        self.y
    }

    pub fn bottom(&self) -> u16 {
        self.y + self.height
    }

    pub fn inner(&self, margin: u16) -> Rect {
        if self.width < 2 * margin || self.height < 2 * margin {
            Rect::default()
        } else {
            Rect {
                x: self.x + margin,
                y: self.y + margin,
                width: self.width - 2 * margin,
                height: self.height - 2 * margin,
            }
        }
    }

    pub fn union(&self, other: &Rect) -> Rect {
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

    pub fn intersection(&self, other: &Rect) -> Rect {
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

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width && self.x + self.width > other.x
            && self.y < other.y + other.height && self.y + self.height > other.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Size {
    Fixed(u16),
    Percent(u16),
    Max(u16),
    Min(u16),
}

/// Wrapper function around the cassowary-rs solver to be able to split a given
/// area into smaller ones based on the preferred widths or heights and the direction.
///
/// # Examples
/// ```
/// # extern crate tui;
/// # use tui::layout::{Rect, Size, Direction, split};
///
/// # fn main() {
///     let chunks = split(&Rect{x: 2, y: 2, width: 10, height: 10},
///                        &Direction::Vertical,
///                        0,
///                        &[Size::Fixed(5), Size::Min(0)]);
///     assert_eq!(chunks, vec![Rect{x:2, y: 2, width: 10, height: 5},
///                             Rect{x: 2, y: 7, width: 10, height: 5}])
/// # }
///
/// ```
pub fn split(area: &Rect, dir: &Direction, margin: u16, sizes: &[Size]) -> Vec<Rect> {
    let mut solver = Solver::new();
    let mut vars: HashMap<Variable, (usize, usize)> = HashMap::new();
    let elements = sizes
        .iter()
        .map(|_| Element::new())
        .collect::<Vec<Element>>();
    let mut results = sizes.iter().map(|_| Rect::default()).collect::<Vec<Rect>>();
    let dest_area = area.inner(margin);
    for (i, e) in elements.iter().enumerate() {
        vars.insert(e.x, (i, 0));
        vars.insert(e.y, (i, 1));
        vars.insert(e.width, (i, 2));
        vars.insert(e.height, (i, 3));
    }
    let mut constraints: Vec<Constraint> = Vec::with_capacity(elements.len() * 4 + sizes.len() * 6);
    for elt in &elements {
        constraints.push(elt.left() | GE(REQUIRED) | f64::from(dest_area.left()));
        constraints.push(elt.top() | GE(REQUIRED) | f64::from(dest_area.top()));
        constraints.push(elt.right() | LE(REQUIRED) | f64::from(dest_area.right()));
        constraints.push(elt.bottom() | LE(REQUIRED) | f64::from(dest_area.bottom()));
    }
    if let Some(first) = elements.first() {
        constraints.push(match *dir {
            Direction::Horizontal => first.left() | EQ(REQUIRED) | f64::from(dest_area.left()),
            Direction::Vertical => first.top() | EQ(REQUIRED) | f64::from(dest_area.top()),
        });
    }
    if let Some(last) = elements.last() {
        constraints.push(match *dir {
            Direction::Horizontal => last.right() | EQ(REQUIRED) | f64::from(dest_area.right()),
            Direction::Vertical => last.bottom() | EQ(REQUIRED) | f64::from(dest_area.bottom()),
        });
    }
    match *dir {
        Direction::Horizontal => {
            for pair in elements.windows(2) {
                constraints.push((pair[0].x + pair[0].width) | EQ(REQUIRED) | pair[1].x);
            }
            for (i, size) in sizes.iter().enumerate() {
                constraints.push(elements[i].y | EQ(REQUIRED) | f64::from(dest_area.y));
                constraints.push(elements[i].height | EQ(REQUIRED) | f64::from(dest_area.height));
                constraints.push(match *size {
                    Size::Fixed(v) => elements[i].width | EQ(WEAK) | f64::from(v),
                    Size::Percent(v) => {
                        elements[i].width | EQ(WEAK) | (f64::from(v * dest_area.width) / 100.0)
                    }
                    Size::Min(v) => elements[i].width | GE(WEAK) | f64::from(v),
                    Size::Max(v) => elements[i].width | LE(WEAK) | f64::from(v),
                });
            }
        }
        Direction::Vertical => {
            for pair in elements.windows(2) {
                constraints.push((pair[0].y + pair[0].height) | EQ(REQUIRED) | pair[1].y);
            }
            for (i, size) in sizes.iter().enumerate() {
                constraints.push(elements[i].x | EQ(REQUIRED) | f64::from(dest_area.x));
                constraints.push(elements[i].width | EQ(REQUIRED) | f64::from(dest_area.width));
                constraints.push(match *size {
                    Size::Fixed(v) => elements[i].height | EQ(WEAK) | f64::from(v),
                    Size::Percent(v) => {
                        elements[i].height | EQ(WEAK) | (f64::from(v * dest_area.height) / 100.0)
                    }
                    Size::Min(v) => elements[i].height | GE(WEAK) | f64::from(v),
                    Size::Max(v) => elements[i].height | LE(WEAK) | f64::from(v),
                });
            }
        }
    }
    solver.add_constraints(&constraints).unwrap();
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

    // Fix imprecision by extending the last item a bit if necessary
    if let Some(last) = results.last_mut() {
        match *dir {
            Direction::Vertical => {
                last.height = dest_area.bottom() - last.y;
            }
            Direction::Horizontal => {
                last.width = dest_area.right() - last.x;
            }
        }
    }
    results
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

/// Describes a layout and may be used to group widgets in a specific area of the terminal
///
/// # Examples
///
/// ```
/// # extern crate tui;
/// use tui::layout::{Group, Direction, Size};
/// # fn main() {
///     Group::default()
///         .direction(Direction::Vertical)
///         .margin(0)
///         .sizes(&[Size::Percent(50), Size::Percent(50)]);
/// # }
/// ```
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Group {
    pub direction: Direction,
    pub margin: u16,
    pub sizes: Vec<Size>,
}

impl Default for Group {
    fn default() -> Group {
        Group {
            direction: Direction::Horizontal,
            margin: 0,
            sizes: Vec::new(),
        }
    }
}

impl Group {
    pub fn direction(&mut self, direction: Direction) -> &mut Group {
        self.direction = direction;
        self
    }

    pub fn margin(&mut self, margin: u16) -> &mut Group {
        self.margin = margin;
        self
    }

    pub fn sizes(&mut self, sizes: &[Size]) -> &mut Group {
        self.sizes = Vec::from(sizes);
        self
    }
    pub fn render<F, B>(&self, t: &mut Terminal<B>, area: &Rect, mut f: F)
    where
        B: Backend,
        F: FnMut(&mut Terminal<B>, &[Rect]),
    {
        let chunks = t.compute_layout(self, area);
        f(t, &chunks);
    }
}
