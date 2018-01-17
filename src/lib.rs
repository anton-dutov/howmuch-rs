#[macro_use]
extern crate log;

use std::time::{Duration, Instant};

#[derive(Debug, PartialEq)]
pub enum Precision {
    Millisecond,
    Microsecond,
    Nanosecond,
}

#[derive(Debug, PartialEq)]
pub enum Output {
    StdOut,
    Log(log::Level),
}

#[derive(Debug, PartialEq)]
pub enum Report {
    Always,
    Never,
    Gt(Duration), // total time >  durtaion
    Ge(Duration), // total time >= durtaion
    Lt(Duration), // total time <  durtaion
    Le(Duration), // total time <= durtaion
}

#[derive(Debug)]
pub struct Tag {
    pub since_begin: Duration,
    pub since_prev:  Duration,
    pub tag: String,
}

#[derive(Debug)]
pub struct HowMuch {
    total: Instant,
    diff:  Instant,
    output: Output,
    precision: Precision,
    report: Report,
    tags: Vec<Tag>,
}

impl HowMuch {
    pub fn new() -> HowMuch {
        let mut hm = HowMuch {
            total: Instant::now(),
            diff:  Instant::now(),
            output: Output::StdOut,
            precision: Precision::Microsecond,
            report: Report::Always,
            tags: Vec::new(),
        };
        hm.tag("BEGIN");
        hm
    }

    pub fn tag(&mut self, tag: &str) {
        let total = self.total.elapsed();
        let diff  = self.diff.elapsed();
        self.diff = Instant::now();

        self.tags.push(Tag{
            since_begin: total,
            since_prev: diff,
            tag: tag.to_owned()
        })
    }

    pub fn set_output(&mut self, output: Output) {
        self.output = output;
    }

    pub fn set_precision(&mut self, precision: Precision) {
        self.precision = precision;
    }

    pub fn set_report(&mut self, report: Report) {
        self.report = report;
    }
}

impl Drop for HowMuch {
    fn drop(&mut self) {
        self.tag("END");

        let to_precision = |v| {
            match self.precision {
                Precision::Millisecond => format!("{:03}", v / 1_000_000),
                Precision::Microsecond => format!("{:06}", v / 1_000),
                Precision::Nanosecond  => format!("{:09}", v),
            }
        };

        let report = || {
            // FIXME: ugly formating

            for tag in &self.tags {
                let record = format!("{:>4}.{} | {:>4}.{} | {}",
                    tag.since_begin.as_secs(),
                    to_precision(tag.since_begin.subsec_nanos()),
                    tag.since_prev.as_secs(),
                    to_precision(tag.since_prev.subsec_nanos()),
                    tag.tag);

                match self.output {
                    Output::StdOut => println!("{}", &record),
                    Output::Log(ref lvl) => log!(lvl.clone(), "{}", &record),
                }
            }
        };

        let mut total = self.total.elapsed();

        if let Some(tag) = self.tags.last() {
            total = tag.since_begin.clone();
        }

        match self.report {
            Report::Always => report(),
            Report::Gt(ref v) if &total >  v => report(),
            Report::Ge(ref v) if &total >= v => report(),
            Report::Lt(ref v) if &total <  v => report(),
            Report::Le(ref v) if &total <= v => report(),
            _ => {}
        }
    }
}
