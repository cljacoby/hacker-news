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

    // TODO: Implement nice error printing on HnError
    if let Err(err) = HackerNews::cmd(&matches) {
        err.formatted_print();
        std::process::exit(1);
    }

    std::process::exit(0);
}
