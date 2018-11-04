use crate::backend::Backend;
use crate::buffer::{Buffer, Cell};
use crate::layout::Rect;
use std::io;

#[derive(Debug)]
pub struct TestBackend {
    width: u16,
    buffer: Buffer,
    height: u16,
    cursor: bool,
}

impl TestBackend {
    pub fn new(width: u16, height: u16) -> TestBackend {
        TestBackend {
            width,
            height,
            buffer: Buffer::empty(Rect {
                x: 0,
                y: 0,
                width,
                height,
            }),
            cursor: false,
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
}

impl Backend for TestBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, c) in content {
            let cell = self.buffer.get_mut(x, y);
            cell.symbol = c.symbol.clone();
            cell.style = c.style;
        }
        Ok(())
    }
    fn hide_cursor(&mut self) -> Result<(), io::Error> {
        self.cursor = false;
        Ok(())
    }
    fn show_cursor(&mut self) -> Result<(), io::Error> {
        self.cursor = true;
        Ok(())
    }
    fn clear(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
    fn size(&self) -> Result<Rect, io::Error> {
        Ok(Rect {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        })
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
