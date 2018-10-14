use std::io;

use backend::Backend;
use buffer::Buffer;
use layout::Rect;
use widgets::Widget;

/// Interface to the terminal backed by Termion
#[derive(Debug)]
pub struct Terminal<B>
where
    B: Backend,
{
    backend: B,
    /// Holds the results of the current and previous draw calls. The two are compared at the end
    /// of each draw pass to output the necessary updates to the terminal
    buffers: [Buffer; 2],
    /// Index of the current buffer in the previous array
    current: usize,
    /// Whether the cursor is currently hidden
    hidden_cursor: bool,
}

pub struct Frame<'a, B: 'a>
where
    B: Backend,
{
    terminal: &'a mut Terminal<B>,
}

impl<'a, B> Frame<'a, B>
where
    B: Backend,
{
    /// Calls the draw method of a given widget on the current buffer
    pub fn render<W>(&mut self, widget: &mut W, area: Rect)
    where
        W: Widget,
    {
        widget.draw(area, self.terminal.current_buffer_mut());
    }
}

impl<B> Drop for Terminal<B>
where
    B: Backend,
{
    fn drop(&mut self) {
        // Attempt to restore the cursor state
        if self.hidden_cursor {
            if let Err(err) = self.show_cursor() {
                error!("Failed to show the cursor: {}", err);
            }
        }
    }
}

impl<B> Terminal<B>
where
    B: Backend,
{
    /// Wrapper around Termion initialization. Each buffer is initialized with a blank string and
    /// default colors for the foreground and the background
    pub fn new(backend: B) -> io::Result<Terminal<B>> {
        let size = backend.size()?;
        Ok(Terminal {
            backend,
            buffers: [Buffer::empty(size), Buffer::empty(size)],
            current: 0,
            hidden_cursor: false,
        })
    }

    pub fn get_frame(&mut self) -> Frame<B> {
        Frame { terminal: self }
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Builds a string representing the minimal escape sequences and characters set necessary to
    /// update the UI and writes it to stdout.
    pub fn flush(&mut self) -> io::Result<()> {
        let width = self.buffers[self.current].area.width;
        let content = self.buffers[self.current]
            .content
            .iter()
            .zip(self.buffers[1 - self.current].content.iter())
            .enumerate()
            .filter_map(|(i, (c, p))| {
                if c != p {
                    let i = i as u16;
                    let x = i % width;
                    let y = i / width;
                    Some((x, y, c))
                } else {
                    None
                }
            });
        self.backend.draw(content)
    }

    /// Updates the interface so that internal buffers matches the current size of the terminal.
    /// This leads to a full redraw of the screen.
    pub fn resize(&mut self, area: Rect) -> io::Result<()> {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].reset();
        self.buffers[1 - self.current].resize(area);
        self.backend.clear()
    }

    /// Flushes the current internal state and prepares the interface for the next draw call
    pub fn draw<F>(&mut self, f: F) -> io::Result<()>
    where
        F: FnOnce(Frame<B>),
    {
        f(self.get_frame());

        // Draw to stdout
        self.flush()?;

        // Swap buffers
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;

        // Flush
        self.backend.flush()?;
        Ok(())
    }

    pub fn hide_cursor(&mut self) -> io::Result<()> {
        self.backend.hide_cursor()?;
        self.hidden_cursor = true;
        Ok(())
    }
    pub fn show_cursor(&mut self) -> io::Result<()> {
        self.backend.show_cursor()?;
        self.hidden_cursor = false;
        Ok(())
    }
    pub fn clear(&mut self) -> io::Result<()> {
        self.backend.clear()
    }
    pub fn size(&self) -> io::Result<Rect> {
        self.backend.size()
    }
}
