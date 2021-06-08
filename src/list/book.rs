#[derive(Debug, PartialEq)]
pub struct Book(atom_syndication::Entry);

impl std::ops::Deref for Book {
    type Target = atom_syndication::Entry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&atom_syndication::Entry> for Book {
    fn from(entry: &atom_syndication::Entry) -> Self {
        Self(entry.clone())
    }
}
