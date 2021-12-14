use std::env;
use std::error::Error;
use hacker_news::cli::HnCommand;
use hacker_news::cli::hacker_news::HackerNews;
use hacker_news::util::init_logger;
use tokio;

// async fn main() {
// #[tokio::main]
// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     init_logger();
// 
// 
//     let app = HackerNews::parser();
//     let matches = app.get_matches_from(env::args_os());
// 
//     let _res = match HackerNews::cmd(&matches).await {
//         Ok(_) => std::process::exit(0),
//         Err(e) => {
//             println!("{}", e);
//             std::process::exit(1);
//         }
//     };
// }

fn main() {
    // tokio::runtime::Builder::new_multi_thread()
    tokio::runtime::Builder::new_current_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let app = HackerNews::parser();
            let matches = app.get_matches_from(env::args_os());
            let _res = match HackerNews::cmd(&matches).await {
                Ok(_) => std::process::exit(0),
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };

        })
}
