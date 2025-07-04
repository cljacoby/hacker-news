use std::sync::Arc;

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use log::debug;

use crate::cli::HnCommand;
use crate::client::HnClient;
use crate::error::HnError;
use crate::model::Id;

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

    async fn cmd(matches: &ArgMatches<'_>) -> Result<(), Box<HnError>> {
        // SAFE: The clap App will guarantee required arguments are received
        let id = matches.value_of("id").unwrap();
        let id: Id = id
            .parse()
            .map_err(|_| HnError::ArgumentError(Some("id argument not parseable as numeric")))?;

        let client = HnClient::new();
        let item = client.item(id).await.map_err(|err| {
            tracing::error!(err=?err, "failed query command");
            Box::new(HnError::Unknown)
        })?;

        // todo: tmp testing, delete
        let client = Arc::new(client);
        debug!("item = {:#?}", item);

        let thread = client.thread(item.id()).await;
        // .map_err(|err| {
        //     tracing::error!(err=?err, "failed query command");
        //     Box::new(HnError::Unknown)
        // })?;

        debug!("thread = {:#?}", thread);

        Ok(())
    }
}
