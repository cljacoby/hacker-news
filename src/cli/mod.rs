use std::error::Error;
use clap::App;
use clap::ArgMatches;

pub(crate) mod login;
pub(crate) mod news;
pub(crate) mod query;
pub(crate) mod thread;
pub(crate) mod tree;
pub(crate) mod browse;
pub mod hacker_news;

/// A trait defining the interface to add a subcommand to the command line
/// application. 
pub trait HnCommand {

    /// The name of this subcommand. Will be used at the command line interface
    /// to name this subcommand.
    const NAME: &'static str;

    /// A function which returns a [clap](https://docs.rs/clap/2.33.3/clap/index.html)
    /// App instance. This App will be used as a subcommand in the over all command line
    /// application structure.
    fn parser<'a, 'b>() -> App<'a, 'b>;

    /// The command executed when this subcommand is actually run. This function receives a
    /// [clap](https://docs.rs/clap/2.33.3/clap/index.html) ArgMatches instance, which can
    /// drive optional or argument based logic.
    fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>>;
}
