use clap::App;
use clap::ArgMatches;
use clap::SubCommand;
// use grid_printer::GridPrinter;
// use crate::client::html_client::Client;
use crate::client::json_client::JsonClient as Client;
use crate::model::firebase::Item;

use crate::cli::HnCommand;
use crate::error::HnError;

/// Get front page listings of Hacker News.
pub struct News;

impl HnCommand for News {
    const NAME: &'static str = "news";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
    }

    fn cmd(_matches: &ArgMatches) -> Result<(), Box<HnError>> {
        let client = Client::new();
        let top = client.top_stories().unwrap();
        println!("top = {:#?}", top);

        let mut stories = Vec::with_capacity(top.len());
        for (i, item) in top[..10].iter().enumerate() {
            let story = client.item(*item).expect("failed to fetch story");
            println!("fetched story {}/{}, story {:?}", i, top.len(), story);
            stories.push(story);
        }

        // let listings = client.news()?;
        // let grid: Vec<Vec<String>> = listings.into_iter().map(|l| vec![
        //     l.id.clone().to_string(),
        //     l.score.unwrap_or(0).to_string(),
        //     l.user.clone().unwrap_or_else(|| "".to_string()),
        //     l.title,
        // ]).collect();

        // let rows = grid.len();
        // let cols = match grid.get(0) {
        //     None => return Ok(()),
        //     Some(first_row) => first_row.len(),
        // };
        // let printer = GridPrinter::builder(rows, cols)
        //     .col_spacing(4)
        //     .build();
        // printer.print(&grid);

        Ok(())
    }

}

