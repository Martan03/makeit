use pareg::Pareg;
use termint::termal::{self, printcln};

use crate::{args::Action, error::Error};

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub action: Action,
}

impl Args {
    pub const APP_NAME: &str = "{{ _PNAME }}";

    pub const VERSION_NUMBER: &str = {
        let v = option_env!("CARGO_PKG_VERSION");
        if let Some(v) = v {
            v
        } else {
            "unknown"
        }
    };

    /// Returns parsed CLI arguments.
    ///
    /// # Errors
    /// Returns an [`Error`] if an issue with parsing arguments occures.
    pub fn parse(mut args: Pareg) -> Result<Self, Error> {
        let mut parsed = Self::default();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" | "help" => {
                    parsed.action = Action::Help;
                    break;
                }
                "-v" | "--version" => {
                    parsed.action = Action::Version;
                    break;
                }
                _ => return args.err_unknown_argument().err()?,
            }
        }
        Ok(parsed)
    }

    /// Prints the help.
    pub fn help() {
        printcln!(
            "Welcome to {'g}{}{'_} by {}{'_}
{'bl}Version {}{'_}

TODO: Edit this help message according to your project.

{'g}Usage{'_}:
  {'c}{}{'_} [{'y}flags{'_}]

{'g}Flags{'_}:
  {'y}-h  --help{'_}
    Displays this help.

  {'y}-v  --version{'_}
    Displays the version number of {'c}{}{'_}.",
            Self::APP_NAME,
            termal::gradient("Martan03", (0, 220, 255), (175, 80, 255)),
            Self::VERSION_NUMBER,
            Self::APP_NAME,
            Self::APP_NAME
        );
    }

    /// Prints the version of the app.
    pub fn version() {
        println!("{} {}", Self::APP_NAME, Self::VERSION_NUMBER);
    }
}
