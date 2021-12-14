use std::error::Error;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use crate::client::html_client::Client;
use crate::model::Id;
use crate::cli::HnCommand;
use async_trait::async_trait;

pub struct Query;

#[async_trait]
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

    async fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let id = match matches.value_of("id") {
            None => unreachable!("clap will require an argument value"),
            Some(id) => id,
        };
        let id: Id = id.parse()?;

        let client = Client::new();
        let item = client.item(id).await?;
        println!("item = {:#?}", item);
        
        Ok(())
    }
}

