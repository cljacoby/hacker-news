use std::env;
use std::error::Error;
use hacker_news::cli::HnCommand;
use hacker_news::cli::hacker_news::HackerNews;
use hacker_news::util::init_logger;
use hacker_news::error::HnError;

fn main() -> Result<(), Box<dyn Error>> {
    init_logger();


    let app = HackerNews::parser();
    let matches = app.get_matches_from(env::args_os());

    match HackerNews::cmd(&matches) {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            match err.downcast_ref::<HnError>() {
                Some(hn_err) => hn_err.print(),
                None => eprintln!("{}", err),
            }
            std::process::exit(1);
        }
    }
}
