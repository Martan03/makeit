use std::fmt::Display;

pub enum TemplateErr {
    NotFound(String),
    PreExec,
    PostExec,
}

impl Display for TemplateErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateErr::NotFound(n) => write!(f, "template '{n}' not found"),
            TemplateErr::PreExec => write!(f, "executing pre script"),
            TemplateErr::PostExec => write!(f, "executing post script"),
        }
    }
}
