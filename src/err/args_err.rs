use std::fmt::Display;

/// Enum representing args error
pub enum ArgsErr {
    MissingParam,
}

impl Display for ArgsErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsErr::MissingParam => write!(f, "missing argument parameter"),
        }
    }
}
