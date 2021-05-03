pub struct List {
    pub state: tui::widgets::ListState,
    pub items: Vec<String>,
}

impl List {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            state: tui::widgets::ListState::default(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl From<atom_syndication::Feed> for List {
    fn from(feed: atom_syndication::Feed) -> Self {
        let mut items = feed.entries.iter()
            .map(|x| x.title.clone())
            .collect::<Vec<_>>();
        items.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        Self {
            items,
            state: tui::widgets::ListState::default(),
        }
    }
}
