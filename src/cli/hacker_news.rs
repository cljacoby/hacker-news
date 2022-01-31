use clap::App;
use clap::ArgMatches;
use crate::cli::HnCommand;
use crate::cli::tree::Tree;
use crate::cli::login::Login;
use crate::cli::query::Query;
use crate::cli::news::News;
use crate::cli::thread::Thread;
use crate::error::HnError;

/// Top level parser/cmd for the cli
pub struct HackerNews;

impl HnCommand for HackerNews {
    const NAME: &'static str = "hackernews";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        App::new(Self::NAME)
            .subcommand(Query::parser())
            .subcommand(Tree::parser())
            .subcommand(News::parser())
            .subcommand(Login::parser())
            .subcommand(Thread::parser())
    }

    fn cmd(matches: &ArgMatches) -> Result<(), Box<HnError>> {
        match matches.subcommand() {
            (Query::NAME, Some(matches)) => Query::cmd(matches),
            (Tree::NAME, Some(matches)) => Tree::cmd(matches),
            (News::NAME, Some(matches)) => News::cmd(matches),
            (Login::NAME, Some(matches)) => Login::cmd(matches),
            (Thread::NAME, Some(matches)) => Thread::cmd(matches),
            // Lack of a subcommand defaults to listing the current HN front page
            _ => News::cmd(matches),
        }
    }

}

