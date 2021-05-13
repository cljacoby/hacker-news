use std::env;
use std::error::Error;
use std::fmt::Write;
use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use env_logger;
use hnews::firebase::models::Comment;
use hnews::firebase::models::Id;
use hnews::firebase::client::HNClient;
use hnews::html::client::Client;
use hnews::html::client::Listing;

fn init_logger() {
    #[allow(unused_variables)]
    env_logger::init();
}

/// Query an item by the itemId
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

        let client = HNClient::new();
        let resp = client.get_by_id(id)?;
        println!("{:#?}", resp);

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
    
    fn write_tabular<Wrt>(mut fmt: Wrt, listing: &Listing) -> Result<(), Box<dyn Error>>
        where Wrt: std::fmt::Write
    {
        fmt.write_str(&format!("{}|{}|{}|{}|{}\n",
            listing.title,
            listing.id,
            listing.score.unwrap_or(0),
            listing.user.clone().unwrap_or("".to_string()),
            listing.url
        ));

        Ok(())
    }

    pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let client = Client::new("", "");
        // let stdout = std::io::stdout();
        // let mut handle = stdout.lock();
        let mut out = String::new();

        for listing in client.news()? {
            // write_tabular(&mut out, &listing);
            write_ron(&mut out, &listing);
        }
        println!("{}", out);

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
        
        // TODO: Having to make this mutable is not ideal
        let mut client = Client::new(username, password);
        client.login()?;

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
