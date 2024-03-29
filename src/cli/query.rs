use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;

use crate::client::html_client::Client;
use crate::model::Id;
use crate::cli::HnCommand;
use crate::error::HnError;

pub struct Query;

impl HnCommand for Query {
    const NAME: &'static str = "query";

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
        // SAFE: The clap App will guarantee required arguments are received
        let id = matches.value_of("id")
            .unwrap();
        let id: Id = id.parse()
            .map_err(|_| HnError::ArgumentError(Some("id argument not parseable as numeric")))?;

        let client = Client::new();
        let item = client.item(id)?;
        println!("item = {:#?}", item);
        
        Ok(())
    }
}

