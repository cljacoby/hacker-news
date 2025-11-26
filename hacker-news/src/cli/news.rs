use crate::client::Client;
use clap::App;
use clap::ArgMatches;
use clap::SubCommand;

use crate::cli::HnCommand;
use crate::error::HnError;

// TODO: Should put item -> story conversion within Client code
use crate::api::{Item, Story};

/// Get front page listings of Hacker News.
pub struct News;

impl HnCommand for News {
    const NAME: &'static str = "news";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
    }

    async fn cmd(_matches: &ArgMatches<'_>) -> Result<(), Box<HnError>> {
        let hn_client = Client::new();
        let top = hn_client.top_stories().await.unwrap();
        let stories: Vec<Story> = hn_client
            .items(&top[..30])
            .await
            .unwrap()
            .into_iter()
            .filter_map(|item| match item {
                Item::Story(story) => Some(story),
                _ => None,
            })
            .collect();
        tracing::debug!("stories: {:?}", stories);

        for story in stories {
            println!(
                "{id}|{title}|{by}",
                id = story.id,
                title = story.title,
                by = story.by.unwrap()
            );
        }

        Ok(())
    }
}
