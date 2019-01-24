use std::io;

use crate::backend::Backend;
use crate::buffer::Cell;
use crate::layout::Rect;
use crate::style::{Color, Modifier, Style};
#[cfg(unix)]
use crate::symbols::{bar, block, line, DOT};
#[cfg(unix)]
use pancurses::ToChtype;
#[cfg(unix)]
use unicode_segmentation::UnicodeSegmentation;

pub struct CursesBackend {
    curses: easycurses::EasyCurses,
}

impl CursesBackend {
    pub fn new() -> Result<CursesBackend, String> {
        match easycurses::EasyCurses::initialize_system() {
            Some(mut curses) => {
                curses.set_echo(false);
                curses.set_input_timeout(easycurses::TimeoutMode::Never);
                curses.set_input_mode(easycurses::InputMode::RawCharacter);
                curses.set_keypad_enabled(true);
                Ok(CursesBackend { curses })
            }
            None => Err(String::from(
                "Can't initialize curses, make sure it is not running already.",
            )),
        }
    }

    pub fn get_curses_window(&self) -> &easycurses::EasyCurses {
        &self.curses
    }

    pub fn get_curses_window_mut(&mut self) -> &mut easycurses::EasyCurses {
        &mut self.curses
    }
}

impl Backend for CursesBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
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
    fn hide_cursor(&mut self) -> Result<(), io::Error> {
        self.curses
            .set_cursor_visibility(easycurses::CursorVisibility::Invisible);
        Ok(())
    }
    fn show_cursor(&mut self) -> Result<(), io::Error> {
        self.curses
            .set_cursor_visibility(easycurses::CursorVisibility::Visible);
        Ok(())
    }
    fn clear(&mut self) -> Result<(), io::Error> {
        self.curses.clear();
        // self.curses.refresh();
        Ok(())
    }
    fn size(&self) -> Result<Rect, io::Error> {
        let (nrows, ncols) = self.curses.get_row_col_count();
        Ok(Rect::new(0, 0, ncols as u16, nrows as u16))
    }
    fn flush(&mut self) -> Result<(), io::Error> {
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
        let ch = convert_to_curses_char(grapheme);
        curses.win.addch(ch);
    }
}

#[cfg(windows)]
fn draw(curses: &mut easycurses::EasyCurses, symbol: &str) {
    curses.print(symbol);
}

#[cfg(unix)]
/// Unicode to ASCII / ncurses extended characters
fn convert_to_curses_char(unicode: &str) -> pancurses::chtype {
    match unicode {
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
        block::THREE_EIGHTHS => pancurses::ACS_BLOCK(),
        block::ONE_QUATER => pancurses::ACS_BLOCK(),
        block::ONE_EIGHTH => pancurses::ACS_BLOCK(),
        bar::SEVEN_EIGHTHS => pancurses::ACS_BLOCK(),
        bar::THREE_QUATERS => pancurses::ACS_BLOCK(),
        bar::FIVE_EIGHTHS => pancurses::ACS_BLOCK(),
        bar::HALF => pancurses::ACS_BLOCK(),
        bar::THREE_EIGHTHS => pancurses::ACS_BLOCK(),
        bar::ONE_QUATER => pancurses::ACS_BLOCK(),
        bar::ONE_EIGHTH => pancurses::ACS_BLOCK(),
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
