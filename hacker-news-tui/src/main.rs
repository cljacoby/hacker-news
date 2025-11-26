use chrono::DateTime;
use color_eyre::Result;
use hacker_news::api::derived::Listing;
use hacker_news::client::Client;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, BorderType, Borders, Cell, Padding, Row, Table},
    DefaultTerminal, Frame,
};

struct App {
    listings: Vec<Listing>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let hn_client = Client::new();
    let top = hn_client.top_stories().await.unwrap();
    let listings: Vec<Listing> = hn_client
        .items(&top[..30])
        .await
        .unwrap()
        .into_iter()
        .filter_map(|item| Listing::try_from(item).ok())
        .collect();

    color_eyre::install()?;
    let terminal = ratatui::init();
    // let result = run(terminal);
    let app = App { listings };
    let result = run(terminal, app);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app: App) -> Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, &app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break Ok(());
            }
        }
    }
}

fn listing_to_rows(idx: usize, listing: &Listing, index_width: usize) -> Vec<Row<'static>> {
    let number = format!("{:>width$}.", idx + 1, width = index_width);

    let title = listing.title.clone();
    let author = listing.by.as_deref().unwrap_or("unknown");
    let comments = listing.kids.as_ref().map(|k| k.len()).unwrap_or(0);
    let timestamp = {
        let dt =
            DateTime::from_timestamp(listing.time as i64, 0).expect("failed to parse datetime");
        dt.format("%Y-%m-%d").to_string()
    };

    let title = Line::from(title);
    // todo: should this just be a Vec<Cell> instead of string formatting?
    let meta_text = match listing.score {
        Some(score) => format!(
            "{} points | by {} | {} | {} comments",
            score, author, timestamp, comments
        ),
        None => format!("by {} | {} | {} comments", author, timestamp, comments),
    };
    let meta = Line::from(meta_text);

    vec![
        Row::new(vec![Cell::from(number), Cell::from(title)]),
        Row::new(vec![Cell::from(""), Cell::from(meta)]),
    ]
}

fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let vertical = Layout::vertical([
        Constraint::Percentage(5),
        Constraint::Percentage(90),
        Constraint::Percentage(5),
    ])
    .split(area);

    let middle = vertical[1];

    let horizontal = Layout::horizontal([
        Constraint::Percentage(5),
        Constraint::Percentage(90),
        Constraint::Percentage(5),
    ])
    .split(middle);

    let box_area = horizontal[1];

    // width of the largest index ("30" -> 2 digits, etc.)
    let index_width = app.listings.len().to_string().len();

    let rows: Vec<Row> = app
        .listings
        .iter()
        .enumerate()
        .map(|(idx, listing)| listing_to_rows(idx, listing, index_width))
        .flatten()
        .collect();

    let block = Block::default()
        .title("Hacker News")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));

    let table = Table::new(
        rows,
        [
            // index column: width of digits + ". "
            Constraint::Length(index_width as u16 + 2),
            Constraint::Min(10),
        ],
    )
    .block(block)
    .column_spacing(1);

    frame.render_widget(table, box_area);
}
