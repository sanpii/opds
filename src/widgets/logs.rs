pub struct Logs;

impl<'a> super::Widget<'a, tui::widgets::List<'a>> for Logs {
    fn draw(state: &'a crate::State) -> tui::widgets::List<'a> {
        let block = tui::widgets::Block::bordered()
            .border_type(tui::widgets::BorderType::Rounded)
            .title("Logs");

        tui::widgets::List::new(&state.logs).block(block)
    }
}
