use std::io;
use std::collections::HashMap;

use backend::Backend;
use buffer::Buffer;
use layout::{Rect, Group, split};
use widgets::Widget;

/// Holds a computed layout and keeps track of its use between successive draw calls
#[derive(Debug)]
pub struct LayoutEntry {
    chunks: Vec<Rect>,
    hot: bool,
}

/// Interface to the terminal backed by Termion
pub struct Terminal<B>
where
    B: Backend,
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
where
    B: Backend,
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
            .entry((group.clone(), *area))
            .or_insert_with(|| {
                let chunks = split(area, &group.direction, group.margin, &group.sizes);
                debug!(
                    "New layout computed:\n* Group = {:?}\n* Chunks = {:?}",
                    group,
                    chunks
                );
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
    pub fn render<W>(&mut self, widget: &mut W, area: &Rect)
    where
        W: Widget,
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
        self.backend.clear()
    }

    /// Flushes the current internal state and prepares the interface for the next draw call
    pub fn draw(&mut self) -> Result<(), io::Error> {

        // Draw to stdout
        self.flush()?;

        // Clean layout cache
        let hot = self.layout_cache
            .drain()
            .filter(|&(_, ref v)| v.hot)
            .collect::<Vec<((Group, Rect), LayoutEntry)>>();

        for (key, value) in hot {
            self.layout_cache.insert(key, value);
        }

        for e in self.layout_cache.values_mut() {
            e.hot = false;
        }

        // Swap buffers
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;

        // Flush
        self.backend.flush()?;
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
