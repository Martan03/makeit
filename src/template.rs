use std::{
    error::Error,
    fs::{copy, create_dir, create_dir_all, read_dir},
    path::PathBuf,
    process::Command,
};

use crate::{config::Config, err::template_err::TemplateErr};

pub struct Template {
    path: PathBuf,
    pre: PathBuf,
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
        let pre = config.template_dir.join("pre");
        Ok(Self { path, pre })
    }

    /// Loads template by given name
    pub fn load(config: &Config, name: &str) -> Result<Self, TemplateErr> {
        let path = config.template_dir.join(name);

        if !path.exists() {
            return Err(TemplateErr::NotFound(name.to_string()));
        }

        let pre = config.template_dir.join("pre");
        Ok(Self { path, pre })
    }

    pub fn copy(&self, to: &PathBuf) -> Result<(), String> {
        create_dir_all(&to).map_err(|e| e.to_string())?;
        let src = self.get_template_dir();
        Template::copy_files(&src, to).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn pre_exec(&self) -> Result<(), String> {
        let status = Command::new(&self.pre)
            .status()
            .map_err(|e| e.to_string())?;

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
