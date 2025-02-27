mod errors;
mod events;
mod list;
mod opds;
mod widgets;

use errors::*;
use events::*;
use list::*;
use opds::*;

use clap::Parser;

#[derive(Parser)]
struct Opt {
    #[arg(long)]
    /// basic auth username
    username: Option<String>,
    #[arg(long)]
    /// basic auth password
    password: Option<String>,
    /// OPDS root URL
    url: String,
}

pub struct State {
    ariane: Vec<Subsection>,
    show_debug: bool,
    show_help: bool,
    list: List,
    book: Option<usize>,
}

impl State {
    fn new() -> Self {
        Self {
            ariane: vec![],
            show_debug: cfg!(debug_assertions),
            show_help: false,
            list: List::new(),
            book: None,
        }
    }
}

fn main() -> Result {
    use tui::termion::raw::IntoRawMode;
    use tui::termion::screen::IntoAlternateScreen;
    use widgets::Widget;

    ratatui_simple_logger::init().ok();

    let opt = Opt::parse();
    let mut opds = Opds::new(&opt.url, opt.username, opt.password);
    opds.root();
    let mut state = State::new();
    state.ariane.push(Subsection {
        title: "#".to_string(),
        link: opt.url,
    });

    let events = Events::new();
    let stdout = std::io::stdout().into_raw_mode()?;
    let screen = stdout.into_alternate_screen()?;
    let backend = tui::backend::TermionBackend::new(screen);
    let mut terminal = tui::Terminal::new(backend)?;

    loop {
        if let Some(feed) = opds.next() {
            state.list = List::from(feed);

            if state.ariane.len() >= 2 {
                let prev = &state.ariane[state.ariane.len() - 2];
                state
                    .list
                    .items
                    .insert(0, list::Item::Previous(prev.link.clone()));
            }
        }

        terminal.draw(|f| {
            let layout = tui::layout::Layout::default()
                .margin(1)
                .direction(tui::layout::Direction::Vertical)
                .constraints(
                    [
                        tui::layout::Constraint::Length(3),
                        tui::layout::Constraint::Min(0),
                    ]
                    .as_ref(),
                )
                .split(f.area());

            let block = tui::widgets::Block::bordered()
                .border_type(tui::widgets::BorderType::Rounded);
            let url = tui::widgets::Paragraph::new(
                state
                    .ariane
                    .iter()
                    .map(|x| x.title.clone())
                    .collect::<Vec<_>>()
                    .join("/"),
            )
            .block(block);
            f.render_widget(url, layout[0]);

            let mut npanes = 1;
            if state.show_debug {
                npanes += 1;
            }
            if state.book.is_some() {
                npanes += 1;
            }
            if state.show_help {
                npanes += 1;
            }

            let constrains =
                vec![tui::layout::Constraint::Percentage(100 / npanes); npanes as usize];
            let main = tui::layout::Layout::default()
                .direction(tui::layout::Direction::Horizontal)
                .constraints(constrains)
                .split(layout[1]);

            let mut area = 0;

            let block = tui::widgets::Block::bordered()
                .border_type(tui::widgets::BorderType::Rounded);

            let widgets = tui::widgets::List::new(&state.list.items)
                .block(block)
                .scroll_padding(1)
                .highlight_style(
                    tui::style::Style::default().add_modifier(tui::style::Modifier::BOLD),
                )
                .highlight_symbol("> ");
            f.render_stateful_widget(widgets, main[area], &mut state.list.state);
            area += 1;

            if state.book.is_some() {
                f.render_widget(widgets::Details::draw(&state), main[area]);
                area += 1;
            }

            if state.show_debug {
                f.render_widget(ratatui_simple_logger::Widget::new(), main[area]);
                area += 1;
            }

            if state.show_help {
                f.render_widget(widgets::Help::draw(&state), main[area]);
            }
        })?;

        if let Ok(key) = events.next() {
            use tui::termion::event::Key::*;

            match key {
                Char('d') => state.show_debug = !state.show_debug,
                Char('h') => state.show_help = !state.show_help,
                Char('q') => break,
                Char('\n') => {
                    if let Some(item) = state.list.selected() {
                        match item {
                            Item::Book(_) => state.book = state.list.nth(),
                            Item::Previous(link) => {
                                state.ariane.pop();
                                opds.send(link);
                            }
                            Item::Subsection(subsection) => {
                                state.ariane.push(subsection.clone());
                                opds.send(&subsection.link);
                            }
                        }
                    }
                }
                Esc => state.list.unselect(),
                Down => {
                    state.book = None;
                    state.list.next();
                }
                Up => {
                    state.book = None;
                    state.list.previous();
                }
                PageDown => {
                    state.book = None;
                    state.list.inc(10);
                }
                PageUp => {
                    state.book = None;
                    state.list.dec(10);
                }
                _ => (),
            }
        };
    }

    Ok(())
}
