use std::env;
use std::error::Error;

use hnews::firebase::models::Comment;
use hnews::firebase::models::Id;
use hnews::firebase::client::HNClient;

use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;

use env_logger;

fn init_logger() {
    #[allow(unused_variables)]
    let logger = env_logger::init();
}

// Query an item by the itemId
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

// For a comment-able item, retireve all the comments
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

        // Instantiate client, and retrieve comment data
        let mut replies: Vec<Comment> = vec![];
        let client = HNClient::new();
        for reply in client.iter_replies(id)? {
            let reply = reply?;
            replies.push(reply);
        }

        println!("{:#?}", replies);

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
