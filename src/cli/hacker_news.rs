// use crate::cli::login::Login;
// use crate::cli::tree::Tree;
// use crate::cli::thread::Thread;
use crate::cli::query::Query;
use crate::cli::news::News;
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
            .subcommand(News::parser())
            .subcommand(Query::parser())
            // .subcommand(Tree::parser())
            // .subcommand(Login::parser())
            // .subcommand(Thread::parser())
    }

    async fn cmd(matches: &ArgMatches<'_>) -> Result<(), Box<HnError>> {
        match matches.subcommand() {
            (Query::NAME, Some(matches)) => Query::cmd(matches).await.map_err(|err| {
                tracing::error!(err=?err, cmd=Query::NAME, "subcommand failed");
                Box::new(HnError::Unknown)
            }),
            // (Tree::NAME, Some(matches)) => Tree::cmd(matches).map_err(|e| {
            //     log::error!("hackernews subcommand {:?} failed", Tree::NAME);
            //     e
            // }),
            // (Login::NAME, Some(matches)) => Login::cmd(matches).map_err(|e| {
            //     log::error!("hackernews subcommand {:?} failed", Login::NAME);
            //     e
            // }),
            // (Thread::NAME, Some(matches)) => Thread::cmd(matches).map_err(|e| {
            //     log::error!("hackernews subcommand {:?} failed", Thread::NAME);
            //     e
            // }),
            // // Lack of a subcommand defaults to listing the current HN front page
            // (News::NAME, Some(matches)) => News::cmd(matches).await,
            (_, _) => News::cmd(matches).await,
        }
    }
}
