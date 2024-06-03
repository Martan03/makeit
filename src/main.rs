use std::{env, process};

use args::{Action, Args};
use config::Config;
use err::error::Error;
use termint::{enums::fg::Fg, widgets::span::StrSpanExtension};

use crate::template::Template;

mod args;
mod config;
mod err;
mod file_options;
mod parse;
mod prompt;
mod template;
mod writer;

fn main() {
    if let Err(e) = run() {
        println!("{} {e}", "Error:".fg(Fg::Red));
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let mut args = Args::parse(std::env::args())?;

    let config = Config::load()?;
    match args.action {
        Some(Action::Create) => create(&config, args),
        Some(Action::Remove) => remove(&config, &args),
        Some(Action::List) => Template::list(&config),
        Some(Action::Help) => Ok(Args::help()),
        _ => load(&config, &mut args),
    }
}

fn load(config: &Config, args: &mut Args) -> Result<(), Error> {
    args.check_template()?;

    let dst = args.get_path();
    if let Some(name) = dst.file_name() {
        args.add_var("_PNAME", name.to_string_lossy().to_string());
    }
    args.add_var("_PDIR", dst.to_string_lossy().to_string());
    args.add_var("_OS", env::consts::OS.to_string());

    Template::load(config, args)
}

fn create(config: &Config, args: Args) -> Result<(), Error> {
    args.check_template()?;
    Template::create(config, args)
}

fn remove(config: &Config, args: &Args) -> Result<(), Error> {
    args.check_template()?;
    Template::remove(config, args)
}
