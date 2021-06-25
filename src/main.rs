use std::env;
use std::error::Error;
use env_logger;
use hacker_news::cli::HnCommand;
use hacker_news::cli::hacker_news::HackerNews;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let app = HackerNews::parser();
    let matches = app.get_matches_from(env::args_os());

    match HackerNews::cmd(&matches) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
