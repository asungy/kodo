#[cfg(unix)]
use std::os::unix::io::FromRawFd;
use std::os::fd::IntoRawFd;

use log::{Record, Level, Metadata};
use std::fs::File;
use std::io::{self, Write};

/// Destination targets for log outputs.
pub enum Target {
    /// Outputs to stdout/stderr.
    STD,
    /// Outputs to file descriptor.
    FILE(File),
}

pub struct Logger {
    /// Maximum log level. Severity is in descending order.
    level: Level,
    /// Where logs are directed to.
    target: Target,
}

impl Logger {
    pub fn new(level: Level, filename: Option<String>) -> Self {
        let target = match filename {
            Some(filename) => {
                let f = match File::create(filename) {
                    Ok(f) => f,
                    Err(err) => { panic!("{}", err); },
                };

                Target::FILE(f)
            },
            None => { Target::STD },
        };

        Logger {
            level,
            target,
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match &self.target {
                Target::STD => {
                    let mut stdout = io::stdout().lock();
                    stdout.write_all(
                        format!(
                            "[{level}] - {message}\n",
                            level = record.level(),
                            message = record.args(),
                        ).as_bytes()
                    ).unwrap()
                },
                Target::FILE(_file) => {
                    // let raw_fd = &file.into_raw_fd();
                    // let mut f = unsafe { File::from_raw_fd(*raw_fd) };
                    // write!(
                    //     &mut f,
                    //     "[{level}] - {message}\n",
                    //     level = record.level(),
                    //     message = record.args(),
                    // ).unwrap();
                    todo!("Implement me!");
                },
            }
        }
    }

    fn flush(&self) {}
}
