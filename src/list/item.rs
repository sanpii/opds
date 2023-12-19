#[derive(Debug, PartialEq)]
pub enum Item {
    Book(crate::list::Book),
    Previous(String),
    Subsection(crate::list::Subsection),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Previous(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Self::Previous(_)) => Some(std::cmp::Ordering::Less),
            (Self::Book(_), Self::Subsection(_)) => Some(std::cmp::Ordering::Greater),
            (Self::Book(a), Self::Book(b)) => a.title.to_string().partial_cmp(&b.title.to_string()),
            (Self::Subsection(_), Self::Book(_)) => Some(std::cmp::Ordering::Less),
            (Self::Subsection(a), Self::Subsection(b)) => a.partial_cmp(b),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Item {}

impl From<&atom_syndication::Entry> for Item {
    fn from(entry: &atom_syndication::Entry) -> Self {
        for l in &entry.links {
            let kind = l.mime_type.clone().map(Type::from).and_then(|x| x.kind);
            if kind == Some(Kind::Navigation) {
                let subsection = crate::list::Subsection {
                    link: l.href.clone(),
                    title: entry.title.to_string(),
                };

                return Self::Subsection(subsection);
            }
        }

        Self::Book(entry.into())
    }
}

impl<'a> From<&'a Item> for tui::text::Text<'a> {
    fn from(item: &'a Item) -> Self {
        match item {
            Item::Book(book) => format!(" {}", book.title.to_string()).into(),
            Item::Previous(_) => "󰛃 ..".to_string().into(),
            Item::Subsection(subsection) => format!("󱉟 {}", subsection.title).into(),
        }
    }
}

struct Type {
    _mime_type: String,
    kind: Option<Kind>,
}

impl From<String> for Type {
    fn from(ty: String) -> Self {
        let mut mime_type = ty.clone();
        let mut kind = None;

        for option in ty.split(';') {
            let parts = option.splitn(2, '=').collect::<Vec<_>>();

            if parts.len() == 1 {
                mime_type = parts[0].to_string();
            } else if parts[0] == "kind" {
                kind = parts[1].try_into().ok();
            }
        }

        Self {
            _mime_type: mime_type,
            kind,
        }
    }
}

#[derive(Eq, PartialEq)]
enum Kind {
    Acquisition,
    Navigation,
}

impl TryFrom<&str> for Kind {
    type Error = ();

    fn try_from(kind: &str) -> Result<Self, Self::Error> {
        let k = match kind {
            "acquisition" => Self::Acquisition,
            "navigation" => Self::Navigation,
            _ => return Err(()),
        };

        Ok(k)
    }
}
