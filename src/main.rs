use std::env;
use std::error::Error;
use hnews::cli::HnCommand;
use hnews::cli::hnews::Hnews;

fn main() -> Result<(), Box<dyn Error>> {
    let app = Hnews::parser();
    let matches = app.get_matches_from(env::args_os());

    match Hnews::cmd(&matches) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
