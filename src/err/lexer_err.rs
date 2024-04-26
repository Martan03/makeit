use std::fmt::Display;

pub enum LexerErr {
    InvalidToken,
    UnclosedLit,
    InvalidIdent,
    UnclosedBlock,
    UnexpectedToken,
}

impl Display for LexerErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerErr::InvalidToken => write!(f, "invalid token found"),
            LexerErr::UnclosedLit => write!(f, "unclosed literal"),
            LexerErr::InvalidIdent => {
                write!(f, "identifier contains invalid characters")
            }
            LexerErr::UnclosedBlock => write!(f, "code block not closed"),
            LexerErr::UnexpectedToken => write!(f, "unexpected token"),
        }
    }
}
