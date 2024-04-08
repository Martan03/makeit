use config::Config;

use crate::template::Template;

mod args;
mod config;
mod err;
mod template;

fn main() -> Result<(), String> {
    let config = Config::load()?;

    let tmplt = Template::load(&config, "test").map_err(|e| e.to_string())?;
    _ = tmplt.pre_exec();
    // tmplt
    //     .copy(&PathBuf::from("./test"))
    //     .map_err(|e| e.to_string())?;
    _ = tmplt.save();

    Ok(())
}
