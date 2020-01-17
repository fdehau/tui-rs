use std::collections::HashMap;
use std::fmt::Display;
use std::iter::{self, Iterator};

use unicode_width::UnicodeWidthStr;

use cassowary::strength::{MEDIUM, REQUIRED, WEAK};
use cassowary::WeightedRelation::*;
use cassowary::{Expression, Solver};

use crate::buffer::Buffer;
use crate::layout::{Constraint, Rect};
use crate::style::Style;
use crate::widgets::{Block, Widget};

/// Holds data to be displayed in a Table widget
pub enum Row<D, I>
where
    D: Iterator<Item = I>,
    I: Display,
{
    Data(D),
    StyledData(D, Style),
}

/// A widget to display data in formatted columns
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, Table, Row};
/// # use tui::layout::Constraint;
/// # use tui::style::{Style, Color};
/// # fn main() {
/// let row_style = Style::default().fg(Color::White);
/// Table::new(
///         ["Col1", "Col2", "Col3"].into_iter(),
///         vec![
///             Row::StyledData(["Row11", "Row12", "Row13"].into_iter(), row_style),
///             Row::StyledData(["Row21", "Row22", "Row23"].into_iter(), row_style),
///             Row::StyledData(["Row31", "Row32", "Row33"].into_iter(), row_style),
///             Row::Data(["Row41", "Row42", "Row43"].into_iter())
///         ].into_iter()
///     )
///     .block(Block::default().title("Table"))
///     .header_style(Style::default().fg(Color::Yellow))
///     .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)])
///     .style(Style::default().fg(Color::White))
///     .column_spacing(1);
/// # }
/// ```
pub struct Table<'a, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T>,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<D, I>>,
{
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Base style for the widget
    style: Style,
    /// Header row for all columns
    header: H,
    /// Style for the header
    header_style: Style,
    /// Width constraints for each column
    widths: &'a [Constraint],
    /// Space between each column
    column_spacing: u16,
    /// Data to display in each row
    rows: R,
    /// Index of the selected row
    selected: Option<usize>,
    /// Style used to render the selected row
    highlight_style: Style,
    /// Symbol in front of the selected row
    highlight_symbol: Option<&'a str>,
}

impl<'a, T, H, I, D, R> Default for Table<'a, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T> + Default,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<D, I>> + Default,
{
    fn default() -> Table<'a, T, H, I, D, R> {
        Table {
            block: None,
            style: Style::default(),
            header: H::default(),
            header_style: Style::default(),
            widths: &[],
            rows: R::default(),
            column_spacing: 1,
            selected: None,
            highlight_style: Default::default(),
            highlight_symbol: None,
        }
    }
}

impl<'a, T, H, I, D, R> Table<'a, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T>,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<D, I>>,
{
    pub fn new(header: H, rows: R) -> Table<'a, T, H, I, D, R> {
        Table {
            block: None,
            style: Style::default(),
            header,
            header_style: Style::default(),
            widths: &[],
            rows,
            column_spacing: 1,
            selected: None,
            highlight_style: Style::default(),
            highlight_symbol: None,
        }
    }
    pub fn block(mut self, block: Block<'a>) -> Table<'a, T, H, I, D, R> {
        self.block = Some(block);
        self
    }

    pub fn header<II>(mut self, header: II) -> Table<'a, T, H, I, D, R>
    where
        II: IntoIterator<Item = T, IntoIter = H>,
    {
        self.header = header.into_iter();
        self
    }

    pub fn header_style(mut self, style: Style) -> Table<'a, T, H, I, D, R> {
        self.header_style = style;
        self
    }

    pub fn widths(mut self, widths: &'a [Constraint]) -> Table<'a, T, H, I, D, R> {
        assert!(
            widths.iter().all(|w| {
                match w {
                    Constraint::Percentage(p) => *p <= 100,
                    _ => true,
                }
            }),
            "Percentages should be between 0 and 100 inclusively."
        );
        self.widths = widths;
        self
    }

    pub fn rows<II>(mut self, rows: II) -> Table<'a, T, H, I, D, R>
    where
        II: IntoIterator<Item = Row<D, I>, IntoIter = R>,
    {
        self.rows = rows.into_iter();
        self
    }

    pub fn style(mut self, style: Style) -> Table<'a, T, H, I, D, R> {
        self.style = style;
        self
    }

    pub fn column_spacing(mut self, spacing: u16) -> Table<'a, T, H, I, D, R> {
        self.column_spacing = spacing;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> Table<'a, T, H, I, D, R> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, highlight_style: Style) -> Table<'a, T, H, I, D, R> {
        self.highlight_style = highlight_style;
        self
    }

    pub fn select(mut self, index: Option<usize>) -> Table<'a, T, H, I, D, R> {
        self.selected = index;
        self
    }
}

impl<'a, T, H, I, D, R> Widget for Table<'a, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T>,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<D, I>>,
{
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        // Render block if necessary and get the drawing area
        let table_area = match self.block {
            Some(ref mut b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => area,
        };

        // Set the background
        self.background(table_area, buf, self.style.bg);

        let mut solver = Solver::new();
        let mut var_indices = HashMap::new();
        let mut ccs = Vec::new();
        let mut variables = Vec::new();
        for i in 0..self.widths.len() {
            let var = cassowary::Variable::new();
            variables.push(var);
            var_indices.insert(var, i);
        }
        for (i, constraint) in self.widths.iter().enumerate() {
            ccs.push(variables[i] | GE(WEAK) | 0.);
            ccs.push(match *constraint {
                Constraint::Length(v) => variables[i] | EQ(MEDIUM) | f64::from(v),
                Constraint::Percentage(v) => {
                    variables[i] | EQ(WEAK) | (f64::from(v * area.width) / 100.0)
                }
                Constraint::Ratio(n, d) => {
                    variables[i] | EQ(WEAK) | (f64::from(area.width) * f64::from(n) / f64::from(d))
                }
                Constraint::Min(v) => variables[i] | GE(WEAK) | f64::from(v),
                Constraint::Max(v) => variables[i] | LE(WEAK) | f64::from(v),
            })
        }
        solver
            .add_constraint(
                variables
                    .iter()
                    .fold(Expression::from_constant(0.), |acc, v| acc + *v)
                    | LE(REQUIRED)
                    | f64::from(
                        area.width - 2 - (self.column_spacing * (variables.len() as u16 - 1)),
                    ),
            )
            .unwrap();
        solver.add_constraints(&ccs).unwrap();
        let mut solved_widths = vec![0; variables.len()];
        for &(var, value) in solver.fetch_changes() {
            let index = var_indices[&var];
            let value = if value.is_sign_negative() {
                0
            } else {
                value as u16
            };
            solved_widths[index] = value
        }

        let mut y = table_area.top();
        let mut x = table_area.left();

        // Retrieve highlight symbol if one
        let highlight_symbol = self.highlight_symbol.unwrap_or("").to_owned();
        let blank_symbol = iter::repeat(" ")
            .take(highlight_symbol.width())
            .collect::<String>();

        // Draw header
        if y < table_area.bottom() {
            for (i, (w, t)) in solved_widths.iter().zip(self.header.by_ref()).enumerate() {
                let s = if i == 0 {
                    format!("{}{}", blank_symbol, t)
                } else {
                    format!("{}", t)
                };
                buf.set_stringn(x, y, s, *w as usize, self.header_style);
                x += *w + self.column_spacing;
            }
        }
        y += 2;

        // Determine offset needed to display selected item
        let offset = if let Some(selected) = self.selected {
            let window_height = (table_area.bottom() - y) as usize;
            if selected >= window_height {
                selected - window_height + 1
            } else {
                0
            }
        } else {
            0
        };

        // Draw rows
        let default_style = Style::default();
        if y < table_area.bottom() {
            let remaining = (table_area.bottom() - y) as usize;
            for (i, row) in self.rows.by_ref().skip(offset).take(remaining).enumerate() {
                let (data, style, symbol) = match row {
                    Row::Data(d) | Row::StyledData(d, _)
                        if Some(i) == self.selected.map(|s| s - offset) =>
                    {
                        (d, self.highlight_style, &highlight_symbol)
                    }
                    Row::Data(d) => (d, default_style, &blank_symbol),
                    Row::StyledData(d, s) => (d, s, &blank_symbol),
                };
                x = table_area.left();
                for (c, (w, elt)) in solved_widths.iter().zip(data).enumerate() {
                    let s = if c == 0 {
                        format!("{}{}", symbol, elt)
                    } else {
                        format!("{}", elt)
                    };
                    buf.set_stringn(x, y + i as u16, s, *w as usize, style);
                    x += *w + self.column_spacing;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn table_invalid_percentages() {
        Table::new([""].iter(), vec![Row::Data([""].iter())].into_iter())
            .widths(&[Constraint::Percentage(110)]);
    }
}
