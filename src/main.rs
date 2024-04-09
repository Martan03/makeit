use args::Args;
use config::Config;
use err::template_err::TemplateErr;
use termint::{enums::fg::Fg, widgets::span::StrSpanExtension};

use crate::template::Template;

mod args;
mod config;
mod err;
mod template;

fn main() -> Result<(), String> {
    let args = Args::parse(std::env::args()).map_err(|_| "args err")?;
    if args.help {
        Args::help();
        return Ok(());
    }

    let config = Config::load()?;
    match args.action {
        args::Action::Load => load(&config, &args),
        args::Action::Create => create(&config, &args),
        args::Action::List => Template::list(&config),
    }
    .map_err(|e| e.to_string())?;

    Ok(())
}

fn load(config: &Config, args: &Args) -> Result<(), TemplateErr> {
    let Some(template) = &args.template else {
        printe("no template name provided");
        return Ok(());
    };
    Template::load(&config, &args.dst, template)
}

fn create(config: &Config, args: &Args) -> Result<(), TemplateErr> {
    let Some(template) = &args.template else {
        printe("no template name provided");
        return Ok(());
    };
    Template::create(&config, &args.dst, template)
}

fn printe(msg: &str) {
    eprintln!("{} {msg}", "Error:".fg(Fg::Red));
}
