use std::cmp::{min, max};
use std::collections::HashMap;

use cassowary::{Solver, Variable, Constraint};
use cassowary::WeightedRelation::*;
use cassowary::strength::{REQUIRED, WEAK};

use terminal::Terminal;

#[derive(Hash, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

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
        self.x < other.x + other.width && self.x + self.width > other.x &&
        self.y < other.y + other.height && self.y + self.height > other.y
    }
}

#[derive(Debug, Clone, Hash)]
pub enum Size {
    Fixed(u16),
    Percent(u16),
    Max(u16),
    Min(u16),
}

/// # Examples
/// ```
/// extern crate tui;
/// use tui::layout::{Rect, Size, Alignment, Direction, split};
///
/// fn main() {
///     let chunks = split(&Rect{x: 2, y: 2, width: 10, height: 10},
///                        &Direction::Vertical,
///                        &Alignment::Left,
///                        0,
///                        &[Size::Fixed(5), Size::Min(5)]);
/// }
///
/// ```
#[allow(unused_variables)]
pub fn split(area: &Rect, dir: &Direction, margin: u16, sizes: &[Size]) -> Vec<Rect> {
    let mut solver = Solver::new();
    let mut vars: HashMap<Variable, (usize, usize)> = HashMap::new();
    let elements = sizes.iter().map(|_| Element::new()).collect::<Vec<Element>>();
    let mut results = sizes.iter().map(|_| Rect::default()).collect::<Vec<Rect>>();
    let dest_area = area.inner(margin);
    for (i, e) in elements.iter().enumerate() {
        vars.insert(e.x, (i, 0));
        vars.insert(e.y, (i, 1));
        vars.insert(e.width, (i, 2));
        vars.insert(e.height, (i, 3));
    }
    let mut constraints: Vec<Constraint> = Vec::new();
    if let Some(first) = elements.first() {
        constraints.push(match *dir {
            Direction::Horizontal => first.x | EQ(REQUIRED) | dest_area.x as f64,
            Direction::Vertical => first.y | EQ(REQUIRED) | dest_area.y as f64,
        });
    }
    if let Some(last) = elements.last() {
        constraints.push(match *dir {
            Direction::Horizontal => {
                (last.x + last.width) | EQ(REQUIRED) | (dest_area.x + dest_area.width) as f64
            }
            Direction::Vertical => {
                (last.y + last.height) | EQ(REQUIRED) | (dest_area.y + dest_area.height) as f64
            }
        })
    }
    match *dir {
        Direction::Horizontal => {
            for pair in elements.windows(2) {
                constraints.push((pair[0].x + pair[0].width) | EQ(REQUIRED) | pair[1].x);
            }
            for (i, size) in sizes.iter().enumerate() {
                constraints.push(elements[i].y | EQ(REQUIRED) | dest_area.y as f64);
                constraints.push(elements[i].height | EQ(REQUIRED) | dest_area.height as f64);
                constraints.push(match *size {
                    Size::Fixed(v) => elements[i].width | EQ(WEAK) | v as f64,
                    Size::Percent(v) => {
                        elements[i].width | EQ(WEAK) | ((v * dest_area.width) as f64 / 100.0)
                    }
                    Size::Min(v) => elements[i].width | GE(WEAK) | v as f64,
                    Size::Max(v) => elements[i].width | LE(WEAK) | v as f64,
                });
            }
        }
        Direction::Vertical => {
            for pair in elements.windows(2) {
                constraints.push((pair[0].y + pair[0].height) | EQ(REQUIRED) | pair[1].y);
            }
            for (i, size) in sizes.iter().enumerate() {
                constraints.push(elements[i].x | EQ(REQUIRED) | dest_area.x as f64);
                constraints.push(elements[i].width | EQ(REQUIRED) | dest_area.width as f64);
                constraints.push(match *size {
                    Size::Fixed(v) => elements[i].height | EQ(WEAK) | v as f64,
                    Size::Percent(v) => {
                        elements[i].height | EQ(WEAK) | ((v * dest_area.height) as f64 / 100.0)
                    }
                    Size::Min(v) => elements[i].height | GE(WEAK) | v as f64,
                    Size::Max(v) => elements[i].height | LE(WEAK) | v as f64,
                });
            }
        }
    }
    solver.add_constraints(&constraints).unwrap();
    // TODO: Find a better way to handle overflow error
    for &(var, value) in solver.fetch_changes() {
        let (index, attr) = vars[&var];
        let value = value as u16;
        match attr {
            0 => {
                if value <= area.width {
                    results[index].x = value;
                }
            }
            1 => {
                if value <= area.height {
                    results[index].y = value;
                }
            }
            2 => {
                if value <= area.width {
                    results[index].width = value;
                }
            }
            3 => {
                if value <= area.height {
                    results[index].height = value;
                }
            }
            _ => {}
        }
    }
    results
}

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
}

#[derive(Hash)]
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
    pub fn render<F>(&self, t: &mut Terminal, area: &Rect, mut f: F)
        where F: FnMut(&mut Terminal, &[Rect])
    {
        let chunks = t.compute_layout(self, area);
        f(t, &chunks);
    }
}
