use std::env;
use std::error::Error;

use hnews::models::Id;
use hnews::models::Item;
use hnews::HNClient;
use hnews::error::HNError;

use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;


use env_logger;

// Default timeout for request loops
// const TIMEOUT: u64 = 100;

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

        
        // // Parse timeout. Obtain from `--timeout` argument, or else use default
        // let millis = match matches.value_of("timeout") {
        //     None => TIMEOUT,
        //     Some(millis) => millis.parse::<u64>().map_err(|src_err| {
        //         HNError::new(
        //             format!("Could not parse timeout argument `{}`", millis),
        //             Some(Box::new(src_err)),
        //         )
        //     })?,
        // };
        // let timeout = Duration::from_millis(millis);

        // Instantiate client, and retrieve top level story
        let client = HNClient::new();
        match client.get_by_id(id)? {
            Item::Comment(comment) => {
                for reply in client.walk_comment_replies(comment) {
                    let reply = reply?;
                    if let Some(text) = reply.text {
                        println!("reply = {}", text);
                    }
                }
            },
            Item::Story(story) => {
                for reply in client.walk_story_replies(story) {
                    let reply = reply?;
                    if let Some(text) = reply.text {
                        println!("reply = {}", text);
                    }
                }
            },
            _ => {
                let err = HNError::new(
                    format!("Cannot only perform `tree` on Comments and Stories"),
                    None);
                return Err(Box::new(err))
            }
        }

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
