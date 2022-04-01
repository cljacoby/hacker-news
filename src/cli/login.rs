use crate::client::html_client::Client;
use clap::App;
use clap::SubCommand;
use clap::Arg;
use clap::ArgMatches;

use crate::cli::HnCommand;
use crate::error::HnError;

/// Login with a given username and password


pub struct Login;

impl HnCommand for Login {
    const NAME: &'static str = "login";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
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

    fn cmd(matches: &ArgMatches) -> Result<(), Box<HnError>> {
        let username = matches
            .value_of("username")
            .ok_or(Box::new(HnError::ArgumentError(Some("username not received"))))?;
        let password = matches
            .value_of("password")
            .ok_or(Box::new(HnError::ArgumentError(Some("password not received"))))?;
        
        let client = Client::new();
        client.login(username, password)
            .map_err(|_src| HnError::AuthenticationError)?;

        Ok(())
    }

}

    
