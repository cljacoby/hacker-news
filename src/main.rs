use std::env;
use std::error::Error;
// use std::fmt::Write;
// use clap::App;
// use clap::AppSettings;
// use clap::Arg;
// use clap::ArgMatches;
// use clap::SubCommand;
// use env_logger;
// use hnews::models::Id;
// use hnews::client::Client;
// use hnews::models::Listing;
// use hnews::config::HNConfig;
use hnews::cli::hnews;



// fn init_logger() {
//     #[allow(unused_variables)]
//     env_logger::init();
// }

/*
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
*/

/*
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
*/

/*
/// Get front page listings of Hacker News.
pub mod news {

    use super::*;
    use grid_printer::GridPrinter;
    use grid_printer::style::StyleOpt;
    use grid_printer::style::Fg;
    
    pub const NAME: &'static str = "news";

    pub fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(news::NAME)
    }

    pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let client = Client::new("", "");
        let listings = client.news()?;
        let grid: Vec<Vec<String>> = listings.into_iter().map(|l| vec![
            l.id.clone().to_string(),
            l.score.clone().unwrap_or(0).to_string(),
            l.user.clone().unwrap_or("".to_string()),
            l.title.clone(),
        ]).collect();

        let rows = grid.len();
        let cols = match grid.get(0) {
            None => return Ok(()),
            Some(first_row) => first_row.len(),
        };
        let printer = GridPrinter::builder(rows, cols)
            .col_spacing(4)
            .col_style(1, StyleOpt::new().fg(Fg::Red))?
            .build();
        printer.print(&grid);

        Ok(())
    }

}
*/

/*
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
*/

// Top level parser/cmd for the cli
/*
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
*/

fn main() -> Result<(), Box<dyn Error>> {
    let app = hnews::parser();
    let matches = app.get_matches_from(env::args_os());

    match hnews::cmd(&matches) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
