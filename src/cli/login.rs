use std::error::Error;
use crate::client::Client;
use clap::App;
use clap::SubCommand;
use clap::Arg;
use clap::ArgMatches;

/// Login with a given username and password

pub const NAME: &'static str = "login";

pub fn parser<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(NAME)
        .arg(
            Arg::with_name("username")
                .value_name("username")
                .required(true)
                .takes_value(true)
                // .min_values(1),
        )
        // TODO: Ideally this should be a prompted input with no display
        .arg(
            Arg::with_name("password")
                .value_name("password")
                .required(true)
                .takes_value(true)
                // .min_values(1),
        )
}

pub fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let username = matches
        .value_of("username")
        .ok_or("username is required for login")?;
    let password = matches
    .value_of("password")
    .ok_or("password is required for login")?;
    
    let client = Client::new(username, password);
    client.login()?;

    Ok(())
}
    
