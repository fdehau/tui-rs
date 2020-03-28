use termit_ansi::model::AnsiHandler;
use termit_ansi::{
    model::{Ansi as TAnsi, AnsiError, Ctl},
    parser::AnsiParser,
};

use crate::style::{Color, Modifier, Style};
use crate::widgets::Text;

#[derive(Debug)]
pub enum Ansi {
    /// The error and raw bytes that are invalid
    Error(AnsiError, Vec<u8>),
    /// Escape - either as part of a sequence or on it's own
    Esc,
    /// normal or unicode character
    /// * <c>+
    Data(String),
    /// Ansi command
    Command(Ctl, u32, String, Vec<u8>),
}

#[derive(Debug)]
pub struct AnsiBuffer {
    buf: Vec<Ansi>,
}

impl AnsiBuffer {
    fn empty() -> AnsiBuffer {
        AnsiBuffer { buf: Vec::new() }
    }

    pub fn new(input: &str) -> AnsiBuffer {
        let mut buffer = AnsiBuffer::empty();
        let mut parser_ansi = AnsiParser::new([0u8; 32]);
        let buf = input.as_bytes();

        parser_ansi.parse(&mut buffer, &buf);
        buffer.compact();
        buffer
    }

    fn compact(&mut self) {
        let mut new_buf = Vec::new();

        let end =
            self.buf
                .drain(..)
                .into_iter()
                .fold(None::<String>, |current, a| -> Option<String> {
                    match a {
                        Ansi::Data(s) => {
                            let mut cs = current.unwrap_or(String::new());
                            cs.push_str(s.as_str());
                            Some(cs)
                        }
                        other => {
                            current.map(|s| -> () { new_buf.push(Ansi::Data(s)) });
                            new_buf.push(other);
                            None
                        }
                    }
                });

        end.map(|s| -> () { new_buf.push(Ansi::Data(s)) });

        self.buf = new_buf;
    }

    pub fn as_text(&mut self) -> Vec<Text> {
        let mut current: Option<Style> = None;
        let mut t: Vec<Text> = Vec::new();
        self.buf.iter().for_each(|msg| -> () {
            match msg {
                Ansi::Data(data) => {
                    let text = match current {
                        Some(s) => Text::styled(data.as_str(), s),
                        None => Text::raw(data.as_str()),
                    };

                    t.push(text);
                }
                Ansi::Command(Ctl::CSI, _, code, _) => {
                    let c = code.parse::<u8>().unwrap();
                    if c == 0 {
                        current = None
                    } else {
                        let mut s = current.unwrap_or(Style::default());
                        apply_sgr(c, &mut s);
                        current = Some(s)
                    }
                }
                _ => (),
            }
        });

        t
    }
}

impl AnsiHandler for AnsiBuffer {
    fn handle(&mut self, tansi: TAnsi, _raw: &[u8]) {
        let ansi = match tansi {
            TAnsi::Data(str) => Ansi::Data(String::from(str)),
            TAnsi::Esc => Ansi::Esc,
            TAnsi::Command(c, f, p, t) => Ansi::Command(c, f, String::from(p), Vec::from(t)),
            TAnsi::Error(err, raw) => Ansi::Error(err, Vec::from(raw)),
        };
        self.buf.push(ansi)
    }
}

fn apply_sgr(code: u8, style: &mut Style) {
    match code {
        0 => style.reset(),
        1 => style.modifier = style.modifier | Modifier::BOLD,
        2 => style.modifier = style.modifier | Modifier::DIM,
        3 => style.modifier = style.modifier | Modifier::ITALIC,
        4 => style.modifier = style.modifier | Modifier::UNDERLINED,
        5 => style.modifier = style.modifier | Modifier::SLOW_BLINK,
        6 => style.modifier = style.modifier | Modifier::RAPID_BLINK,
        7 => style.modifier = style.modifier | Modifier::REVERSED,
        8 => style.modifier = style.modifier | Modifier::HIDDEN,
        9 => style.modifier = style.modifier | Modifier::CROSSED_OUT,
        30 => style.fg = Color::Black,
        31 => style.fg = Color::Red,
        32 => style.fg = Color::Green,
        33 => style.fg = Color::Yellow,
        34 => style.fg = Color::Blue,
        35 => style.fg = Color::Magenta,
        36 => style.fg = Color::Cyan,
        37 => style.fg = Color::White,
        _ => (),
    }
}
