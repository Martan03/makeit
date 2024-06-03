use std::{collections::HashMap, fs::canonicalize, path::PathBuf};

use termint::{
    enums::fg::Fg,
    help,
    widgets::{grad::Grad, span::StrSpanExtension},
};

use crate::err::args_err::ArgsErr;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Create,
    Remove,
    List,
    Help,
}

/// Struct for parsing arguments
#[derive(Debug)]
pub struct Args {
    pub template: Option<String>,
    pub dst: Option<String>,
    pub action: Option<Action>,
    pub vars: HashMap<String, String>,
    pub pre: Option<String>,
    pub post: Option<String>,
    pub yes: bool,
}

impl Args {
    /// Parses arguments
    pub fn parse(args: std::env::Args) -> Result<Args, ArgsErr> {
        let mut parsed = Args::default();

        let mut args_iter = args.into_iter();
        args_iter.next();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-c" | "--create" => parsed.set_action(Action::Create)?,
                "-r" | "--remove" => parsed.set_action(Action::Remove)?,
                "-l" | "--list" => parsed.set_action(Action::List)?,
                "-h" | "--help" => parsed.set_action(Action::Help)?,
                "-d" | "--dir" => parsed.set_path(
                    args_iter.next().ok_or(ArgsErr::MissingParam)?,
                )?,
                "--pre" => {
                    parsed.pre =
                        Some(args_iter.next().ok_or(ArgsErr::MissingParam)?)
                }
                "--post" => {
                    parsed.post =
                        Some(args_iter.next().ok_or(ArgsErr::MissingParam)?)
                }
                "-y" | "--yes" => parsed.yes = true,
                var if var.starts_with("-D") => parsed.parse_var(var),
                name => parsed.set_template(name.to_string())?,
            }
        }
        Ok(parsed)
    }

    /// Checks if template name is provided
    pub fn check_template(&self) -> Result<(), ArgsErr> {
        if self.template.is_none() {
            return Err(ArgsErr::NoTemplate);
        }
        Ok(())
    }

    /// Adds variable if isn't already defined
    pub fn add_var(&mut self, name: &str, value: String) {
        if !self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), value);
        }
    }

    /// Gets destination path
    pub fn get_path(&self) -> PathBuf {
        if let Some(dst) = &self.dst {
            canonicalize(&dst).unwrap_or(PathBuf::from(dst))
        } else {
            canonicalize(".").unwrap_or(PathBuf::from("."))
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
            "-l  --list" => "Lists all templates\n"
            "-r  --remove" => "Remove template with given name\n"
            "-d  --dir" ["path"] =>
                "Sets directory to create/load template from/to\n"
            "--pre" ["script"] =>
                "Sets pre-script to given script (only with '--create')\n"
            "--post" ["script"] =>
                "Sets post-script to given script (only with '--create')\n"
            "-D\x1b[39m[variable name]=[value]" => "Defines a variable\n"
            "-y  --yes" => "Automatically answers yes in yes-no prompts\n"
            "-h   --help" => "Prints this help (other options are ignored)"
        );
    }

    fn set_template(&mut self, template: String) -> Result<(), ArgsErr> {
        if self.template.is_some() {
            Err(ArgsErr::MultipleTemplates)
        } else {
            self.template = Some(template);
            Ok(())
        }
    }

    /// Sets action to given value, returns Err when already set
    fn set_action(&mut self, action: Action) -> Result<(), ArgsErr> {
        if self.action.is_some() {
            Err(ArgsErr::MultipleActions)
        } else {
            self.action = Some(action);
            Ok(())
        }
    }

    /// Sets path to given value, returns Err when already set
    fn set_path(&mut self, path: String) -> Result<(), ArgsErr> {
        if self.dst.is_some() {
            Err(ArgsErr::MultiplePaths)
        } else {
            self.dst = Some(path);
            Ok(())
        }
    }

    /// Parses variable
    fn parse_var(&mut self, arg: &str) {
        let var = &arg[2..];
        if let Some((name, val)) = var.split_once('=') {
            self.vars.insert(name.to_string(), val.to_string());
        } else {
            self.vars.insert(var.to_string(), "".to_string());
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            template: None,
            dst: None,
            action: None,
            yes: false,
            pre: None,
            post: None,
            vars: HashMap::new(),
        }
    }
}
