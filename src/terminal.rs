use std::io;
use std::io::Write;
use std::collections::HashMap;

use termion;
use termion::raw::{IntoRawMode, RawTerminal};

use buffer::Buffer;
use widgets::Widget;
use layout::Rect;
use util::hash;

pub struct Terminal {
    stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn new() -> Result<Terminal, io::Error> {
        let stdout = try!(io::stdout().into_raw_mode());
        Ok(Terminal { stdout: stdout })
    }

    pub fn size() -> Result<Rect, io::Error> {
        let terminal = try!(termion::terminal_size());
        Ok(Rect {
            x: 0,
            y: 0,
            width: terminal.0,
            height: terminal.1,
        })
    }

    pub fn render_buffer(&mut self, buffer: Buffer) {
        for (i, cell) in buffer.content().iter().enumerate() {
            let (lx, ly) = buffer.pos_of(i).unwrap();
            let (x, y) = (lx + buffer.area().x, ly + buffer.area().y);
            if cell.symbol != "" {
                write!(self.stdout,
                       "{}{}{}{}",
                       termion::cursor::Goto(x + 1, y + 1),
                       cell.fg.fg(),
                       cell.bg.bg(),
                       cell.symbol)
                    .unwrap();
            }
        }
        self.stdout.flush().unwrap();
    }
    pub fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
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
