use std::env;
use std::error::Error;
use std::collections::VecDeque;
use std::time::Duration;
use std::thread;

use hnews::Item;
use hnews::Id;
use hnews::HNClient;

use clap::App;
use clap::AppSettings;
use clap::Arg;

const TIMEOUT: u64 = 100;

fn fetch_kids(client: &HNClient, id: Id) -> Result<Option<Vec<Id>>, Box<dyn Error>> {
    let resp = client.get_by_id(id)?;
    println!("resp = {:?}", resp);
    match resp {
        Item::Story(story) => Ok(story.kids),
        Item::Comment(comment) => Ok(comment.kids),
        _ => Ok(None),
    }
}


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

    let client = HNClient::new();
    let id: Id = id.parse()?;
    let item = client.get_by_id(id)?;
    let story = match item {
        Item::Story(story) => story,
        _ => {
            eprintln!("Item id {:?} is not of type Story", id);
            std::process::exit(1);
        },
    };

    let timeout = Duration::from_millis(TIMEOUT);

    let mut ids: VecDeque<Id> = VecDeque::new();
    ids.push_back(story.id);
    while let Some(id) = ids.pop_front() {
        thread::sleep(timeout);
        println!("popped id = {}", id);
        if let Some(kids) = fetch_kids(&client, id)? {
            for kid in kids {
                ids.push_back(kid);
            }
        }
    }

    Ok(())

}
