use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::thread;
use std::time::Duration;

use hnews::Comment;
use hnews::HNClient;
use hnews::error::HNError;
use hnews::Id;
use hnews::Item;

use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;

use serde_json;
use serde_json::json;

use env_logger;

// Default timeout for request loops
const TIMEOUT: u64 = 100;

fn init_logger() {
    #[allow(unused_variables)]
    let logger = env_logger::init();
}

pub mod query {

    use super::*;

    pub fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("query").arg(
            Arg::with_name("id")
                .value_name("id")
                .required(true)
                .takes_value(true)
                .min_values(1),
        )
    }

    pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let id = match matches.value_of("id") {
            None => unreachable!("clap will require an argument value"),
            Some(id) => id,
        };
        let id: Id = id.parse()?;

        let client = HNClient::new();
        let resp = client.get_by_id(id)?;
        println!("{:#?}", resp);

        Ok(())
    }
}

pub mod tree {

    use super::*;

    pub fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("tree")
            .arg(
                Arg::with_name("id")
                    .value_name("id")
                    .required(true)
                    .takes_value(true)
                    .min_values(1),
            )
            .arg(
                Arg::with_name("timeout")
                    .value_name("timeout")
                    .long("timeout")
                    .short("t")
                    .required(false)
                    .takes_value(true)
                    .min_values(1),
            )
    }

    pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        // Parse command-line argument of HackerNews ID
        let id: Id = matches
            .value_of("id")
            .ok_or("Id is required for query")?
            .parse()?;

        // Parse timeout. Obtain from `--timeout` argument, or else use default
        let millis = match matches.value_of("timeout") {
            None => TIMEOUT,
            Some(millis) => millis.parse::<u64>().map_err(|src_err| {
                HNError::new(
                    format!("Could not parse timeout argument `{}`", millis),
                    Some(Box::new(src_err)),
                )
            })?,
        };
        let timeout = Duration::from_millis(millis);

        // Instantiate client, and retrieve top level story
        let client = HNClient::new();
        let item = client.get_by_id(id)?;
        let story = match item {
            Item::Story(story) => Ok(story),
            _ => {
                let err = HNError::new(format!("Item id {} is not of type Story", id), None);

                Err(err)
            }
        }?;

        // TODO: It would be nice if the ID queue could accept all Item varaints,
        // and handle accordingly.

        // Create output sink for comments
        let mut comments: Vec<Comment> = vec![];

        // Queue used for BFS style tree traversal
        let mut ids: VecDeque<Id> = VecDeque::new();
        if let Some(kids) = story.kids.as_ref() {
            for kid in kids.iter() {
                ids.push_back(*kid);
            }
        }

        // Pop an Id, get comment data, and push all child IDs to the queue
        while let Some(id) = ids.pop_front() {
            thread::sleep(timeout);
            eprintln!("popped id = {}", id);
            if let Item::Comment(comment) = client.get_by_id(id)? {
                if let Some(kids) = comment.kids.as_ref() {
                    for kid in kids.iter() {
                        ids.push_back(*kid);
                    }
                }
                comments.push(comment);
            }
        }

        let data = json!({
            "story": story,
            "comments": comments,
        });
        let s = serde_json::to_string(&data)?;
        println!("{}", s);

        Ok(())
    }
}

// Top level parser/cmd for the cli
pub mod hn {

    use super::*;

    pub fn parser<'a, 'b>() -> App<'a, 'b> {
        App::new("hnews")
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(query::parser())
            .subcommand(tree::parser())
    }

    pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        init_logger();

        match matches.subcommand() {
            ("query", Some(matches)) => query::cmd(matches),
            ("tree", Some(matches)) => tree::cmd(matches),
            _ => unreachable!("clap will require passing a recognized subcommand"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = hn::parser();
    let matches = app.get_matches_from(env::args_os());

    match hn::cmd(&matches) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
