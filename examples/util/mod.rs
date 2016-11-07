#![allow(dead_code)]

extern crate rand;
extern crate log4rs;
extern crate log;

use self::rand::distributions::{IndependentSample, Range};

use self::log::LogLevelFilter;
use self::log4rs::append::file::FileAppender;
use self::log4rs::encode::pattern::PatternEncoder;
use self::log4rs::config::{Appender, Config, Root};

pub fn setup_log(file_name: &str) {
    let log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} / {d(%H:%M:%S)} / \
                                              {M}:{L}{n}{m}{n}{n}")))
        .build(file_name)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("log", Box::new(log)))
        .build(Root::builder().appender("log").build(LogLevelFilter::Debug))
        .unwrap();
    log4rs::init_config(config).unwrap();
}

#[derive(Clone)]
pub struct RandomSignal {
    range: Range<u64>,
    rng: rand::ThreadRng,
}

impl RandomSignal {
    pub fn new(lower: u64, upper: u64) -> RandomSignal {
        RandomSignal {
            range: Range::new(lower, upper),
            rng: rand::thread_rng(),
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.range.ind_sample(&mut self.rng))
    }
}

#[derive(Clone)]
pub struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    pub fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
        SinSignal {
            x: 0.0,
            interval: interval,
            period: period,
            scale: scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

pub struct MyTabs<'a> {
    pub titles: Vec<&'a str>,
    pub selection: usize,
}

impl<'a> MyTabs<'a> {
    pub fn next(&mut self) {
        self.selection = (self.selection + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.selection > 0 {
            self.selection -= 1;
        } else {
            self.selection = self.titles.len() - 1;
        }
    }
}
