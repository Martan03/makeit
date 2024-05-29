use std::fmt::Display;

pub enum TemplateErr {
    NotFound(String),
    Creating(String),
    Loading,
    Listing,
    PreExec,
    PostExec,
}

impl Display for TemplateErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateErr::NotFound(n) => write!(f, "template '{n}' not found"),
            TemplateErr::Creating(n) => write!(f, "creating template '{n}'"),
            TemplateErr::Loading => write!(f, "loading template"),
            TemplateErr::Listing => write!(f, "listing templates"),
            TemplateErr::PreExec => write!(f, "executing pre script"),
            TemplateErr::PostExec => write!(f, "executing post script"),
        }
    }
}
