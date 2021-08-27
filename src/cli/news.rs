use std::error::Error;
use clap::App;
use clap::ArgMatches;
use clap::SubCommand;
use grid_printer::GridPrinter;
use crate::client::html_client::Client;
use crate::cli::HnCommand;

/// Get front page listings of Hacker News.
pub struct News;

impl HnCommand for News {
    const NAME: &'static str = "news";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
    }

    fn cmd(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let client = Client::new();
        let listings = client.news()?;
        let grid: Vec<Vec<String>> = listings.iter().map(|l| {
            vec![
                format!("{}", l.id),
                match l.score {
                    Some(score) => format!("{}", score),
                    None => "".to_string(),
                },
                match l.user {
                    Some(ref user) => format!("{}", user.clone()),
                    None => "".to_string(),
                },
                format!("{}", l.title)
            ]
        }).collect();

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

