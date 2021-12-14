use std::env;
use std::error::Error;
use hacker_news::cli::HnCommand;
use hacker_news::cli::hacker_news::HackerNews;
use hacker_news::util::init_logger;

#[tokio::main(core_threads = 4)]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logger();


    let app = HackerNews::parser();
    let matches = app.get_matches_from(env::args_os());

    let _res = match HackerNews::cmd(&matches).await {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
}
