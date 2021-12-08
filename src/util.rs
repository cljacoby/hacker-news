use std::io::Write;
use env_logger::Builder;
use log::Level;
use log;
use log::LevelFilter;
use std::sync::Once;

static TEST_LOGGER: Once = Once::new(); 

pub fn setup() {
    TEST_LOGGER.call_once(|| {
        // init_logger()
        env_logger::init();
    });
}

fn log_level_to_color(level: &Level) -> &'static str {
    match level {
        Level::Error => "\x1b[1;31m",
        Level::Warn => "\x1b[1;33m",
        Level::Info => "\x1b[1;36m",
        Level::Debug => "\x1b[1;32m",
        Level::Trace => "\x1b[1;35m",
    }
}

// TODO: This colorizes the log level, but does not check if output is a tty, and
// therefore will insert the ansi color escapes even if redirected to a file.

pub fn init_logger() {
    let mut logger = Builder::from_default_env();
    logger.filter(Some("hnews::html"), LevelFilter::Trace);
    logger.format(|buf, record| {
        
        let timestamp = buf.timestamp();
        let level = record.level();
        let level = format!("{color}{level}{reset}", 
            color = log_level_to_color(&level),
            level = level,
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

