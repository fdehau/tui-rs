use std::iter;
use std::io;
use std::io::Write;

use termion;
use termion::raw::{IntoRawMode, RawTerminal};

use buffer::Buffer;
use layout::Rect;

pub struct Terminal {
    stdout: RawTerminal<io::Stdout>,
    width: u16,
    height: u16,
}

impl Terminal {
    pub fn new() -> Result<Terminal, io::Error> {
        let terminal = try!(termion::terminal_size());
        let stdout = try!(io::stdout().into_raw_mode());
        Ok(Terminal {
            stdout: stdout,
            width: terminal.0,
            height: terminal.1,
        })
    }

    pub fn area(&self) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }

    pub fn render(&mut self, buffer: &Buffer) {
        for (i, cell) in buffer.content().iter().enumerate() {
            let (lx, ly) = buffer.pos_of(i);
            let (x, y) = (lx + buffer.area().x, ly + buffer.area().y);
            write!(self.stdout,
                   "{}{}{}{}",
                   termion::cursor::Goto(x + 1, y + 1),
                   cell.fg.fg(),
                   cell.bg.bg(),
                   cell.symbol)
                .unwrap();
        }
        self.stdout.flush().unwrap();
    }
    pub fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        self.stdout.flush().unwrap();
    }
    pub fn hide_cursor(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Hide).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn show_cursor(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
        self.stdout.flush().unwrap();
    }
}
