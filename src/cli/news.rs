use clap::App;
use clap::ArgMatches;
use clap::SubCommand;
use crate::client::HnClient;

use crate::cli::HnCommand;
use crate::error::HnError;

/// Get front page listings of Hacker News.
pub struct News;

impl HnCommand for News {
    const NAME: &'static str = "news";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
    }

    async fn cmd(_matches: &ArgMatches<'_>) -> Result<(), Box<HnError>> {
        let hn_client = HnClient::new();
        let top = hn_client.top_stories().await.unwrap();
        println!("top = {:#?}", top);

        let mut stories = Vec::with_capacity(top.len());
        for (i, item) in top[..10].iter().enumerate() {
            let story = hn_client.item(*item).await.expect("failed to fetch story");
            println!("fetched story {}/{}, story {:#?}", i, top.len(), story);
            stories.push(story);
        }

        Ok(())
    }

}

