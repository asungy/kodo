mod utils;

use utils::logger::Logger;
use log::{
    Level,
    LevelFilter,
    debug,
    error,
    info,
    trace,
    warn,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOGGER: Logger = Logger::new(Level::Info, Some(String::from("log.txt")));
}

fn main() -> () {
    log::set_logger(&*LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);

    trace!("Trace message");
    debug!("Debug message");
    info!("Info message");
    warn!("Warning message");
    error!("Error message");
}
