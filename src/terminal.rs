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

pub struct LayoutEntry {
    chunks: Vec<Rect>,
    hot: bool,
}

pub struct Terminal {
    stdout: RawTerminal<io::Stdout>,
    layout_cache: HashMap<u64, LayoutEntry>,
    buffers: [Buffer; 2],
    current: usize,
}

impl Terminal {
    pub fn new() -> Result<Terminal, io::Error> {
        let stdout = try!(io::stdout().into_raw_mode());
        let size = try!(Terminal::size());
        Ok(Terminal {
            stdout: stdout,
            layout_cache: HashMap::new(),
            buffers: [Buffer::empty(size), Buffer::empty(size)],
            current: 0,
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
        let entry = self.layout_cache
            .entry(hash)
            .or_insert({
                let chunks = split(area, &group.direction, group.margin, &group.sizes);
                LayoutEntry {
                    chunks: chunks,
                    hot: true,
                }
            });
        entry.hot = true;
        entry.chunks.clone()
    }

    pub fn draw(&mut self) {
        let width = self.buffers[self.current].area.width;
        let mut string = String::with_capacity(self.buffers[self.current].content.len());
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        let mut last_y = 0;
        let mut last_x = 0;
        let content = self.buffers[self.current]
            .content
            .iter()
            .zip(self.buffers[1 - self.current].content.iter())
            .enumerate()
            .filter_map(|(i, (c, p))| if c != p {
                let i = i as u16;
                let x = i % width;
                let y = i / width;
                Some((x, y, c))
            } else {
                None
            });
        let mut inst = 0;
        for (x, y, cell) in content {
            if y != last_y || x != last_x + 1 {
                string.push_str(&format!("{}", termion::cursor::Goto(x + 1, y + 1)));
                inst += 1;
            }
            last_x = x;
            last_y = y;
            if cell.fg != fg {
                string.push_str(&cell.fg.fg());
                fg = cell.fg;
                inst += 1;
            }
            if cell.bg != bg {
                string.push_str(&cell.bg.bg());
                bg = cell.bg;
                inst += 1;
            }
            string.push_str(&format!("{}", cell.symbol));
            inst += 1;
        }
        string.push_str(&format!("{}{}", Color::Reset.fg(), Color::Reset.bg()));
        info!("{}", inst);
        write!(self.stdout, "{}", string).unwrap();
    }

    pub fn render<W>(&mut self, widget: &W, area: &Rect)
        where W: Widget
    {
        widget.buffer(area, &mut self.buffers[self.current]);
    }

    pub fn resize(&mut self, area: Rect) {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].resize(area);
        self.buffers[1 - self.current].reset();
        self.clear();
    }

    pub fn finish(&mut self) {

        // Draw to stdout
        self.draw();

        // Clean layout cache
        let to_remove = self.layout_cache
            .iter()
            .filter_map(|(h, e)| if !e.hot { Some(*h) } else { None })
            .collect::<Vec<u64>>();
        for h in to_remove {
            self.layout_cache.remove(&h);
        }

        // Swap buffers
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;

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
