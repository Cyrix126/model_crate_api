use thiserror::Error;

/// thiserror Error struct, possibles Errors returned by this client library.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ErrorRequest(#[from] reqwest::Error),
}
