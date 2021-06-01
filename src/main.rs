use std::env;
use std::error::Error;
use hnews::cli::hnews;

fn main() -> Result<(), Box<dyn Error>> {
    let app = hnews::parser();
    let matches = app.get_matches_from(env::args_os());

    match hnews::cmd(&matches) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
