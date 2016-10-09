use std::cmp::{min, max};
use std::collections::HashMap;

use cassowary::{Solver, Variable, Constraint};
use cassowary::WeightedRelation::*;
use cassowary::strength::{WEAK, MEDIUM, STRONG, REQUIRED};

use buffer::Buffer;

pub enum Alignment {
    Top,
    Left,
    Center,
    Bottom,
    Right,
}

pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
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
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn area(&self) -> u16 {
        self.width * self.height
    }

    pub fn inner(&self, spacing: u16) -> Rect {
        if self.width - spacing < 0 || self.height - spacing < 0 {
            Rect::default()
        } else {
            Rect {
                x: self.x + spacing,
                y: self.y + spacing,
                width: self.width - 2 * spacing,
                height: self.height - 2 * spacing,
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

#[derive(Debug, Clone)]
pub enum Size {
    Fixed(f64),
    Percent(f64),
}

/// # Examples
/// ```
/// extern crate tui;
/// use tui::layout::{Rect, Size, Alignment, Direction, split};
///
/// fn main() {
///     let chunks = split(&Rect{x: 2, y: 2, width: 10, height: 10}, Direction::Vertical,
///     Alignment::Left, &[Size::Fixed(5.0), Size::Percent(80.0)]);
/// }
///
/// ```
pub fn split(area: &Rect, dir: &Direction, align: &Alignment, sizes: &[Size]) -> Vec<Rect> {
    let mut solver = Solver::new();
    let mut vars: HashMap<Variable, (usize, usize)> = HashMap::new();
    let elements = sizes.iter().map(|e| Element::new()).collect::<Vec<Element>>();
    let mut results = sizes.iter().map(|e| Rect::default()).collect::<Vec<Rect>>();
    for (i, e) in elements.iter().enumerate() {
        vars.insert(e.x, (i, 0));
        vars.insert(e.y, (i, 1));
        vars.insert(e.width, (i, 2));
        vars.insert(e.height, (i, 3));
    }
    let mut constraints: Vec<Constraint> = Vec::new();
    if let Some(size) = sizes.first() {
        constraints.push(match *dir {
            Direction::Horizontal => elements[0].x | EQ(REQUIRED) | area.x as f64,
            Direction::Vertical => elements[0].y | EQ(REQUIRED) | area.y as f64,
        })
    }
    if let Some(size) = sizes.last() {
        let last = elements.last().unwrap();
        constraints.push(match *dir {
            Direction::Horizontal => {
                last.x + last.width | EQ(REQUIRED) | (area.x + area.width) as f64
            }
            Direction::Vertical => {
                last.y + last.height | EQ(REQUIRED) | (area.y + area.height) as f64
            }
        })
    }
    match *dir {
        Direction::Horizontal => {
            for pair in elements.windows(2) {
                constraints.push(pair[0].x + pair[0].width | LE(REQUIRED) | pair[1].x);
            }
            for (i, size) in sizes.iter().enumerate() {
                let cs = [elements[i].y | EQ(REQUIRED) | area.y as f64,
                          elements[i].height | EQ(REQUIRED) | area.height as f64,
                          match *size {
                              Size::Fixed(f) => elements[i].width | EQ(REQUIRED) | f,
                              Size::Percent(p) => {
                                  elements[i].width | EQ(WEAK) | area.width as f64 * p / 100.0
                              }
                          }];
                constraints.extend_from_slice(&cs);
            }
        }
        Direction::Vertical => {
            for pair in elements.windows(2) {
                constraints.push(pair[0].y + pair[0].height | LE(REQUIRED) | pair[1].y);
            }
            for (i, size) in sizes.iter().enumerate() {
                let cs = [elements[i].x | EQ(REQUIRED) | area.x as f64,
                          elements[i].width | EQ(REQUIRED) | area.width as f64,
                          match *size {
                              Size::Fixed(f) => elements[i].height | EQ(REQUIRED) | f,
                              Size::Percent(p) => {
                                  elements[i].height | EQ(WEAK) | area.height as f64 * p / 100.0
                              }
                          }];
                constraints.extend_from_slice(&cs);
            }
        }
    }
    solver.add_constraints(&constraints).unwrap();
    for &(var, value) in solver.fetch_changes() {
        let (index, attr) = vars[&var];
        match attr {
            0 => {
                results[index].x = value as u16;
            }
            1 => {
                results[index].y = value as u16;
            }
            2 => {
                results[index].width = value as u16;
            }
            3 => {
                results[index].height = value as u16;
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

pub struct Group {
    direction: Direction,
    alignment: Alignment,
    chunks: Vec<Size>,
}

impl Default for Group {
    fn default() -> Group {
        Group {
            direction: Direction::Horizontal,
            alignment: Alignment::Left,
            chunks: Vec::new(),
        }
    }
}

impl Group {
    pub fn direction(&mut self, direction: Direction) -> &mut Group {
        self.direction = direction;
        self
    }

    pub fn alignment(&mut self, alignment: Alignment) -> &mut Group {
        self.alignment = alignment;
        self
    }

    pub fn chunks(&mut self, chunks: &[Size]) -> &mut Group {
        self.chunks = Vec::from(chunks);
        self
    }
    pub fn render<F>(&self, area: &Rect, f: F) -> Buffer
        where F: Fn(&[Rect]) -> Vec<Buffer>
    {
        let chunks = split(area, &self.direction, &self.alignment, &self.chunks);
        let results = f(&chunks);
        let mut result = results[0].clone();
        for r in results.iter().skip(1) {
            result.merge(&r);
        }
        result
    }
}
