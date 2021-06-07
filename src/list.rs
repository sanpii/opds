#[derive(Debug, PartialEq, Eq, Ord)]
pub enum Item {
    Book(Book),
    Subsection(Subsection),
}

impl Item {
    pub fn link(&self) -> &str {
        match self {
            Self::Book(book) => &book.link,
            Self::Subsection(subsection) => &subsection.link,
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Book(_), Self::Subsection(_)) => Some(std::cmp::Ordering::Less),
            (Self::Book(a), Self::Book(b)) => a.partial_cmp(b),
            (Self::Subsection(_), Self::Book(_)) => Some(std::cmp::Ordering::Greater),
            (Self::Subsection(a), Self::Subsection(b)) => a.partial_cmp(b),
        }
    }
}

impl std::convert::TryFrom<&atom_syndication::Entry> for Item {
    type Error = ();

    fn try_from(entry: &atom_syndication::Entry) -> Result<Self, Self::Error> {
        let mut item = None;

        for l in &entry.links {
            if l.rel == "subsection" {
                let subsection = Subsection {
                    link: l.href.clone(),
                    title: entry.title.to_string(),
                };

                item = Some(Self::Subsection(subsection));
                break;
            } else {
                let book = Book {
                    link: l.href.clone(),
                    title: entry.title.to_string(),
                };

                item = Some(Self::Book(book));
            }
        }

        if let Some(item) = item {
            Ok(item)
        } else {
            Err(())
        }
    }

}

impl<'a> From<&'a Item> for tui::text::Text<'a> {
    fn from(item: &'a Item) -> Self {
        match item {
            Item::Book(book) => format!("📕 {}", book.title).into(),
            Item::Subsection(subsection) => format!("📚 {}", subsection.title).into(),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Book {
    pub title: String,
    pub link: String,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
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
