pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Atom(#[from] atom_syndication::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Http(#[from] attohttpc::Error),
    #[error("{0}")]
    Logger(#[from] log::SetLoggerError),
}
