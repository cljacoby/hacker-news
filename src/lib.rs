use std::io::Write;
use env_logger;
use env_logger::Builder;
use log;
use log::LevelFilter;

pub mod error;
pub mod client;
pub mod config;
pub mod parser;
pub mod model;
pub mod cli;

pub fn init_logger() {
    let mut logger = Builder::from_default_env();
    logger.filter(Some("hnews::html"), LevelFilter::Trace);
    logger.format(|buf, record| {
        
        let timestamp = buf.timestamp();
        let level = record.level();
        let mod_path = record.module_path().unwrap_or("Could not obtain module path");
        let file =record.file().unwrap_or("Could not obtain file");
        let line = record.line().unwrap_or(0);
        let args = record.args();

        writeln!(
            buf,
            "[{timestamp} {level} {mod_path} {file}:{line}] {args}",
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

    static TEST_LOGGER: Once = Once::new(); 

    pub fn setup() {
        TEST_LOGGER.call_once(|| {
            // init_logger()
            env_logger::init();
        });
    }
}
