use std::io;
use std::io::Write;
use std::collections::HashMap;

use termion;
use termion::raw::{IntoRawMode, RawTerminal};

use buffer::Buffer;
use layout::{Rect, Group, split};
use widgets::Widget;
use style::Color;
use util::hash;

pub struct Terminal {
    stdout: RawTerminal<io::Stdout>,
    layout_cache: HashMap<u64, Vec<Rect>>,
    layout_queue: Vec<(u64, Vec<Rect>)>,
    previous: Buffer,
    current: Buffer,
}

impl Terminal {
    pub fn new() -> Result<Terminal, io::Error> {
        let stdout = try!(io::stdout().into_raw_mode());
        let size = try!(Terminal::size());
        Ok(Terminal {
            stdout: stdout,
            layout_cache: HashMap::new(),
            layout_queue: Vec::new(),
            previous: Buffer::empty(size),
            current: Buffer::empty(size),
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

    pub fn compute_layout(&mut self, group: &Group, area: &Rect) -> Vec<Rect> {
        let hash = hash(group, area);
        let chunks = match self.layout_cache.get(&hash) {
            Some(chunks) => chunks.clone(),
            None => split(area, &group.direction, group.margin, &group.sizes),
        };
        self.layout_queue.push((hash, chunks.clone()));
        chunks
    }

    pub fn draw(&mut self) {
        let width = self.current.area.width;
        let mut string = String::with_capacity(self.current.content.len());
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        let content = self.current
            .content
            .iter()
            .zip(self.previous.content.iter())
            .enumerate()
            .filter_map(|(i, (c, p))| if c != p {
                let i = i as u16;
                let x = i % width;
                let y = i / width;
                Some((x, y, c))
            } else {
                None
            });
        for (x, y, cell) in content {
            string.push_str(&format!("{}", termion::cursor::Goto(x + 1, y + 1)));
            if cell.fg != fg {
                string.push_str(&cell.fg.fg());
                fg = cell.fg;
            }
            if cell.bg != bg {
                string.push_str(&cell.bg.bg());
                bg = cell.bg;
            }
            string.push_str(&format!("{}", cell.symbol));
        }
        string.push_str(&format!("{}{}", Color::Reset.fg(), Color::Reset.bg()));
        info!("{}", string.len());
        write!(self.stdout, "{}", string).unwrap();
    }

    pub fn render<W>(&mut self, widget: &W, area: &Rect)
        where W: Widget
    {
        widget.buffer(area, &mut self.current);
    }

    pub fn resize(&mut self, area: Rect) {
        self.current.resize(area);
        self.previous.resize(area);
        self.previous.reset();
        self.clear();
    }

    pub fn finish(&mut self) {

        // Draw to stdout
        self.draw();

        // Update layout cache
        self.layout_cache.clear();
        for (hash, chunks) in self.layout_queue.drain(..) {
            self.layout_cache.insert(hash, chunks);
        }

        // Swap buffers
        for (i, c) in self.current.content.iter().enumerate() {
            self.previous.content[i] = c.clone();
        }
        self.current.reset();

        // Flush
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
