use std::io;
use std::io::Write;
use std::collections::HashMap;

use termion;
use termion::raw::{IntoRawMode, RawTerminal};

use buffer::Buffer;
use widgets::WidgetType;
use layout::{Rect, Tree};

pub struct Terminal {
    width: u16,
    height: u16,
    stdout: RawTerminal<io::Stdout>,
    previous: HashMap<(WidgetType, Rect), u64>,
}

impl Terminal {
    pub fn new() -> Result<Terminal, io::Error> {
        let terminal = try!(termion::terminal_size());
        let stdout = try!(io::stdout().into_raw_mode());
        Ok(Terminal {
            width: terminal.0,
            height: terminal.1,
            stdout: stdout,
            previous: HashMap::new(),
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

    pub fn render(&mut self, ui: Tree) {
        debug!("Render Pass");
        let mut buffers: Vec<Buffer> = Vec::new();
        let mut previous: HashMap<(WidgetType, Rect), u64> = HashMap::new();
        for node in ui.into_iter() {
            let area = *node.buffer.area();
            match self.previous.remove(&(node.widget_type, area)) {
                Some(h) => {
                    if h == node.hash {
                        debug!("Skip {:?} at {:?}", node.widget_type, area);
                    } else {
                        debug!("Update {:?} at {:?}", node.widget_type, area);
                        buffers.push(node.buffer);
                    }
                }
                None => {
                    buffers.push(node.buffer);
                    debug!("Render {:?} at {:?}", node.widget_type, area);
                }
            }
            previous.insert((node.widget_type, area), node.hash);
        }
        for (&(t, a), _h) in &self.previous {
            buffers.insert(0, Buffer::empty(a));
            debug!("Erased {:?} at {:?}", t, a);
        }
        for buf in buffers {
            self.render_buffer(&buf);
        }
        self.previous = previous;
    }

    pub fn render_buffer(&mut self, buffer: &Buffer) {
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
