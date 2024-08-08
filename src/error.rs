use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("{0}")]
    InvalidArgument(String),
}

impl std::convert::From<jiff::Error> for Error {
    fn from(err: jiff::Error) -> Self {
        Self::InvalidArgument(err.to_string())
    }
}
