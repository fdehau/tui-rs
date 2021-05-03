use crate::{
    backend::Backend,
    buffer::Buffer,
    layout::Rect,
    widgets::{RenderContext, Widget},
};
use std::{any::Any, collections::HashMap, hash::Hash, io, panic::Location};

#[derive(Debug, Clone, PartialEq)]
/// UNSTABLE
enum ResizeBehavior {
    Fixed,
    Auto,
}

#[derive(Debug, Clone, PartialEq)]
/// UNSTABLE
pub struct Viewport {
    area: Rect,
    resize_behavior: ResizeBehavior,
}

impl Viewport {
    /// UNSTABLE
    pub fn fixed(area: Rect) -> Viewport {
        Viewport {
            area,
            resize_behavior: ResizeBehavior::Fixed,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CallLocation(&'static Location<'static>);

impl CallLocation {
    fn as_ptr(&self) -> *const Location<'static> {
        self.0
    }
}

impl Hash for CallLocation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state)
    }
}

impl PartialEq for CallLocation {
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl Eq for CallLocation {}

/// StateEntry is used to link a [`Frame::render_widget`] to [`Widget::State`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct StateKey {
    /// Location of the call to [`Frame::render_widget`].
    call_location: CallLocation,
    /// Optional id that can be used to have multiple widgets state at the same call location.
    id: Option<String>,
}

/// StateEntry holds the state of a [`Widget`].
struct StateEntry {
    /// State of a [`Widget`].
    state: Box<dyn Any>,
    /// Index of the frame where the state was used for the last time.
    frame_index: usize,
}

#[derive(Debug, Clone, PartialEq)]
/// Options to pass to [`Terminal::with_options`]
pub struct TerminalOptions {
    /// Viewport used to draw to the terminal
    pub viewport: Viewport,
}

/// Interface to the terminal backed by Termion
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
    /// Viewport
    viewport: Viewport,
    /// State of the widgets rendered in the previous frame.
    widget_states: HashMap<StateKey, StateEntry>,
    /// Index of the current frame. Incremented each time [`Terminal::draw`] is called and wraps
    /// when it is greater than [`std::usize::MAX`].
    frame_index: usize,
}

/// Represents a consistent terminal interface for rendering.
pub struct Frame<'a, B: 'a>
where
    B: Backend,
{
    terminal: &'a mut Terminal<B>,

    /// Where should the cursor be after drawing this frame?
    ///
    /// If `None`, the cursor is hidden and its position is controlled by the backend. If `Some((x,
    /// y))`, the cursor is shown and placed at `(x, y)` after the call to `Terminal::draw()`.
    cursor_position: Option<(u16, u16)>,
}

/// RenderArgs are the arguments required to render a [`Widget`].
pub struct RenderArgs {
    /// Area where the widget will be rendered.
    area: Rect,
    /// Optional id that can be used to uniquely identify the provided [`Widget`].
    id: Option<String>,
}

impl From<Rect> for RenderArgs {
    fn from(area: Rect) -> RenderArgs {
        RenderArgs { area, id: None }
    }
}

impl RenderArgs {
    /// Set the [`Widget`] id.
    pub fn id<S>(mut self, id: S) -> Self
    where
        S: Into<String>,
    {
        self.id = Some(id.into());
        self
    }
}

impl<'a, B> Frame<'a, B>
where
    B: Backend,
{
    /// Terminal size, guaranteed not to change when rendering.
    pub fn size(&self) -> Rect {
        self.terminal.viewport.area
    }

    /// Render a [`Widget`] to the current buffer using [`Widget::render`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use std::io;
    /// # use tui::Terminal;
    /// # use tui::backend::TermionBackend;
    /// # use tui::layout::Rect;
    /// # use tui::widgets::Block;
    /// # let stdout = io::stdout();
    /// # let backend = TermionBackend::new(stdout);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// let block = Block::default();
    /// let area = Rect::new(0, 0, 5, 5);
    /// let mut frame = terminal.get_frame();
    /// frame.render_widget(block, area);
    /// ```
    ///
    /// If you happen to render two or more widgets using the same render call, you may want to
    /// associate them with a unique id so they do not share any internal state.
    ///
    /// For example, let say your app shows a list of songs of a given album:
    /// ```rust,no_run
    /// # use std::{collections::HashMap, io};
    /// # use tui::{Terminal, RenderArgs};
    /// # use tui::backend::TermionBackend;
    /// # use tui::layout::Rect;
    /// # use tui::widgets::{Block, List, ListItem};
    /// # let stdout = io::stdout();
    /// # let backend = TermionBackend::new(stdout);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// struct App {
    ///    albums: HashMap<String, Vec<String>>,
    ///    selected_album: String
    /// }
    /// # let app = App {
    /// #    albums: HashMap::new(),
    /// #    selected_album: String::new(),
    /// # };
    /// terminal.draw(|f| {
    ///     let songs: Vec<ListItem> = app.albums[&app.selected_album]
    ///         .iter()
    ///         .map(|song| ListItem::new(song.as_ref()))
    ///         .collect();
    ///     let song_list = List::new(songs)
    ///         .block(Block::default().title(app.selected_album.as_ref()));
    ///     // Giving a unique id here makes sure the list state is reset whenever the album
    ///     // currently displayed changes.
    ///     let args = RenderArgs::from(f.size()).id(app.selected_album.clone());
    ///     f.render_widget(song_list, args);
    /// });
    /// ```
    #[track_caller]
    pub fn render_widget<W, R>(&mut self, widget: W, args: R)
    where
        W: Widget,
        W::State: 'static + Default,
        R: Into<RenderArgs>,
    {
        // Fetch the previous internal state of the widget (or initialize it with a default value).
        let args: RenderArgs = args.into();
        let location = Location::caller();
        let key = StateKey {
            call_location: CallLocation(location),
            id: args.id,
        };
        let entry = self
            .terminal
            .widget_states
            .entry(key)
            .or_insert_with(|| StateEntry {
                state: Box::new(<W::State>::default()),
                frame_index: 0,
            });
        let state: &mut W::State = entry
            .state
            .downcast_mut()
            .expect("The state associated to a widget is not of an expected type");

        // Update the frame index to communicate that it was used during the current draw call.
        entry.frame_index = self.terminal.frame_index;

        // Render the widget
        let buffer = &mut self.terminal.buffers[self.terminal.current];
        let mut context = RenderContext {
            area: args.area,
            buffer,
            state,
        };
        widget.render(&mut context);
    }

    /// After drawing this frame, make the cursor visible and put it at the specified (x, y)
    /// coordinates. If this method is not called, the cursor will be hidden.
    ///
    /// Note that this will interfere with calls to `Terminal::hide_cursor()`,
    /// `Terminal::show_cursor()`, and `Terminal::set_cursor()`. Pick one of the APIs and stick
    /// with it.
    pub fn set_cursor(&mut self, x: u16, y: u16) {
        self.cursor_position = Some((x, y));
    }
}

/// CompletedFrame represents the state of the terminal after all changes performed in the last
/// [`Terminal::draw`] call have been applied. Therefore, it is only valid until the next call to
/// [`Terminal::draw`].
pub struct CompletedFrame<'a> {
    pub buffer: &'a Buffer,
    pub area: Rect,
}

impl<B> Drop for Terminal<B>
where
    B: Backend,
{
    fn drop(&mut self) {
        // Attempt to restore the cursor state
        if self.hidden_cursor {
            if let Err(err) = self.show_cursor() {
                eprintln!("Failed to show the cursor: {}", err);
            }
        }
    }
}

impl<B> Terminal<B>
where
    B: Backend,
{
    /// Wrapper around Terminal initialization. Each buffer is initialized with a blank string and
    /// default colors for the foreground and the background
    pub fn new(backend: B) -> io::Result<Terminal<B>> {
        let size = backend.size()?;
        Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport {
                    area: size,
                    resize_behavior: ResizeBehavior::Auto,
                },
            },
        )
    }

    /// UNSTABLE
    pub fn with_options(backend: B, options: TerminalOptions) -> io::Result<Terminal<B>> {
        Ok(Terminal {
            backend,
            buffers: [
                Buffer::empty(options.viewport.area),
                Buffer::empty(options.viewport.area),
            ],
            current: 0,
            hidden_cursor: false,
            viewport: options.viewport,
            widget_states: HashMap::new(),
            frame_index: 0,
        })
    }

    /// Get a Frame object which provides a consistent view into the terminal state for rendering.
    pub fn get_frame(&mut self) -> Frame<B> {
        Frame {
            terminal: self,
            cursor_position: None,
        }
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

    /// Obtains a difference between the previous and the current buffer and passes it to the
    /// current backend for drawing.
    pub fn flush(&mut self) -> io::Result<()> {
        let previous_buffer = &self.buffers[1 - self.current];
        let current_buffer = &self.buffers[self.current];
        let updates = previous_buffer.diff(current_buffer);
        self.backend.draw(updates.into_iter())
    }

    /// Updates the Terminal so that internal buffers match the requested size. Requested size will
    /// be saved so the size can remain consistent when rendering.
    /// This leads to a full clear of the screen.
    pub fn resize(&mut self, area: Rect) -> io::Result<()> {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].resize(area);
        self.viewport.area = area;
        self.clear()
    }

    /// Queries the backend for size and resizes if it doesn't match the previous size.
    pub fn autoresize(&mut self) -> io::Result<()> {
        if self.viewport.resize_behavior == ResizeBehavior::Auto {
            let size = self.size()?;
            if size != self.viewport.area {
                self.resize(size)?;
            }
        };
        Ok(())
    }

    /// Synchronizes terminal size, calls the rendering closure, flushes the current internal state
    /// and prepares for the next draw call.
    pub fn draw<F>(&mut self, f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame<B>),
    {
        // Autoresize - otherwise we get glitches if shrinking or potential desync between widgets
        // and the terminal (if growing), which may OOB.
        self.autoresize()?;

        let mut frame = self.get_frame();
        f(&mut frame);
        // We can't change the cursor position right away because we have to flush the frame to
        // stdout first. But we also can't keep the frame around, since it holds a &mut to
        // Terminal. Thus, we're taking the important data out of the Frame and dropping it.
        let cursor_position = frame.cursor_position;

        // Draw to stdout
        self.flush()?;

        match cursor_position {
            None => self.hide_cursor()?,
            Some((x, y)) => {
                self.show_cursor()?;
                self.set_cursor(x, y)?;
            }
        }

        // Swap buffers
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;

        // Clean states that were not used in this frame
        let frame_index = self.frame_index;
        self.widget_states
            .retain(|_, v| v.frame_index == frame_index);
        self.frame_index = self.frame_index.wrapping_add(1);

        // Flush
        self.backend.flush()?;
        Ok(CompletedFrame {
            buffer: &self.buffers[1 - self.current],
            area: self.viewport.area,
        })
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

    pub fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        self.backend.get_cursor()
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.backend.set_cursor(x, y)
    }

    /// Clear the terminal and force a full redraw on the next draw call.
    pub fn clear(&mut self) -> io::Result<()> {
        self.backend.clear()?;
        // Reset the back buffer to make sure the next update will redraw everything.
        self.buffers[1 - self.current].reset();
        Ok(())
    }

    /// Queries the real size of the backend.
    pub fn size(&self) -> io::Result<Rect> {
        self.backend.size()
    }
}
