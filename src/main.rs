use config::Config;

mod args;
mod config;
mod err;

fn main() -> Result<(), String> {
    let config = Config::load()?;
    println!("{}", config.template_dir.to_str().unwrap_or(""));
    config.save()?;

    Ok(())
}
