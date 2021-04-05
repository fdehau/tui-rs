use crate::{
    buffer::{Buffer, Cell},
    layout::Rect,
    style::Style,
    widgets::Widget,
};

const LOOKUP_DIGITS: [([char; 4], [char; 4], [char; 4]); 10] = [
    (
        ['┌', '─', '─', '┐'],
        ['│', ' ', ' ', '│'],
        ['└', '─', '─', '┘'],
    ),
    (
        [' ', ' ', ' ', '╷'],
        [' ', ' ', ' ', '│'],
        [' ', ' ', ' ', '╵'],
    ),
    (
        ['╶', '─', '─', '┐'],
        ['┌', '─', '─', '┘'],
        ['└', '─', '─', '╴'],
    ),
    (
        ['╶', '─', '─', '┐'],
        [' ', '─', '─', '┤'],
        ['╶', '─', '─', '┘'],
    ),
    (
        ['╷', ' ', ' ', '╷'],
        ['└', '─', '─', '┤'],
        [' ', ' ', ' ', '╵'],
    ),
    (
        ['┌', '─', '─', '╴'],
        ['└', '─', '─', '┐'],
        ['╶', '─', '─', '┘'],
    ),
    (
        ['┌', '─', '─', '╴'],
        ['├', '─', '─', '┐'],
        ['└', '─', '─', '┘'],
    ),
    (
        ['╶', '─', '─', '┐'],
        [' ', ' ', ' ', '│'],
        [' ', ' ', ' ', '╵'],
    ),
    (
        ['┌', '─', '─', '┐'],
        ['├', '─', '─', '┤'],
        ['└', '─', '─', '┘'],
    ),
    (
        ['┌', '─', '─', '┐'],
        ['└', '─', '─', '┤'],
        ['╶', '─', '─', '┘'],
    ),
];

#[derive(Clone, Debug, Default)]
pub struct SevenSegment {
    top: Vec<Cell>,
    centre: Vec<Cell>,
    bottom: Vec<Cell>,
}

impl SevenSegment {
    /// Try to construct a seven-segment-display.
    /// Such segments can only
    pub fn new(text: &str) -> Result<Self, char> {
        let (length, _) = text.chars().try_fold(
            (0, false),
            |(length, need_space), character| match character {
                '0'..='9' | ' ' => Ok(if need_space {
                    (length + 5, true)
                } else {
                    (length + 4, true)
                }),
                ':' | ',' | '.' => Ok((length + 1, false)),
                other => Err(other),
            },
        )?;
        println!("Need: {}", length);
        Ok(text
            .chars()
            .try_fold(
                (
                    SevenSegment {
                        top: Vec::with_capacity(length),
                        centre: Vec::with_capacity(length),
                        bottom: Vec::with_capacity(length),
                    },
                    false,
                ),
                |(mut display, need_space), character| match character {
                    '0'..='9' => {
                        if need_space {
                            display.top.push(Cell::default());
                            display.centre.push(Cell::default());
                            display.bottom.push(Cell::default());
                        }
                        let number = LOOKUP_DIGITS[character as usize - b'0' as usize];
                        display.top.push(number.0[0].into());
                        display.top.push(number.0[1].into());
                        display.top.push(number.0[2].into());
                        display.top.push(number.0[3].into());
                        display.centre.push(number.1[0].into());
                        display.centre.push(number.1[1].into());
                        display.centre.push(number.1[2].into());
                        display.centre.push(number.1[3].into());
                        display.bottom.push(number.2[0].into());
                        display.bottom.push(number.2[1].into());
                        display.bottom.push(number.2[2].into());
                        display.bottom.push(number.2[3].into());
                        Ok((display, true))
                    }
                    ' ' => {
                        if need_space {
                            display.top.push(Cell::default());
                            display.centre.push(Cell::default());
                            display.bottom.push(Cell::default());
                        }
                        display.top.push(Cell::default());
                        display.top.push(Cell::default());
                        display.top.push(Cell::default());
                        display.top.push(Cell::default());
                        display.centre.push(Cell::default());
                        display.centre.push(Cell::default());
                        display.centre.push(Cell::default());
                        display.centre.push(Cell::default());
                        display.bottom.push(Cell::default());
                        display.bottom.push(Cell::default());
                        display.bottom.push(Cell::default());
                        display.bottom.push(Cell::default());
                        Ok((display, true))
                    }
                    ':' => {
                        display.top.push(Cell::default());
                        display.centre.push(Cell {
                            symbol: ":".to_owned(),
                            ..Default::default()
                        });
                        display.bottom.push(Cell::default());
                        Ok((display, false))
                    }
                    '.' | ',' => {
                        display.top.push(Cell::default());
                        display.centre.push(Cell::default());
                        display.bottom.push(Cell {
                            symbol: character.into(),
                            ..Default::default()
                        });
                        Ok((display, false))
                    }
                    error => Err(error),
                },
            )?
            .0)
    }
}

impl<'a> Widget for SevenSegment {
    fn render(&mut self, area: Rect, buffer: &mut Buffer) {
        if area.area() > 0 {
            let mut width = self.top.len();
            let other = area.width as usize;
            println!("{} vs. {}", width, other);
            if other < width {
                width = other;
            }
            let top = area.top();
            let left = area.left();
            if area.height >= 1 {
                let mut index = buffer.index_of(left, top);
                for cell in self.top.iter().take(width) {
                    buffer.content[index] = cell.to_owned();
                    index += 1;
                }
            }
            if area.height >= 2 {
                let mut index = buffer.index_of(left, top + 1);
                for cell in self.centre.iter().take(width) {
                    buffer.content[index] = cell.to_owned();
                    index += 1;
                }
            }
            if area.height >= 3 {
                let mut index = buffer.index_of(left, top + 2);
                for cell in self.bottom.iter().take(width) {
                    buffer.content[index] = cell.to_owned();
                    index += 1;
                }
            }
        }
    }
}
