pub struct Logs;

impl<'a> super::Widget<'a, tui::widgets::List<'a>> for Logs {
    fn draw(state: &'a crate::State) -> tui::widgets::List<'a> {
        let block = tui::widgets::Block::default()
            .border_type(tui::widgets::BorderType::Rounded)
            .borders(tui::widgets::Borders::ALL)
            .title("Logs");
        let items = state.logs.iter()
            .map(tui::widgets::ListItem::from)
            .collect::<Vec<_>>();

        tui::widgets::List::new(items)
            .block(block)
    }
}
