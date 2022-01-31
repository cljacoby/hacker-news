use std::env;
use std::error::Error;
use hacker_news::cli::HnCommand;
use hacker_news::cli::hacker_news::HackerNews;
use hacker_news::util::init_logger;
// use hacker_news::error::HnError;

fn main() -> Result<(), Box<dyn Error>> {
    init_logger();


    let app = HackerNews::parser();
    let matches = app.get_matches_from(env::args_os());

    match HackerNews::cmd(&matches) {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            // TODO: Implement nice error printing on HnError
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
