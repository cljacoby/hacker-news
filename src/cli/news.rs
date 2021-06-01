use std::error::Error;
use clap::App;
use clap::ArgMatches;
use clap::SubCommand;
use grid_printer::GridPrinter;
use grid_printer::style::StyleOpt;
use grid_printer::style::Fg;
use crate::client::Client;

/// Get front page listings of Hacker News.

pub const NAME: &'static str = "news";

pub fn parser<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(NAME)
}

pub fn cmd(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let client = Client::new("", "");
    let listings = client.news()?;
    let grid: Vec<Vec<String>> = listings.into_iter().map(|l| vec![
        l.id.clone().to_string(),
        l.score.clone().unwrap_or(0).to_string(),
        l.user.clone().unwrap_or("".to_string()),
        l.title.clone(),
    ]).collect();

    let rows = grid.len();
    let cols = match grid.get(0) {
        None => return Ok(()),
        Some(first_row) => first_row.len(),
    };
    let printer = GridPrinter::builder(rows, cols)
        .col_spacing(4)
        .col_style(1, StyleOpt::new().fg(Fg::Red))?
        .build();
    printer.print(&grid);

    Ok(())
}
