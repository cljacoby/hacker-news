//! A library for interacting with Hacker News.


use std::io::Write;
use env_logger::Builder;
use log::LevelFilter;

pub mod error;
pub mod client;
pub mod config;
pub mod parser;
pub mod model;
pub mod cli;

// TODO: I want to use this custom log string writer, as it in addition to providing
// the module path it provides you with the line of source code and file path to the
// emitting file. However, I also really want the colorization of the log level
// that the default log function provides. This function provides a proof of concept 
// for how the custom log string function could still provide colorization.

pub fn init_logger() {
    let mut logger = Builder::from_default_env();
    logger.filter(Some("hnews::html"), LevelFilter::Trace);
    logger.format(|buf, record| {
        
        let timestamp = buf.timestamp();
        let level = format!("{green}{level}{reset}", 
            green = "\x1b[1;32m",
            level = record.level(),
            reset = "\x1b[1;0m",
        );
        let mod_path = record.module_path().unwrap_or("Could not obtain module path");
        let file =record.file().unwrap_or("Could not obtain file");
        let line = record.line().unwrap_or(0);
        let args = record.args();

        writeln!(
            buf,
            "[{timestamp} {level} {mod_path} {file}:{line}] {args}",
            // "[{timestamp} {level} {file}:{line}] {args}",
            timestamp = timestamp,
            level = level,
            mod_path = mod_path,
            file = file,
            line = line,
            args = args
        )
    });

    logger.init();
}

#[cfg(test)]
mod tests {

    use std::sync::Once;
    // use super::init_logger;

    static TEST_LOGGER: Once = Once::new(); 

    pub fn setup() {
        TEST_LOGGER.call_once(|| {
            // init_logger()
            env_logger::init();
        });
    }
}
