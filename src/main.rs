use std::env;
use std::error::Error;

use hnews;

use clap::App;
use clap::AppSettings;
use clap::Arg;

fn main() -> Result<(), Box<Error>> {
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
    let story = client.get_story_by_id(id)?;
    story.print();

    let res = match story.kids {
        None => {
            println!("{}'s kids field was absent.", id);
        },
        Some(v) => match v {
            None => {
                println!("{}'s kids field was null.", id);
            },
            Some(kids) => { 
                for kid in kids.iter() {
                    let id = &format!("{}", kid);
                    // TODO: The abstraction here is wrong, as the instanes of `kid` are Comments,
                    // not Stories
                    let story = client.get_story_by_id(id)?;
                    story.print();
                }
            }
        }
    };
    
    Ok(())
}
