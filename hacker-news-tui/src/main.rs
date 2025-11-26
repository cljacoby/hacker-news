use color_eyre::Result;
use chrono::DateTime;
use hacker_news::api::derived::Listing;
use hacker_news::client::Client;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{
        Alignment,
        Constraint,
        Layout,
        // Rect
    },
    // style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
    DefaultTerminal,
    Frame,
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

fn listing_to_lines(idx: usize, listing: &Listing) -> Vec<Line<'_>> {
    let mut lines: Vec<Line> = Vec::new();
    let title = &listing.title;
    let author = listing.by.as_deref().unwrap_or("unknown");
    let comments = listing.kids.as_ref().map(|k| k.len()).unwrap_or(0);
    let timestamp = {
        let dt = DateTime::from_timestamp(listing.time as i64, 0)
            .expect("failed to parse datetime");
        dt.format("%Y-%m-%d").to_string()
    };

    lines.push(Line::from(format!("{:>2}. {}", idx + 1, title)));
    lines.push(Line::from(format!(
        "by {} | {} | {} comments",
        author, timestamp, comments
    )));

    lines
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

    let mut lines: Vec<Line> = Vec::new();
    for (idx, listing) in app.listings.iter().enumerate() {
        lines.extend_from_slice(&listing_to_lines(idx, listing));
    }

    let block = Block::default()
        .title("Hacker News")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, box_area);
}
