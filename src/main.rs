mod errors;
mod events;
mod opds;

use errors::*;
use events::*;
use opds::*;

use clap::Clap;

#[derive(Clap)]
struct Opt {
    #[clap(long)]
    /// basic auth username
    username: Option<String>,
    #[clap(long)]
    /// basic auth password
    password: Option<String>,
    /// OPDS root URL
    url: String,
}

struct State {
    path: String,
    feed: Option<atom_syndication::Feed>,
}

impl State {
    fn new() -> Self {
        Self {
            path: "/".to_string(),
            feed: None,
        }
    }
}

fn main() -> Result {
    use termion::raw::IntoRawMode;

    let opt = Opt::parse();
    let mut state = State::new();
    let mut opds = Opds::new(&opt.url);
    opds.root();

    let events = Events::new();
    let stdout = std::io::stdout().into_raw_mode()?;
    let screen = termion::screen::AlternateScreen::from(stdout);
    let backend = tui::backend::TermionBackend::new(screen);
    let mut terminal = tui::Terminal::new(backend)?;

    loop {
        if let Some(feed) = opds.next() {
            state.feed = Some(feed);
        }

        terminal.draw(|f| {
            let layout = tui::layout::Layout::default()
                .margin(1)
                .direction(tui::layout::Direction::Vertical)
                .constraints([
                    tui::layout::Constraint::Length(3),
                    tui::layout::Constraint::Min(0),
                ].as_ref())
                .split(f.size());

            let block = tui::widgets::Block::default()
                .border_type(tui::widgets::BorderType::Rounded)
                .borders(tui::widgets::Borders::ALL);
            let url = tui::widgets::Paragraph::new(state.path.clone())
                .block(block);
            f.render_widget(url, layout[0]);

            if let Some(feed) = &state.feed {
                let block = tui::widgets::Block::default()
                    .border_type(tui::widgets::BorderType::Rounded)
                    .borders(tui::widgets::Borders::ALL);
                let mut entries = feed.entries.iter()
                    .map(|x| x.title.clone())
                    .collect::<Vec<_>>();
                entries.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                let items = entries.iter()
                    .map(|x| tui::widgets::ListItem::new(x.as_str()))
                    .collect::<Vec<_>>();
                let list = tui::widgets::List::new(items)
                    .block(block)
                    .highlight_style(
                        tui::style::Style::default()
                            .bg(tui::style::Color::LightGreen)
                            .add_modifier(tui::style::Modifier::BOLD),
                    )
                    .highlight_symbol(">");
                f.render_widget(list, layout[1]);
            }
        })?;

        if let Ok(key) = events.next() {
            match key {
                termion::event::Key::Char('q') => {
                    break;
                }
                _ => {}
            }
        };
    }

    Ok(())
}
