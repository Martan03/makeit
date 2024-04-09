use std::path::PathBuf;

use termint::{
    enums::fg::Fg,
    help,
    widgets::{grad::Grad, span::StrSpanExtension},
};

use crate::err::args_err::ArgsErr;

/// Struct for parsing arguments
pub struct Args {
    pub template: Option<String>,
    pub dst: Option<PathBuf>,
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
            "makeit" ["Template name"] ["Option"] => "Loads given template\n"
            "Options":
            "-h  --help" => "Prints this help (other options are ignored)"
        );
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            template: None,
            dst: None,
            help: false,
        }
    }
}
