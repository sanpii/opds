#[derive(Clone)]
pub struct Message {
    level: log::Level,
    message: String,
}

impl<'a> From<&'a Message> for tui::widgets::ListItem<'a> {
    fn from(message: &'a Message) -> Self {
        use tui::style::{Color, Style};

        let s = match message.level {
            log::Level::Error => Style::default().fg(Color::Red),
            log::Level::Warn => Style::default().fg(Color::Yellow),
            log::Level::Info => Style::default().fg(Color::Green),
            log::Level::Debug => Style::default().fg(Color::Blue),
            log::Level::Trace => Style::default().fg(Color::Gray),
        };
        let span = tui::text::Spans::from(vec![
            tui::text::Span::styled(format!("{:<9}", message.level), s),
            tui::text::Span::raw(" "),
            tui::text::Span::raw(message.message.as_str()),
        ]);
        tui::widgets::ListItem::new(span)
    }
}

impl<'a> From<&'a log::Record<'a>> for Message {
    fn from(record: &'a log::Record) -> Self {
        Self {
            level: record.level(),
            message: record.args().to_string(),
        }
    }
}

#[derive(Default)]
pub struct Logger {
    messages: std::sync::RwLock<Vec<Message>>,
}

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn messages(&self) -> Vec<Message> {
        self.messages
            .read()
            .unwrap()
            .clone()
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.target() == env!("CARGO_PKG_NAME")
    }

    fn log(&self, record: &log::Record) {
        self.messages
            .write()
            .unwrap()
            .push(record.into());
    }

    fn flush(&self) {
    }
}
