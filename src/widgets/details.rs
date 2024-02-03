pub struct Details;

impl<'a> super::Widget<'a, tui::widgets::Table<'a>> for Details {
    fn draw(state: &'a crate::State) -> tui::widgets::Table<'a> {
        let block = tui::widgets::Block::bordered()
            .border_type(tui::widgets::BorderType::Rounded)
            .title("Details");

        let book = match state.list.selected() {
            Some(crate::Item::Book(book)) => book,
            _ => unreachable!(),
        };

        let mut rows = vec![
            tui::widgets::Row::new(vec!["title:", &book.title.value]),
            tui::widgets::Row::new(vec!["summary:", book.summary.as_deref().unwrap_or("-")]),
        ];

        rows.push(tui::widgets::Row::new(vec!["authors:"]));
        for author in &book.authors {
            rows.push(tui::widgets::Row::new(vec!["", &author.name]));
        }

        rows.push(tui::widgets::Row::new(vec!["categories:"]));
        for category in &book.categories {
            rows.push(tui::widgets::Row::new(vec!["", &category.term]));
        }

        rows.push(tui::widgets::Row::new(vec!["links:"]));
        for link in &book.links {
            rows.push(tui::widgets::Row::new(vec!["", &link.href]));
        }

        tui::widgets::Table::new(
            rows,
            [
                tui::layout::Constraint::Min(11),
                tui::layout::Constraint::Percentage(100),
            ],
        )
        .block(block)
    }
}
