mod book;
mod item;

pub use book::Book;
pub use item::Item;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Subsection {
    pub title: String,
    pub link: String,
}

pub struct List {
    pub state: tui::widgets::ListState,
    pub items: Vec<Item>,
}

impl List {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            state: tui::widgets::ListState::default(),
        }
    }

    pub fn next(&mut self) {
        self.state.select_next();
    }

    pub fn previous(&mut self) {
        self.state.select_previous();
    }

    pub fn inc(&mut self, nth: usize) {
        let selected = self.state.selected().unwrap_or(nth);
        let i = (selected + nth).min(self.items.len() - 1);
        self.state.select(Some(i));
    }

    pub fn dec(&mut self, nth: usize) {
        let selected = self.state.selected().unwrap_or(0);
        let i = selected.saturating_add(nth);
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn selected(&self) -> Option<&Item> {
        self.state.selected().map(|x| &self.items[x])
    }

    pub fn nth(&self) -> Option<usize> {
        self.state.selected()
    }
}

impl From<atom_syndication::Feed> for List {
    fn from(feed: atom_syndication::Feed) -> Self {
        let mut items = feed
            .entries
            .iter()
            .filter_map(|x| Item::try_from(x).ok())
            .collect::<Vec<_>>();
        items.sort();

        Self {
            items,
            state: tui::widgets::ListState::default(),
        }
    }
}
