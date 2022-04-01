use crate::cli::login::Login;
use crate::cli::news::News;
use crate::cli::query::Query;
use crate::cli::thread::Thread;
use crate::cli::tree::Tree;
use crate::cli::HnCommand;
use crate::error::HnError;
use clap::App;
use clap::ArgMatches;

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
            (Query::NAME, Some(matches)) => Query::cmd(matches).map_err(|e| {
                log::error!("hackernews subcommand {:?} failed", Query::NAME);
                e
            }),
            (Tree::NAME, Some(matches)) => Tree::cmd(matches).map_err(|e| {
                log::error!("hackernews subcommand {:?} failed", Tree::NAME);
                e
            }),
            (News::NAME, Some(matches)) => News::cmd(matches).map_err(|e| {
                log::error!("hackernews subcommand {:?} failed", News::NAME);
                e
            }),
            (Login::NAME, Some(matches)) => Login::cmd(matches).map_err(|e| {
                log::error!("hackernews subcommand {:?} failed", Login::NAME);
                e
            }),
            (Thread::NAME, Some(matches)) => Thread::cmd(matches).map_err(|e| {
                log::error!("hackernews subcommand {:?} failed", Thread::NAME);
                e
            }),
            // Lack of a subcommand defaults to listing the current HN front page
            _ => News::cmd(matches).map_err(|e| {
                log::error!("hackernews subcommand {:?} failed", News::NAME);
                e
            }),
        }
    }
}
