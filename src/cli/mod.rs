use clap::App;
use clap::ArgMatches;

use crate::error::HnError;

// pub(crate) mod login;
pub(crate) mod news;
pub(crate) mod query;
// pub(crate) mod thread;
// pub(crate) mod tree;
pub mod hacker_news;

/// A trait defining the interface to add a subcommand to the command line
/// application.
#[allow(async_fn_in_trait)]
pub trait HnCommand {
    /// The name of this subcommand.
    const NAME: &'static str;

    /// A function which returns a [clap](https://docs.rs/clap/2.33.3/clap/index.html)
    /// [App] instance. This [App] will be used as a subcommand in the over all CLI.
    fn parser<'a, 'b>() -> App<'a, 'b>;

    /// The 'main' function subcommand is actually invoked. This function receives an
    /// [ArgMatches] instance to define it's behavior.
    // fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>>;
    async fn cmd(matches: &ArgMatches) -> Result<(), Box<HnError>>;
}
