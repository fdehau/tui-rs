use std::io;
use std::io::Write;
use std::collections::HashMap;

use termion;
use termion::raw::{IntoRawMode, RawTerminal};

use buffer::Buffer;
use widgets::WidgetType;
use layout::{Rect, Tree};

pub struct Terminal {
    stdout: RawTerminal<io::Stdout>,
    cache: HashMap<(WidgetType, Rect), u64>,
}

impl Terminal {
    pub fn new() -> Result<Terminal, io::Error> {
        let stdout = try!(io::stdout().into_raw_mode());
        Ok(Terminal {
            stdout: stdout,
            cache: HashMap::new(),
        })
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

    pub fn render(&mut self, ui: Tree) {
        debug!("Render Pass");
        let mut buffers: Vec<Buffer> = Vec::new();
        let mut cache: HashMap<(WidgetType, Rect), u64> = HashMap::new();
        for node in ui {
            let area = *node.buffer.area();
            match self.cache.remove(&(node.widget_type, area)) {
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
            cache.insert((node.widget_type, area), node.hash);
        }
        for &(t, a) in self.cache.keys() {
            buffers.insert(0, Buffer::empty(a));
            debug!("Erased {:?} at {:?}", t, a);
        }
        for buf in buffers {
            self.render_buffer(&buf);
        }
        self.cache = cache;
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
