use hacker_news::cli::hacker_news::HackerNews;
use hacker_news::cli::HnCommand;
use std::env;
use std::error::Error;
use tracing_subscriber::filter::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = HackerNews::parser();
    let matches = app.get_matches_from(env::args_os());

    // TODO: Implement nice error printing on HnError
    if let Err(err) = HackerNews::cmd(&matches).await {
        err.formatted_print();
        std::process::exit(1);
    }

    std::process::exit(0);
}
