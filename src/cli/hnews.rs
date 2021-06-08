use std::error::Error;
use clap::App;
use clap::ArgMatches;
use crate::cli::HnCommand;
use crate::cli::tree::Tree;
use crate::cli::login::Login;
use crate::cli::query::Query;
use crate::cli::news::News;
use crate::cli::comments::Comments;

/// Top level parser/cmd for the cli
pub struct Hnews;

impl HnCommand for Hnews {
    const NAME: &'static str = "hnews";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        App::new(Self::NAME)
            .subcommand(Query::parser())
            .subcommand(Tree::parser())
            .subcommand(News::parser())
            .subcommand(Login::parser())
            .subcommand(Comments::parser())
    }

    fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        env_logger::init();

        match matches.subcommand() {
            (Query::NAME, Some(matches)) => Query::cmd(matches),
            (Tree::NAME, Some(matches)) => Tree::cmd(matches),
            (News::NAME, Some(matches)) => News::cmd(matches),
            (Login::NAME, Some(matches)) => Login::cmd(matches),
            (Comments::NAME, Some(matches)) => Comments::cmd(matches),
            // Lack of a subcommand defaults to listing the current HN front page
            _ => News::cmd(matches),
        }
    }

}

