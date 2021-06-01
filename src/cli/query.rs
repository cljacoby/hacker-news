use std::error::Error;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use crate::client::Client;
use crate::models::Id;

pub const NAME: &'static str = "query";

pub fn parser<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(NAME).arg(
        Arg::with_name("id")
            .value_name("id")
            .required(true)
            .takes_value(true)
            .min_values(1),
    )
}

pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let id = match matches.value_of("id") {
        None => unreachable!("clap will require an argument value"),
        Some(id) => id,
    };
    let id: Id = id.parse()?;

    let client = Client::new("test", "test");
    let item = client.item(id)?;
    println!("item = {:#?}", item);

    let comments = client._comments(id)?;
    println!("comments = {:#?}", comments);

    Ok(())
}

