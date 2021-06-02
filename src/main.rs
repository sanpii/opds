mod errors;
mod events;
mod list;
mod opds;
mod widgets;

use errors::*;
use events::*;
use list::*;
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

pub struct State {
    show_help: bool,
    path: String,
    list: List,
}

impl State {
    fn new() -> Self {
        Self {
            show_help: false,
            path: "/".to_string(),
            list: List::new(),
        }
    }
}

fn main() -> Result {
    use termion::raw::IntoRawMode;
    use widgets::Widget;

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
            state.list = List::from(feed);
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

            let mut npanes = 1;
            if state.show_help {
                npanes += 1;
            }

            let constrains = vec![tui::layout::Constraint::Percentage(100 / npanes); npanes as usize];
            let main = tui::layout::Layout::default()
                .direction(tui::layout::Direction::Horizontal)
                .constraints(constrains)
                .split(layout[1]);

            let mut area = 0;

            let block = tui::widgets::Block::default()
                .border_type(tui::widgets::BorderType::Rounded)
                .borders(tui::widgets::Borders::ALL);
            let items = state.list.items.iter()
                .map(|x| tui::widgets::ListItem::new(x.as_str()))
                .collect::<Vec<_>>();
            let widgets = tui::widgets::List::new(items)
                .block(block)
                .highlight_style(
                    tui::style::Style::default()
                        .add_modifier(tui::style::Modifier::BOLD),
                )
                .highlight_symbol("> ");
            f.render_stateful_widget(widgets, main[area], &mut state.list.state);
            area += 1;

            if state.show_help {
                f.render_widget(widgets::Help::draw(&state), main[area]);
            }
        })?;

        if let Ok(key) = events.next() {
            use termion::event::Key::*;

            match key {
                Char('h') => state.show_help = !state.show_help,
                Char('q') => break,
                Left => state.list.unselect(),
                Down => state.list.next(),
                Up => state.list.previous(),
                _ => (),
            }
        };
    }

    Ok(())
}
