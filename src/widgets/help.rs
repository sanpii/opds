pub struct Help;

impl<'a> super::Widget<'a, tui::widgets::Table<'a>> for Help {
    fn draw(_: &'a crate::State) -> tui::widgets::Table<'a> {
        let block = tui::widgets::Block::default()
            .border_type(tui::widgets::BorderType::Rounded)
            .borders(tui::widgets::Borders::ALL)
            .title("Help");

        tui::widgets::Table::new(vec![
            tui::widgets::Row::new(vec!["Up", "Move backward 1 entry"]),
            tui::widgets::Row::new(vec!["Down", "Move forward 1 entry"]),
            tui::widgets::Row::new(vec!["PgUp", "Move backward 10 entries"]),
            tui::widgets::Row::new(vec!["PgDn", "Move forward 10 entries"]),
            tui::widgets::Row::new(vec![""]),
            tui::widgets::Row::new(vec!["d", "Show logs"]),
            tui::widgets::Row::new(vec!["q", "Quit"]),
            tui::widgets::Row::new(vec!["h", "Show this help"]),
        ])
            .widths(&[tui::layout::Constraint::Min(5), tui::layout::Constraint::Percentage(100)])
            .block(block)
    }
}
