use std::{fmt::Display, io};

use super::{lexer_err::LexerErr, template_err::TemplateErr};

/// Generic error type
pub enum Error {
    IOErr(io::Error),
    LexerErr(LexerErr),
    TemplateErr(TemplateErr),
    Serde(serde_json::Error),
    Msg(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOErr(e) => write!(f, "{e}"),
            Error::LexerErr(e) => write!(f, "{e}"),
            Error::TemplateErr(e) => write!(f, "{e}"),
            Error::Serde(e) => write!(f, "{e}"),
            Error::Msg(m) => write!(f, "{m}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOErr(value)
    }
}

impl From<LexerErr> for Error {
    fn from(value: LexerErr) -> Self {
        Self::LexerErr(value)
    }
}

impl From<TemplateErr> for Error {
    fn from(value: TemplateErr) -> Self {
        Self::TemplateErr(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Msg(value)
    }
}
