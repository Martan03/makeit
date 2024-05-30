use std::env;

use args::Args;
use config::Config;
use err::error::Error;
use termint::{enums::fg::Fg, widgets::span::StrSpanExtension};

use crate::template::Template;

mod args;
mod ast;
mod config;
mod err;
mod file_options;
mod lexer;
mod parser;
mod prompt;
mod template;
mod writer;

fn main() -> Result<(), String> {
    let mut args = Args::parse(std::env::args()).map_err(|_| "args err")?;
    if args.help {
        Args::help();
        return Ok(());
    }

    let config = Config::load()?;
    match args.action {
        args::Action::Load => load(&config, &mut args),
        args::Action::Create => create(&config, &args),
        args::Action::List => Template::list(&config),
    }
    .map_err(|e| e.to_string())
}

fn load(config: &Config, args: &mut Args) -> Result<(), Error> {
    if let Some(name) = args.dst.file_name() {
        args.add_var("_PNAME", name.to_string_lossy().to_string());
    }
    args.add_var("_PDIR", args.dst.to_string_lossy().to_string());
    args.add_var("_OS", env::consts::OS.to_string());

    let Some(template) = &args.template else {
        printe("no template name provided");
        return Ok(());
    };
    Template::load(&config, &args, template)
}

fn create(config: &Config, args: &Args) -> Result<(), Error> {
    let Some(template) = &args.template else {
        printe("no template name provided");
        return Ok(());
    };
    Template::create(&config, &args.dst, template)
}

fn printe(msg: &str) {
    eprintln!("{} {msg}", "Error:".fg(Fg::Red));
}
