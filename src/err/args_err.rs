use std::fmt::Display;

/// Enum representing args error
#[derive(Debug)]
pub enum ArgsErr {
    NoTemplate,
    MultipleTemplates,
    MultipleActions,
    MultiplePaths,
    MissingParam,
}

impl Display for ArgsErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsErr::NoTemplate => write!(f, "no template name provided"),
            ArgsErr::MultipleTemplates => {
                write!(f, "multiple template names provided")
            }
            ArgsErr::MultipleActions => write!(f, "multiple actions provided"),
            ArgsErr::MultiplePaths => write!(f, "multiple paths provided"),
            ArgsErr::MissingParam => write!(f, "missing argument parameter"),
        }
    }
}
