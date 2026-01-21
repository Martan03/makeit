use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Pareg(#[from] pareg::ArgError),
    #[error("{0}")]
    Msg(String),
    #[error("exit")]
    Exit,
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Msg(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Msg(value.to_string())
    }
}
