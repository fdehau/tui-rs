extern crate rustbox;

use std::io;

use super::Backend;
use buffer::Cell;
use layout::Rect;
use style::{Color, Modifier};

pub struct RustboxBackend {
    rustbox: rustbox::RustBox,
}

impl RustboxBackend {
    pub fn new() -> Result<RustboxBackend, rustbox::InitError> {
        let rustbox = try!(rustbox::RustBox::init(Default::default()));
        Ok(RustboxBackend { rustbox: rustbox })
    }

    pub fn with_rustbox(instance: rustbox::RustBox) -> RustboxBackend {
        RustboxBackend { rustbox: instance }
    }

    pub fn rustbox(&self) -> &rustbox::RustBox {
        &self.rustbox
    }
}

impl Backend for RustboxBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let mut inst = 0;
        for (x, y, cell) in content {
            inst += 1;
            self.rustbox.print(
                x as usize,
                y as usize,
                cell.style.modifier.into(),
                cell.style.fg.into(),
                cell.style.bg.into(),
                &cell.symbol,
            );
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
        Ok(Rect {
            x: 0,
            y: 0,
            width: self.rustbox.width() as u16,
            height: self.rustbox.height() as u16,
        })
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        self.rustbox.present();
        Ok(())
    }
}

fn rgb_to_byte(r: u8, g: u8, b: u8) -> u16 {
    u16::from((r & 0xC0) + ((g & 0xE0) >> 2) + ((b & 0xE0) >> 5))
}

impl Into<rustbox::Color> for Color {
    fn into(self) -> rustbox::Color {
        match self {
            Color::Reset => rustbox::Color::Default,
            Color::Black | Color::Gray | Color::DarkGray => rustbox::Color::Black,
            Color::Red | Color::LightRed => rustbox::Color::Red,
            Color::Green | Color::LightGreen => rustbox::Color::Green,
            Color::Yellow | Color::LightYellow => rustbox::Color::Yellow,
            Color::Magenta | Color::LightMagenta => rustbox::Color::Magenta,
            Color::Cyan | Color::LightCyan => rustbox::Color::Cyan,
            Color::White => rustbox::Color::White,
            Color::Blue | Color::LightBlue => rustbox::Color::Blue,
            Color::Rgb(r, g, b) => rustbox::Color::Byte(rgb_to_byte(r, g, b)),
        }
    }
}

impl Into<rustbox::Style> for Modifier {
    fn into(self) -> rustbox::Style {
        match self {
            Modifier::Bold => rustbox::RB_BOLD,
            Modifier::Underline => rustbox::RB_UNDERLINE,
            Modifier::Invert => rustbox::RB_REVERSE,
            _ => rustbox::RB_NORMAL,
        }
    }
}
