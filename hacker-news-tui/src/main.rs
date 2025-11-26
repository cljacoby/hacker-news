use color_eyre::Result;
use hacker_news::client::Client;
use hacker_news::api::Item;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout,
    // Rect
    },
    // style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

struct App {
    items: Vec<Item>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let hn_client = Client::new();
    let top = hn_client.top_stories().await.unwrap();
    let items: Vec<Item> = hn_client
        .items(&top[..30])
        .await
        .unwrap();

    color_eyre::install()?;
    let terminal = ratatui::init();
    // let result = run(terminal);
    let app = App { items };
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

    let lines: Vec<Line> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, story)| {
            let title = story.title().unwrap_or("<untitled>");
            Line::from(format!("{:>2}. {}", i + 1, title))
        })
        .collect();

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

