use std::io;
use std::io::Write;
use std::collections::HashMap;

use termion;
use termion::raw::{IntoRawMode, RawTerminal};

use rustbox;

use buffer::{Buffer, Cell};
use layout::{Rect, Group, split};
use widgets::Widget;
use style::{Color, Modifier, Style};

pub trait Backend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
        where I: Iterator<Item = (u16, u16, &'a Cell)>;
    fn hide_cursor(&mut self) -> Result<(), io::Error>;
    fn show_cursor(&mut self) -> Result<(), io::Error>;
    fn clear(&mut self) -> Result<(), io::Error>;
    fn size(&self) -> Result<Rect, io::Error>;
    fn flush(&mut self) -> Result<(), io::Error>;
}

pub struct TermionBackend {
    stdout: RawTerminal<io::Stdout>,
}

impl TermionBackend {
    pub fn new() -> Result<TermionBackend, io::Error> {
        let stdout = try!(io::stdout().into_raw_mode());
        Ok(TermionBackend { stdout: stdout })
    }
}

impl Backend for TermionBackend {
    /// Clears the entire screen and move the cursor to the top left of the screen
    fn clear(&mut self) -> Result<(), io::Error> {
        try!(write!(self.stdout, "{}", termion::clear::All));
        try!(write!(self.stdout, "{}", termion::cursor::Goto(1, 1)));
        try!(self.stdout.flush());
        Ok(())
    }

    /// Hides cursor
    fn hide_cursor(&mut self) -> Result<(), io::Error> {
        try!(write!(self.stdout, "{}", termion::cursor::Hide));
        try!(self.stdout.flush());
        Ok(())
    }

    /// Shows cursor
    fn show_cursor(&mut self) -> Result<(), io::Error> {
        try!(write!(self.stdout, "{}", termion::cursor::Show));
        try!(self.stdout.flush());
        Ok(())
    }

    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
        where I: Iterator<Item = (u16, u16, &'a Cell)>
    {
        let mut string = String::with_capacity(content.size_hint().0 * 3);
        let mut style = Style::default();
        let mut last_y = 0;
        let mut last_x = 0;
        let mut inst = 0;
        for (x, y, cell) in content {
            if y != last_y || x != last_x + 1 {
                string.push_str(&format!("{}", termion::cursor::Goto(x + 1, y + 1)));
                inst += 1;
            }
            last_x = x;
            last_y = y;
            if cell.style.modifier != style.modifier {
                string.push_str(&cell.style.modifier.termion_modifier());
                style.modifier = cell.style.modifier;
                if style.modifier == Modifier::Reset {
                    style.bg = Color::Reset;
                    style.fg = Color::Reset;
                }
                inst += 1;
            }
            if cell.style.fg != style.fg {
                string.push_str(&cell.style.fg.termion_fg());
                style.fg = cell.style.fg;
                inst += 1;
            }
            if cell.style.bg != style.bg {
                string.push_str(&cell.style.bg.termion_bg());
                style.bg = cell.style.bg;
                inst += 1;
            }
            string.push_str(&cell.symbol);
            inst += 1;
        }
        debug!("{} instructions outputed.", inst);
        try!(write!(self.stdout,
                    "{}{}{}{}",
                    string,
                    Color::Reset.termion_fg(),
                    Color::Reset.termion_bg(),
                    Modifier::Reset.termion_modifier()));
        Ok(())
    }

    /// Return the size of the terminal
    fn size(&self) -> Result<Rect, io::Error> {
        let terminal = try!(termion::terminal_size());
        Ok(Rect {
            x: 0,
            y: 0,
            width: terminal.0,
            height: terminal.1,
        })
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        try!(self.stdout.flush());
        Ok(())
    }
}

pub struct RustboxBackend {
    rustbox: rustbox::RustBox,
}

impl RustboxBackend {
    pub fn new() -> Result<RustboxBackend, rustbox::InitError> {
        let rustbox = try!(rustbox::RustBox::init(Default::default()));
        Ok(RustboxBackend { rustbox: rustbox })
    }

    pub fn rustbox(&self) -> &rustbox::RustBox {
        &self.rustbox
    }
}

impl Backend for RustboxBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
        where I: Iterator<Item = (u16, u16, &'a Cell)>
    {
        let mut inst = 0;
        for (x, y, cell) in content {
            inst += 1;
            self.rustbox.print(x as usize,
                               y as usize,
                               cell.style.modifier.into(),
                               cell.style.fg.into(),
                               cell.style.bg.into(),
                               &cell.symbol);
        }
        debug!("{} instructions outputed", inst);
        Ok(())
    }
    fn hide_cursor(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
    fn show_cursor(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
    fn clear(&mut self) -> Result<(), io::Error> {
        self.rustbox.clear();
        Ok(())
    }
    fn size(&self) -> Result<Rect, io::Error> {
        Ok((Rect {
            x: 0,
            y: 0,
            width: self.rustbox.width() as u16,
            height: self.rustbox.height() as u16,
        }))
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        self.rustbox.present();
        Ok(())
    }
}

/// Holds a computed layout and keeps track of its use between successive draw calls
#[derive(Debug)]
pub struct LayoutEntry {
    chunks: Vec<Rect>,
    hot: bool,
}

/// Interface to the terminal backed by Termion
pub struct Terminal<B>
    where B: Backend
{
    backend: B,
    /// Cache to prevent the layout to be computed at each draw call
    layout_cache: HashMap<(Group, Rect), LayoutEntry>,
    /// Holds the results of the current and previous draw calls. The two are compared at the end
    /// of each draw pass to output the necessary updates to the terminal
    buffers: [Buffer; 2],
    /// Index of the current buffer in the previous array
    current: usize,
}

impl<B> Terminal<B>
    where B: Backend
{
    /// Wrapper around Termion initialization. Each buffer is initialized with a blank string and
    /// default colors for the foreground and the background
    pub fn new(backend: B) -> Result<Terminal<B>, io::Error> {
        let size = try!(backend.size());
        Ok(Terminal {
            backend: backend,
            layout_cache: HashMap::new(),
            buffers: [Buffer::empty(size), Buffer::empty(size)],
            current: 0,
        })
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Check if we have already computed a layout for a given group, otherwise it creates one and
    /// add it to the layout cache. Moreover the function marks the queried entries so that we can
    /// clean outdated ones at the end of the draw call.
    pub fn compute_layout(&mut self, group: &Group, area: &Rect) -> Vec<Rect> {
        let entry = self.layout_cache
            .entry((group.clone(), area.clone()))
            .or_insert_with(|| {
                let chunks = split(area, &group.direction, group.margin, &group.sizes);
                debug!("New layout computed:\n* Group = {:?}\n* Chunks = {:?}",
                       group,
                       chunks);
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
        self.backend.draw(content)
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
        try!(self.backend.clear());
        Ok(())
    }

    /// Flushes the current internal state and prepares the interface for the next draw call
    pub fn draw(&mut self) -> Result<(), io::Error> {

        // Draw to stdout
        try!(self.flush());

        // Clean layout cache
        let hot = self.layout_cache
            .drain()
            .filter(|&(_, ref v)| v.hot == true)
            .collect::<Vec<((Group, Rect), LayoutEntry)>>();

        for (key, value) in hot {
            self.layout_cache.insert(key, value);
        }

        for (_, e) in &mut self.layout_cache {
            e.hot = false;
        }

        // Swap buffers
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;

        // Flush
        try!(self.backend.flush());
        Ok(())
    }

    pub fn hide_cursor(&mut self) -> Result<(), io::Error> {
        self.backend.hide_cursor()
    }
    pub fn show_cursor(&mut self) -> Result<(), io::Error> {
        self.backend.show_cursor()
    }
    pub fn clear(&mut self) -> Result<(), io::Error> {
        self.backend.clear()
    }
    pub fn size(&self) -> Result<Rect, io::Error> {
        self.backend.size()
    }
}
