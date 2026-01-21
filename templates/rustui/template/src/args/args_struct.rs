use pareg::Pareg;

use crate::{args::action::Action, error::Error};

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub action: Action,
}

impl Args {
    /// Returns parsed CLI arguments
    ///
    /// # Errors
    /// Returns an [`Error`] if an issue with parsing arguments occures.
    pub fn parse(mut args: Pareg) -> Result<Self, Error> {
        let mut parsed = Self::default();
        match args.peek() {
            Some("-h") | Some("--help") | Some("help") => {
                parsed.action = Action::Help
            }
            Some(arg) => return Err(format!("unknown argument: {}", arg).into()),
            None => {}
        }
        Ok(parsed)
    }
}
