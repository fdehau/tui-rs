use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::event::{Key,MouseEvent};
use termion::event;
use termion::input::TermRead;

pub enum Event {
    Input(Key),
    Tick,
    MouseEvent(MouseEvent),
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.events(){
                    if let Ok(evt) = evt{
                        match evt{
                            event::Event::Key(key) => {
                                if let Err(_) = tx.send(Event::Input(key)){
                                    return;
                                }
                            }
                            event::Event::Mouse(mouse) => {
                                if let Err(_) = tx.send(Event::MouseEvent(mouse)){
                                    return;
                                }
                            }
                            event::Event::Unsupported(_) => return,
                        }
                    }
                }
            })
        };
        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let tx = tx.clone();
                loop {
                    tx.send(Event::Tick).unwrap();
                    thread::sleep(config.tick_rate);
                }
            })
        };
        Events {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}
