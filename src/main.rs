use std::env;
use std::error::Error;
use std::collections::VecDeque;
use std::time::Duration;
use std::thread;

use hnews::Item;
use hnews::Id;
use hnews::HNClient;
use hnews::HNError;

use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;

// Default timeout for request loops
const TIMEOUT: u64 = 100;

pub mod query {

    use super::*;

    pub fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("query")
                .arg(
                    Arg::with_name("id")
                        .value_name("id")
                        .required(true)
                        .takes_value(true)
                        .min_values(1)
                )   
    }
    
    pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let id = match matches.value_of("id") {
            None => unreachable!("clap requires an argument value"),
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
                        .min_values(1)
                )
                .arg(
                    Arg::with_name("timeout")
                        .value_name("timeout")
                        .long("timeout")
                        .short("t")
                        .takes_value(true)
                        .min_values(1)
                )
    }
    
    pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let id: Id = matches.value_of("id")
            .ok_or("Id is required for query")?
            .parse()?;

        let millis = match matches.value_of("timeout") {
            None => TIMEOUT,
            Some(millis) => {
                millis.parse::<u64>()
                    .map_err(|src_err| HNError::new(
                        format!("Could not parse timeout argument `{}`", millis),
                        Some(Box::new(src_err))
                    ))?
            }
        };
        let timeout = Duration::from_millis(millis);

        let client = HNClient::new();
        let item = client.get_by_id(id)?;
        let story = match item {
            Item::Story(story) => Ok(story),
            _ => {
                Err(format!("Item id {} is not of type Story", id))
            },
        }?;




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
    
    fn fetch_kids(client: &HNClient, id: Id) -> Result<Option<Vec<Id>>, Box<dyn Error>> {
        let resp = client.get_by_id(id)?;
        println!("resp = {:?}", resp);
        match resp {
            Item::Story(story) => Ok(story.kids),
            Item::Comment(comment) => Ok(comment.kids),
            _ => Ok(None),
        }
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











