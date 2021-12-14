use std::error::Error;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use crate::model::Id;
use crate::cli::HnCommand;
use async_trait::async_trait;

pub struct Tree;

#[async_trait]
impl HnCommand for Tree {
    const NAME: &'static str = "tree";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .arg(
                Arg::with_name("id")
                    .value_name("id")
                    .required(true)
                    .takes_value(true)
                    .min_values(1),
            )
            .arg(
                Arg::with_name("timeout")
                    .value_name("timeout")
                    .long("timeout")
                    .short("t")
                    .required(false)
                    .takes_value(true)
                    .min_values(1),
            )
    }

    async fn cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        // Parse command-line argument of HackerNews ID
        let _id: Id = matches
            .value_of("id")
            .ok_or("Id is required for query")?
            .parse()?;

        
        // Instantiate client, and retrieve comment data
        // let mut replies: Vec<Comment> = vec![];
        // let client = HNClient::new();
        // for reply in client.iter_replies(id)? {
        //     let reply = reply?;
        //     replies.push(reply);
        // }
        // println!("{:#?}", replies);
        // Ok(())
        unimplemented!("Re-implement this with HTML based client");
    }
}
