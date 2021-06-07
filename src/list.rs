#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Item {
    pub title: String,
    pub link: String,
}

impl std::convert::TryFrom<&atom_syndication::Entry> for Item {
    type Error = ();

    fn try_from(entry: &atom_syndication::Entry) -> Result<Self, Self::Error> {
        let mut link = None;

        for l in &entry.links {
            if l.rel == "subsection" {
                link = Some(l.href.clone());
                break;
            }
        }

        if let Some(link) = link {
            let item = Self {
                title: entry.title.to_string(),
                link,
            };

            Ok(item)
        } else {
            Err(())
        }
    }

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

    pub fn selected(&self) -> Option<&Item> {
        self.state.selected()
            .map(|x| &self.items[x])
    }
}

impl From<atom_syndication::Feed> for List {
    fn from(feed: atom_syndication::Feed) -> Self {
        use std::convert::TryFrom;

        let mut items = feed.entries.iter()
            .filter_map(|x| Item::try_from(x).ok())
            .collect::<Vec<_>>();
        items.sort();

        Self {
            items,
            state: tui::widgets::ListState::default(),
        }
    }
}
