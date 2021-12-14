use std::error::Error;
use clap::App;
use clap::ArgMatches;
use clap::SubCommand;
use grid_printer::GridPrinter;
use crate::client::html_client::Client;
use crate::cli::HnCommand;
use async_trait::async_trait;

/// Get front page listings of Hacker News.
pub struct News;

#[async_trait]
impl HnCommand for News {
    const NAME: &'static str = "news";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
    }

    async fn cmd(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let client = Client::new();
        let listings = client.news().await?;
        let grid: Vec<Vec<String>> = listings.into_iter().map(|l| vec![
            l.id.clone().to_string(),
            l.score.unwrap_or(0).to_string(),
            l.user.clone().unwrap_or_else(|| "".to_string()),
            l.title,
        ]).collect();

        let rows = grid.len();
        let cols = match grid.get(0) {
            None => return Ok(()),
            Some(first_row) => first_row.len(),
        };
        let printer = GridPrinter::builder(rows, cols)
            .col_spacing(4)
            .build();
        printer.print(&grid);

        Ok(())
    }

}

