use std::path::PathBuf;

use termint::{
    enums::fg::Fg,
    help,
    widgets::{grad::Grad, span::StrSpanExtension},
};

use crate::err::args_err::ArgsErr;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Load,
    Create,
    List,
}

/// Struct for parsing arguments
#[derive(Debug)]
pub struct Args {
    pub template: Option<String>,
    pub dst: PathBuf,
    pub action: Action,
    pub help: bool,
}

impl Args {
    /// Parses arguments
    pub fn parse(args: std::env::Args) -> Result<Args, ArgsErr> {
        let mut parsed = Self::default();

        let mut args_iter = args.into_iter();
        args_iter.next();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-c" | "--create" => parsed.action = Action::Create,
                "-d" | "--dir" => {
                    parsed.dst = PathBuf::from(
                        args_iter.next().ok_or(ArgsErr::MissingParam)?,
                    );
                }
                "-l" | "--list" => parsed.action = Action::List,
                "-h" | "--help" => {
                    parsed.help = true;
                    return Ok(parsed);
                }
                name => parsed.template = Some(name.to_string()),
            }
        }
        Ok(parsed)
    }

    /// Prints help
    pub fn help() {
        println!(
            "Welcome in {} by {}\n",
            "makeit".fg(Fg::Green),
            Grad::new("Martan03", (0, 220, 255), (175, 80, 255))
        );
        help!(
            "Usage":
            "makeit" ["template name"] ["options"] => "Loads given template\n"
            "makeit" ["options"] => "Behaves according to the options\n"
            "Options":
            "-c  --create" => "Creates new template with given name\n"
            "-d --dir" ["path"] =>
                "Sets directory to create/load template from/to\n"
            "-h  --help" => "Prints this help (other options are ignored)"
        );
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            template: None,
            dst: PathBuf::from("."),
            action: Action::Load,
            help: false,
        }
    }
}
