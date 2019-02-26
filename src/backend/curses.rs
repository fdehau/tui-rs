use std::io;

use crate::backend::Backend;
use crate::buffer::Cell;
use crate::layout::Rect;
use crate::style::{Color, Modifier, Style};
use crate::symbols::{bar, block};
#[cfg(unix)]
use crate::symbols::{line, DOT};
#[cfg(unix)]
use pancurses::ToChtype;
use unicode_segmentation::UnicodeSegmentation;

pub struct CursesBackend {
    curses: easycurses::EasyCurses,
}

impl CursesBackend {
    pub fn new() -> Option<CursesBackend> {
        let curses = easycurses::EasyCurses::initialize_system()?;
        Some(CursesBackend { curses })
    }

    pub fn with_curses(curses: easycurses::EasyCurses) -> CursesBackend {
        CursesBackend { curses }
    }

    pub fn get_curses(&self) -> &easycurses::EasyCurses {
        &self.curses
    }

    pub fn get_curses_mut(&mut self) -> &mut easycurses::EasyCurses {
        &mut self.curses
    }
}

impl Backend for CursesBackend {
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let mut last_col = 0;
        let mut last_row = 0;
        let mut style = Style {
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::Reset,
        };
        let mut curses_style = CursesStyle {
            fg: easycurses::Color::White,
            bg: easycurses::Color::Black,
            attribute: pancurses::Attribute::Normal,
        };
        let mut update_color = false;
        for (col, row, cell) in content {
            // eprintln!("{:?}", cell);
            if row != last_row || col != last_col + 1 {
                self.curses.move_rc(row as i32, col as i32);
            }
            last_col = col;
            last_row = row;
            if cell.style.modifier != style.modifier {
                if curses_style.attribute != pancurses::Attribute::Normal {
                    self.curses.win.attroff(curses_style.attribute);
                }
                let attribute: pancurses::Attribute = cell.style.modifier.into();
                self.curses.win.attron(attribute);
                curses_style.attribute = attribute;
                style.modifier = cell.style.modifier;
            };
            if cell.style.fg != style.fg {
                update_color = true;
                if let Some(ccolor) = cell.style.fg.into() {
                    style.fg = cell.style.fg;
                    curses_style.fg = ccolor;
                } else {
                    style.fg = Color::White;
                    curses_style.fg = easycurses::Color::White;
                }
            };
            if cell.style.bg != style.bg {
                update_color = true;
                if let Some(ccolor) = cell.style.bg.into() {
                    style.bg = cell.style.bg;
                    curses_style.bg = ccolor;
                } else {
                    style.bg = Color::Black;
                    curses_style.bg = easycurses::Color::Black;
                }
            };
            if update_color {
                self.curses
                    .set_color_pair(easycurses::ColorPair::new(curses_style.fg, curses_style.bg));
            };
            update_color = false;
            draw(&mut self.curses, cell.symbol.as_str());
        }
        self.curses.win.attrset(pancurses::Attribute::Normal);
        self.curses.set_color_pair(easycurses::ColorPair::new(
            easycurses::Color::White,
            easycurses::Color::Black,
        ));
        Ok(())
    }
    fn hide_cursor(&mut self) -> io::Result<()> {
        self.curses
            .set_cursor_visibility(easycurses::CursorVisibility::Invisible);
        Ok(())
    }
    fn show_cursor(&mut self) -> io::Result<()> {
        self.curses
            .set_cursor_visibility(easycurses::CursorVisibility::Visible);
        Ok(())
    }
    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        let (x, y) = self.curses.get_cursor_rc();
        Ok((x as u16, y as u16))
    }
    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.curses.move_rc(x as i32, y as i32);
        Ok(())
    }
    fn clear(&mut self) -> io::Result<()> {
        self.curses.clear();
        // self.curses.refresh();
        Ok(())
    }
    fn size(&self) -> Result<Rect, io::Error> {
        let (nrows, ncols) = self.curses.get_row_col_count();
        Ok(Rect::new(0, 0, ncols as u16, nrows as u16))
    }
    fn flush(&mut self) -> io::Result<()> {
        self.curses.refresh();
        Ok(())
    }
}

struct CursesStyle {
    fg: easycurses::Color,
    bg: easycurses::Color,
    attribute: pancurses::Attribute,
}

#[cfg(unix)]
/// Deals with lack of unicode support for ncurses on unix
fn draw(curses: &mut easycurses::EasyCurses, symbol: &str) {
    for grapheme in symbol.graphemes(true) {
        let ch = match grapheme {
            line::TOP_RIGHT => pancurses::ACS_URCORNER(),
            line::VERTICAL => pancurses::ACS_VLINE(),
            line::HORIZONTAL => pancurses::ACS_HLINE(),
            line::TOP_LEFT => pancurses::ACS_ULCORNER(),
            line::BOTTOM_RIGHT => pancurses::ACS_LRCORNER(),
            line::BOTTOM_LEFT => pancurses::ACS_LLCORNER(),
            line::VERTICAL_LEFT => pancurses::ACS_RTEE(),
            line::VERTICAL_RIGHT => pancurses::ACS_LTEE(),
            line::HORIZONTAL_DOWN => pancurses::ACS_TTEE(),
            line::HORIZONTAL_UP => pancurses::ACS_BTEE(),
            block::FULL => pancurses::ACS_BLOCK(),
            block::SEVEN_EIGHTHS => pancurses::ACS_BLOCK(),
            block::THREE_QUATERS => pancurses::ACS_BLOCK(),
            block::FIVE_EIGHTHS => pancurses::ACS_BLOCK(),
            block::HALF => pancurses::ACS_BLOCK(),
            block::THREE_EIGHTHS => ' ' as u32,
            block::ONE_QUATER => ' ' as u32,
            block::ONE_EIGHTH => ' ' as u32,
            bar::SEVEN_EIGHTHS => pancurses::ACS_BLOCK(),
            bar::THREE_QUATERS => pancurses::ACS_BLOCK(),
            bar::FIVE_EIGHTHS => pancurses::ACS_BLOCK(),
            bar::HALF => pancurses::ACS_BLOCK(),
            bar::THREE_EIGHTHS => pancurses::ACS_S9(),
            bar::ONE_QUATER => pancurses::ACS_S9(),
            bar::ONE_EIGHTH => pancurses::ACS_S9(),
            DOT => pancurses::ACS_BULLET(),
            unicode_char => {
                if unicode_char.is_ascii() {
                    let mut chars = unicode_char.chars();
                    if let Some(ch) = chars.next() {
                        ch.to_chtype()
                    } else {
                        pancurses::ACS_BLOCK()
                    }
                } else {
                    pancurses::ACS_BLOCK()
                }
            }
        };
        curses.win.addch(ch);
    }
}

#[cfg(windows)]
fn draw(curses: &mut easycurses::EasyCurses, symbol: &str) {
    for grapheme in symbol.graphemes(true) {
        let ch = match grapheme {
            block::SEVEN_EIGHTHS => block::FULL,
            block::THREE_QUATERS => block::FULL,
            block::FIVE_EIGHTHS => block::HALF,
            block::THREE_EIGHTHS => block::HALF,
            block::ONE_QUATER => block::HALF,
            block::ONE_EIGHTH => " ",
            bar::SEVEN_EIGHTHS => bar::FULL,
            bar::THREE_QUATERS => bar::FULL,
            bar::FIVE_EIGHTHS => bar::HALF,
            bar::THREE_EIGHTHS => bar::HALF,
            bar::ONE_QUATER => bar::HALF,
            bar::ONE_EIGHTH => " ",
            ch => ch,
        };
        // curses.win.addch(ch);
        curses.print(ch);
    }
}

impl From<Color> for Option<easycurses::Color> {
    fn from(color: Color) -> Option<easycurses::Color> {
        match color {
            Color::Reset => None,
            Color::Black => Some(easycurses::Color::Black),
            Color::Red | Color::LightRed => Some(easycurses::Color::Red),
            Color::Green | Color::LightGreen => Some(easycurses::Color::Green),
            Color::Yellow | Color::LightYellow => Some(easycurses::Color::Yellow),
            Color::Magenta | Color::LightMagenta => Some(easycurses::Color::Magenta),
            Color::Cyan | Color::LightCyan => Some(easycurses::Color::Cyan),
            Color::White | Color::Gray | Color::DarkGray => Some(easycurses::Color::White),
            Color::Blue | Color::LightBlue => Some(easycurses::Color::Blue),
            Color::Rgb(_, _, _) => None,
        }
    }
}

impl From<Modifier> for pancurses::Attribute {
    fn from(modifier: Modifier) -> pancurses::Attribute {
        match modifier {
            Modifier::Blink => pancurses::Attribute::Blink,
            Modifier::Bold => pancurses::Attribute::Bold,
            Modifier::CrossedOut => pancurses::Attribute::Strikeout,
            Modifier::Faint => pancurses::Attribute::Dim,
            Modifier::Invert => pancurses::Attribute::Reverse,
            Modifier::Italic => pancurses::Attribute::Italic,
            Modifier::Underline => pancurses::Attribute::Underline,
            _ => pancurses::Attribute::Normal,
        }
    }
}
