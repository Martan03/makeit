use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(alias = "templateDir")]
    pub template_dir: PathBuf,
}

impl Config {
    /// Loads config
    pub fn load() -> Result<Self, String> {
        let mut dir = Config::get_dir();
        dir.push("config.json");

        match serde_json::from_str::<Self>(
            &read_to_string(&dir).unwrap_or(String::new()),
        ) {
            Ok(conf) => Ok(conf),
            Err(_) => Ok(Self::default()),
        }
    }

    /// Saves config
    pub fn _save(&self) -> Result<(), String> {
        let mut dir = Config::get_dir();
        create_dir_all(&dir).map_err(|e| e.to_string())?;

        dir.push("config.json");
        let mut file = File::create(&dir).map_err(|e| e.to_string())?;

        let json_string =
            serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        file.write_all(json_string.as_bytes())
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Gets config dir
    fn get_dir() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or(PathBuf::from("."));
        path.push("makeit");
        path
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut dir = Config::get_dir();
        dir.push("templates");
        Self { template_dir: dir }
    }
}
