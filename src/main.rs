use std::env;
use std::error::Error;

use hnews;

use clap::App;
use clap::AppSettings;
use clap::Arg;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("hnews")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("id")
                .value_name("id")
                .required(true)
                .takes_value(true),
        );

    let matches = app.get_matches_from(env::args_os());
    let id = matches
        .value_of("id")
        .expect("Error: Received no id value.");

    let client = hnews::HNClient::new();
    let item = client.get_by_id(id)?;
    println!("{:#?}", item);
    
    Ok(())
}
