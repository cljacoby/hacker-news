use serde_json;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use crate::client::html_client::Client;
use crate::model::Id;
use crate::cli::HnCommand;
use crate::error::HnError;

pub struct Thread;

impl HnCommand for Thread {
    const NAME: &'static str = "thread";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME).arg(
            Arg::with_name("id")
                .value_name("id")
                .required(true)
                .takes_value(true)
                .min_values(1),
        )
    }

    fn cmd(matches: &ArgMatches) -> Result<(), Box<HnError>> {
        log::debug!("Begin hackernews subcommand command");

        let id = match matches.value_of("id") {
            None => unreachable!("clap will require an argument value"),
            Some(id) => id,
        };
        let id: Id = id.parse()
            .map_err(|_| HnError::ArgumentError(Some("Thread id not parseable as u32")))?;

        let client = Client::new();
        let thread = client.thread(id)?;
        let json = serde_json::to_string(&thread)
            .map_err(|_src| HnError::SerializationError(
                Some("Failed to serialize thread using serde derived implementations")
        ))?;
        println!("{}", json);

        Ok(())
    }
}

