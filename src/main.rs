use std::env;
use std::error::Error;
use std::fmt::Write;
use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use env_logger;
use hnews::models::Id;
use hnews::client::Client;
use hnews::models::Listing;
use hnews::config::HNConfig;

fn init_logger() {
    #[allow(unused_variables)]
    env_logger::init();
}

pub mod query {

    use super::*;

    pub const NAME: &'static str = "query";

    pub fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(query::NAME).arg(
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

        let client = Client::new("test", "test");
        let item = client.item(id)?;
        println!("item = {:#?}", item);

        let comments = client._comments(id)?;
        println!("comments = {:#?}", comments);

        Ok(())
    }

}

/// For a comment-able item, retrieve all the comments
pub mod tree {

    use super::*;

    pub const NAME: &'static str = "tree";

    pub fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(tree::NAME)
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
        // let mut replies: Vec<Comment> = vec![];
        // let client = HNClient::new();
        // for reply in client.iter_replies(id)? {
        //     let reply = reply?;
        //     replies.push(reply);
        // }
        // println!("{:#?}", replies);
        // Ok(())
        unimplemented!("Re-implement this with HTML based client");
    }
}

/// Get front page listings of Hacker News.
pub mod news {

    use super::*;
    
    pub const NAME: &'static str = "news";

    pub fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(news::NAME)
    }

    fn write_ron<Wrt>(mut fmt: Wrt, listing: &Listing) -> Result<(), Box<dyn Error>>
        where Wrt: std::fmt::Write
    {
        fmt.write_str(&format!("{:#?}\n", listing))?;

        Ok(())
    }

    // id | score | author | title
    fn print_tabular(listings: &Vec<Listing>) {
        for l in listings.iter() {
            println!("{:<1}        {:<1}        {:<1}        {:<1}",
                l.id,
                l.score.unwrap_or(0),
                l.user.as_ref().unwrap_or(&"".to_string()),
                l.title);
        }
    }

    pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let client = Client::new("", "");
        // for listing in client.news()? {
        //     write_ron(&mut out, &listing);
        // }
        let listings = client.news()?;
        print_tabular(&listings);

        Ok(())
    }

}

/// Login with a given username and password
pub mod login {

    use super::*;

    pub const NAME: &'static str = "login";

    pub fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(login::NAME)
            .arg(
                Arg::with_name("username")
                    .value_name("username")
                    .required(true)
                    .takes_value(true)
                    // .min_values(1),
            )
            // TODO: Ideally this should be a prompted input with no display
            .arg(
                Arg::with_name("password")
                    .value_name("password")
                    .required(true)
                    .takes_value(true)
                    // .min_values(1),
            )
    }

    pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let username = matches
            .value_of("username")
            .ok_or("username is required for login")?;
        let password = matches
        .value_of("password")
        .ok_or("password is required for login")?;
        
        let client = Client::new(username, password);
        client.login()?;

        Ok(())
    }
    
}

// Top level parser/cmd for the cli
pub mod hn {

    use super::*;

    pub fn parser<'a, 'b>() -> App<'a, 'b> {
        App::new("hnews")
            .subcommand(query::parser())
            .subcommand(tree::parser())
            .subcommand(news::parser())
            .subcommand(login::parser())
    }

    pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        init_logger();

        match matches.subcommand() {
            (query::NAME, Some(matches)) => query::cmd(matches),
            (tree::NAME, Some(matches)) => tree::cmd(matches),
            (news::NAME, Some(matches)) => news::cmd(matches),
            (login::NAME, Some(matches)) => login::cmd(matches),
            // Lack of a subcommand defaults to listing the current HN front page
            _ => news::cmd(matches),
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
