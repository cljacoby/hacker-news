use std::error::Error;
use serde_json;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use crate::client::html_client::Client;
use crate::model::Id;
use crate::cli::HnCommand;

pub struct Comments;

impl HnCommand for Comments {
    const NAME: &'static str = "comments";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME).arg(
            Arg::with_name("id")
                .value_name("id")
                .required(true)
                .takes_value(true)
                .min_values(1),
        )
    }

    fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let id = match matches.value_of("id") {
            None => unreachable!("clap will require an argument value"),
            Some(id) => id,
        };
        let id: Id = id.parse()?;

        let client = Client::new("test", "test");

        let comments = client._comments(id)?;
        let json = serde_json::to_string(&comments)?;
        println!("{}", json);

        Ok(())
    }
}

