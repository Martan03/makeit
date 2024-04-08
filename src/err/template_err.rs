use std::{error::Error, fmt::Display};

pub enum TemplateErr {
    Exists(String),
    NotFound(String),
    Creating(String),
    Loading,
}

impl Display for TemplateErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            TemplateErr::Exists(n) => format!("Template '{n}' already exists"),
            TemplateErr::NotFound(n) => format!("Template '{n}' not found"),
            TemplateErr::Creating(n) => format!("Creating template '{n}'"),
            TemplateErr::Loading => format!("Loading template"),
        };
        write!(f, "{msg}")
    }
}
