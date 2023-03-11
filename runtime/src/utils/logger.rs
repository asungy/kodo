use log::{Record, Level, Metadata};
use std::fs::File;
use std::io::{self, Write};
use std::sync::Mutex;
use std::cell::UnsafeCell;

/// Destination targets for log outputs.
pub enum Target {
    /// Outputs to stdout/stderr.
    STD,
    /// Outputs to file descriptor.
    FILE(Mutex<UnsafeCell<File>>),
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
                    Ok(f) => Mutex::new(UnsafeCell::new(f)),
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
                Target::FILE(file) => {
                    let mut file = file.lock().unwrap();
                    let mut file = file.get_mut();
                    write!(
                        &mut file,
                        "[{level}] - {message}\n",
                        level = record.level(),
                        message = record.args(),
                    ).unwrap();
                },
            }
        }
    }

    fn flush(&self) {}
}
