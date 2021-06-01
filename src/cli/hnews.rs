use std::error::Error;
use clap::App;
use clap::ArgMatches;
use crate::cli::query;
use crate::cli::tree;
use crate::cli::news;
use crate::cli::login;

/// Top level parser/cmd for the cli

pub fn parser<'a, 'b>() -> App<'a, 'b> {
    App::new("hnews")
        .subcommand(query::parser())
        .subcommand(tree::parser())
        .subcommand(news::parser())
        .subcommand(login::parser())
}

pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    env_logger::init();

    match matches.subcommand() {
        (query::NAME, Some(matches)) => query::cmd(matches),
        (tree::NAME, Some(matches)) => tree::cmd(matches),
        (news::NAME, Some(matches)) => news::cmd(matches),
        (login::NAME, Some(matches)) => login::cmd(matches),
        // Lack of a subcommand defaults to listing the current HN front page
        _ => news::cmd(matches),
    }
}
