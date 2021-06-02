pub struct Help;

impl<'a> super::Widget<'a, tui::widgets::Table<'a>> for Help {
    fn draw(_: &'a crate::State) -> tui::widgets::Table<'a> {
        let block = tui::widgets::Block::default()
            .border_type(tui::widgets::BorderType::Rounded)
            .borders(tui::widgets::Borders::ALL)
            .title("Help");

        tui::widgets::Table::new(vec![
            tui::widgets::Row::new(vec!["d", "Show logs"]),
            tui::widgets::Row::new(vec!["q", "Quit"]),
            tui::widgets::Row::new(vec!["h", "Show this help"]),
        ])
            .widths(&[tui::layout::Constraint::Min(2), tui::layout::Constraint::Min(0)])
            .block(block)
    }
}
