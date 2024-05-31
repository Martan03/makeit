use std::{collections::HashMap, fs::canonicalize, path::PathBuf};

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
    Remove,
    List,
}

/// Struct for parsing arguments
#[derive(Debug)]
pub struct Args {
    pub template: Option<String>,
    pub dst: PathBuf,
    pub action: Action,
    pub help: bool,
    pub vars: HashMap<String, String>,
    pub yes: bool,
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
                "-r" | "--remove" => parsed.action = Action::Remove,
                "-d" | "--dir" => {
                    parsed.dst = Args::parse_path(
                        args_iter.next().ok_or(ArgsErr::MissingParam)?,
                    );
                }
                "-l" | "--list" => parsed.action = Action::List,
                "-h" | "--help" => {
                    parsed.help = true;
                    return Ok(parsed);
                }
                "-y" | "--yes" => parsed.yes = true,
                var if var.starts_with("-D") => {
                    let var = &var[2..];
                    if let Some((name, val)) = var.split_once('=') {
                        parsed.vars.insert(name.to_string(), val.to_string());
                    } else {
                        parsed.vars.insert(var.to_string(), "".to_string());
                    }
                }
                name => parsed.template = Some(name.to_string()),
            }
        }
        Ok(parsed)
    }

    /// Adds variable if isn't already defined
    pub fn add_var(&mut self, name: &str, value: String) {
        if !self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), value);
        }
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
            "-r  --remove" => "Remove template with given name"
            "-d  --dir" ["path"] =>
                "Sets directory to create/load template from/to\n"
            "-D\x1b[39m[variable name]=[value]" => "Defines a variable\n"
            "-l  --list" => "Lists all templates\n"
            "-y  --yes" => "Automatically answers yes in yes-no prompts\n"
            "-h   --help" => "Prints this help (other options are ignored)"
        );
    }

    /// Parses path
    fn parse_path(path: String) -> PathBuf {
        canonicalize(&path).unwrap_or(PathBuf::from(path))
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            template: None,
            dst: Args::parse_path(".".to_string()),
            action: Action::Load,
            help: false,
            yes: false,
            vars: HashMap::new(),
        }
    }
}
