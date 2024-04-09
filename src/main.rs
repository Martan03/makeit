use std::{fs::create_dir_all, path::PathBuf};

use args::Args;
use config::Config;

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
    if let Some(template) = args.template {
        let dst = PathBuf::from("../testing");
        let tmplt =
            Template::load(&config, &template).map_err(|e| e.to_string())?;
        _ = create_dir_all(&dst);
        tmplt.pre_exec(&dst)?;
        tmplt.copy(&dst)?;
        tmplt.post_exec(&dst)?;
    }

    // let tmplt = Template::new(&config, "test").map_err(|e| e.to_string())?;
    // _ = tmplt.pre_exec();
    // tmplt
    //     .copy(&PathBuf::from("./test"))
    //     .map_err(|e| e.to_string())?;
    // _ = tmplt.save();

    Ok(())
}
