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

/// Holds a computed layout and keeps track of its use between successive draw calls
#[derive(Debug)]
pub struct LayoutEntry {
    chunks: Vec<Rect>,
    hot: bool,
}

/// Interface to the terminal backed by Termion
pub struct Terminal {
    /// Raw mode termion terminal
    stdout: RawTerminal<io::Stdout>,
    /// Cache to prevent the layout to be computed at each draw call
    layout_cache: HashMap<u64, LayoutEntry>,
    /// Holds the results of the current and previous draw calls. The two are compared at the end
    /// of each draw pass to output the necessary updates to the terminal
    buffers: [Buffer; 2],
    /// Index of the current buffer in the previous array
    current: usize,
}

impl Terminal {
    /// Wrapper around Termion initialization. Each buffer is initialized with a blank string and
    /// default colors for the foreground and the background
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

    /// Return the size of the terminal
    pub fn size() -> Result<Rect, io::Error> {
        let terminal = try!(termion::terminal_size());
        Ok(Rect {
            x: 0,
            y: 0,
            width: terminal.0,
            height: terminal.1,
        })
    }

    /// Check if we have already computed a layout for a given group, otherwise it creates one and
    /// add it to the layout cache. Moreover the function marks the queried entries so that we can
    /// clean outdated ones at the end of the draw call.
    pub fn compute_layout(&mut self, group: &Group, area: &Rect) -> Vec<Rect> {
        let hash = hash(group, area);
        let entry = self.layout_cache
            .entry(hash)
            .or_insert_with(|| {
                let chunks = split(area, &group.direction, group.margin, &group.sizes);
                debug!("New layout computed:\n* Group = {:?}\n* Chunks = {:?}\n* Hash = {}",
                       group,
                       chunks,
                       hash);
                LayoutEntry {
                    chunks: chunks,
                    hot: true,
                }
            });
        entry.hot = true;
        entry.chunks.clone()
    }

    /// Builds a string representing the minimal escape sequences and characters set necessary to
    /// update the UI and writes it to stdout.
    pub fn flush(&mut self) -> Result<(), io::Error> {
        let width = self.buffers[self.current].area.width;
        let mut string = String::with_capacity(self.buffers[self.current].content.len() * 3);
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
            string.push_str(&cell.symbol);
            inst += 1;
        }
        debug!("{} instructions outputed.", inst);
        try!(write!(self.stdout,
                    "{}{}{}",
                    string,
                    Color::Reset.fg(),
                    Color::Reset.bg()));
        Ok(())
    }

    /// Calls the draw method of a given widget on the current buffer
    pub fn render<W>(&mut self, widget: &W, area: &Rect)
        where W: Widget
    {
        widget.draw(area, &mut self.buffers[self.current]);
    }

    /// Updates the interface so that internal buffers matches the current size of the terminal.
    /// This leads to a full redraw of the screen.
    pub fn resize(&mut self, area: Rect) -> Result<(), io::Error> {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].resize(area);
        self.buffers[1 - self.current].reset();
        self.layout_cache.clear();
        try!(self.clear());
        Ok(())
    }

    /// Flushes the current internal state and prepares the interface for the next draw call
    pub fn draw(&mut self) -> Result<(), io::Error> {

        // Draw to stdout
        try!(self.flush());

        // Clean layout cache
        let to_remove = self.layout_cache
            .iter()
            .filter_map(|(h, e)| if !e.hot { Some(*h) } else { None })
            .collect::<Vec<u64>>();

        for h in to_remove {
            self.layout_cache.remove(&h);
        }

        for (_, e) in &mut self.layout_cache {
            e.hot = false;
        }

        // Swap buffers
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;

        // Flush
        try!(self.stdout.flush());
        Ok(())
    }

    /// Clears the entire screen and move the cursor to the top left of the screen
    pub fn clear(&mut self) -> Result<(), io::Error> {
        try!(write!(self.stdout, "{}", termion::clear::All));
        try!(write!(self.stdout, "{}", termion::cursor::Goto(1, 1)));
        try!(self.stdout.flush());
        Ok(())
    }

    /// Hides cursor
    pub fn hide_cursor(&mut self) -> Result<(), io::Error> {
        try!(write!(self.stdout, "{}", termion::cursor::Hide));
        try!(self.stdout.flush());
        Ok(())
    }

    /// Shows cursor
    pub fn show_cursor(&mut self) -> Result<(), io::Error> {
        try!(write!(self.stdout, "{}", termion::cursor::Show));
        try!(self.stdout.flush());
        Ok(())
    }
}
