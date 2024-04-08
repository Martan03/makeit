use std::{
    error::Error,
    fs::{copy, create_dir, create_dir_all, read_dir, read_to_string, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

use serde::{Deserialize, Serialize};

use crate::{config::Config, err::template_err::TemplateErr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(skip)]
    path: PathBuf,
    pre: Option<PathBuf>,
    post: Option<PathBuf>,
}

impl Template {
    /// Creates new template with given name
    pub fn new(config: &Config, name: &str) -> Result<Self, TemplateErr> {
        let path = config.template_dir.join(name);

        if path.exists() {
            return Err(TemplateErr::Exists(name.to_string()));
        }

        create_dir_all(&path)
            .map_err(|_| TemplateErr::Creating(name.to_string()))?;
        Ok(Self {
            path,
            pre: Some(config.template_dir.join("pre")),
            post: None,
        })
    }

    /// Loads template by given name
    pub fn load(config: &Config, name: &str) -> Result<Self, TemplateErr> {
        let path = config.template_dir.join(name);

        if !path.exists() {
            return Err(TemplateErr::NotFound(name.to_string()));
        }

        let tmpl_path = path.join(format!("{}.makeit.json", name));
        let json = read_to_string(&tmpl_path).unwrap_or(String::new());
        match serde_json::from_str::<Template>(&json) {
            Ok(tmplt) => Ok(tmplt),
            Err(_) => Ok(Self {
                path: path,
                pre: None,
                post: None,
            }),
        }
    }

    /// Saves the template
    pub fn save(&self) -> Result<(), String> {
        create_dir_all(&self.path).map_err(|e| e.to_string())?;

        let name = self
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("tmplt")?;
        let path = self.path.join(format!("{}.makeit.json", name));
        let mut file = File::create(&path).map_err(|e| e.to_string())?;

        let json_string =
            serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        file.write_all(json_string.as_bytes())
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn copy(&self, to: &PathBuf) -> Result<(), String> {
        create_dir_all(&to).map_err(|e| e.to_string())?;
        let src = self.get_template_dir();
        Template::copy_files(&src, to).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn pre_exec(&self) -> Result<(), String> {
        let Some(pre) = &self.pre else {
            return Ok(());
        };
        let status = Command::new(pre).status().map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Gets template directory path
    fn get_template_dir(&self) -> PathBuf {
        self.path.join("template")
    }

    /// Copies files recursively
    fn copy_files(
        src: &PathBuf,
        dest: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        for entry in read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            let path = entry.path();
            let Some(filename) = path.file_name() else {
                continue;
            };
            let dest_path = dest.join(filename);

            if file_type.is_dir() {
                create_dir(&dest_path)?;
                Template::copy_files(&path, &dest_path)?;
            } else {
                let dest_path = dest.join(filename);
                copy(&path, &dest_path).unwrap();
            }
        }
        Ok(())
    }
}
