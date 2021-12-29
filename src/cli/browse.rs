use crate::cli::HnCommand;
use crate::client::html_client::Client;
use crate::model::Listing;

use clap::App;
use clap::ArgMatches;
use clap::SubCommand;

use std::error::Error;
use std::io;

use crossterm::event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;

use tui::backend::Backend;
use tui::backend::CrosstermBackend;
use tui::layout::Alignment;
use tui::layout::Constraint;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::style::Color;
use tui::style::Modifier;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::Borders;
// use tui::widgets::BorderType;
// use tui::widgets::List;
// use tui::widgets::ListItem;
// use tui::widgets::ListState;
use tui::widgets::Cell;
use tui::widgets::Paragraph;
use tui::widgets::Row;
use tui::widgets::Table;
use tui::widgets::TableState;
use tui::widgets::Wrap;
use tui::Frame;
use tui::Terminal;

pub struct Browse {
    state: TableState,
    listings: Vec<Listing>,
}

impl Browse {
    fn select_next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.listings.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn select_previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.listings.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn run_tui_loop<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f: &mut Frame<B>| {
                Browse::draw_ui(self, f);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down => self.select_next_row(),
                    KeyCode::Up => self.select_previous_row(),
                    _ => {}
                }
            }
        }
    }

    fn draw_ui<B: Backend>(&mut self, f: &mut Frame<B>) {
        let size = f.size();
        let orange = Color::Rgb(255, 102, 0);

        let rects = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            // .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);

        let header_layout = Layout::default()
            .direction(Direction::Horizontal)
            // .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
            .constraints([Constraint::Length(5), Constraint::Length(10)].as_ref())
            .split(rects[0]);

        let logo_box = Block::default()
            .style(Style::default().fg(Color::White))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(orange));
        let logo = Paragraph::new(vec![Spans::from(Span::styled(
            "Y",
            Style::default().fg(Color::White),
        ))])
        .block(logo_box)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
        f.render_widget(logo, header_layout[0]);

        let header_box = Block::default()
            .style(Style::default().fg(Color::White))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Black));
        let header = Paragraph::new(vec![Spans::from(Span::styled(
            "Hacker News",
            Style::default().fg(Color::White),
        ))])
        .block(header_box)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
        f.render_widget(header, header_layout[1]);

        let rows = self.listings.iter().map(|l| {
            let cells = vec![
                Cell::from(l.score.clone().unwrap_or(0).to_string()),
                Cell::from(l.user.clone().unwrap_or("".to_string())),
                Cell::from(l.title.clone()),
            ];

            Row::new(cells).height(1)
        });

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(orange));

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);

        let table = Table::new(rows)
            .block(block)
            .highlight_style(selected_style)
            .widths(&[
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(80),
            ]);
        f.render_stateful_widget(table, rects[1], &mut self.state);
    }
}

impl HnCommand for Browse {
    const NAME: &'static str = "browse";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
    }

    fn cmd(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        log::info!("Start subcommand browse");

        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Setup browse structure
        let client = Client::new();
        let listings = client.news()?;
        let state = TableState::default();
        let mut browse = Browse { listings, state };

        // Start tui. Infinite loop; user initiated break.
        let res = browse.run_tui_loop(&mut terminal);

        // tui loop exited; restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err)
        }

        Ok(())
    }
}
