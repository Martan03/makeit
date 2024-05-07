use std::{
    collections::HashMap,
    error::Error,
    fs::{copy, create_dir, create_dir_all, read_dir, read_to_string, File},
    io::{BufReader, Write},
    path::PathBuf,
    process::Command,
};

use serde::{Deserialize, Serialize};
use utf8_chars::BufReadCharsExt;

use crate::{config::Config, err::template_err::TemplateErr, parser::Parser};

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(skip)]
    path: PathBuf,
    pre: Option<PathBuf>,
    post: Option<PathBuf>,
}

impl Template {
    /// Creates new template with given name
    pub fn create(
        config: &Config,
        src: &PathBuf,
        name: &str,
    ) -> Result<(), TemplateErr> {
        let dir = config.template_dir.join(name);
        if dir.exists() {
            return Err(TemplateErr::Exists(name.to_string()));
        }

        create_dir_all(&dir)
            .map_err(|_| TemplateErr::Creating(name.to_string()))?;

        let dst = dir.join("template");
        create_dir(&dst)
            .map_err(|_| TemplateErr::Creating(name.to_string()))?;
        Template::copy_files(src, &dst)
            .map_err(|_| TemplateErr::Creating(name.to_string()))?;

        let tmplt = Self {
            path: dir,
            pre: None,
            post: None,
        };
        tmplt
            .save()
            .map_err(|_| TemplateErr::Creating(name.to_string()))?;
        Ok(())
    }

    /// Loads template by given name
    pub fn load(
        config: &Config,
        dst: &PathBuf,
        name: &str,
    ) -> Result<(), TemplateErr> {
        let dir = config.template_dir.join(name);
        if !dir.exists() {
            return Err(TemplateErr::NotFound(name.to_string()));
        }

        let path = dir.join("makeit.json");
        let json = read_to_string(&path).unwrap_or(String::new());
        let mut tmplt = serde_json::from_str::<Template>(&json)
            .map_err(|_| TemplateErr::Loading)?;
        tmplt.path = dir;

        create_dir_all(dst).map_err(|_| TemplateErr::Loading)?;
        tmplt.pre_exec(dst)?;
        tmplt.copy(dst)?;
        tmplt.post_exec(dst)?;

        Ok(())
    }

    /// Lists all templates
    pub fn list(config: &Config) -> Result<(), TemplateErr> {
        Ok(Template::list_tmplts(&config.template_dir)
            .map_err(|_| TemplateErr::Listing)?)
    }

    /// Saves the template
    fn save(&self) -> Result<(), String> {
        let path = self.path.join("makeit.json");
        let mut file = File::create(&path).map_err(|e| e.to_string())?;

        let json_string =
            serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        file.write_all(json_string.as_bytes())
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn copy(&self, to: &PathBuf) -> Result<(), TemplateErr> {
        let src = self.get_template_dir();
        if let Err(e) = Template::copy_files(&src, to) {
            eprintln!("{e}");
        };
        Ok(())
    }

    /// Executes pre script
    fn pre_exec(&self, dst: &PathBuf) -> Result<(), TemplateErr> {
        let Some(pre) = &self.pre else {
            return Ok(());
        };
        Ok(Template::exec_script(pre, dst)
            .map_err(|_| TemplateErr::PreExec)?)
    }

    /// Executes post script
    fn post_exec(&self, dst: &PathBuf) -> Result<(), TemplateErr> {
        let Some(post) = &self.post else {
            return Ok(());
        };
        Ok(Template::exec_script(post, dst)
            .map_err(|_| TemplateErr::PostExec)?)
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
                let mut buf = BufReader::new(File::open(&path)?);
                let mut chars = buf.chars();
                let mut parser =
                    Parser::file(&mut chars, HashMap::new(), &dest_path)?;
                parser.parse()?;
            }
        }
        Ok(())
    }

    fn list_tmplts(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            let path = entry.path();
            let Some(filename) = path.file_name() else {
                continue;
            };

            if file_type.is_dir() {
                println!("{}", filename.to_str().unwrap_or(""));
            }
        }
        Ok(())
    }

    /// Executes script
    fn exec_script(script: &PathBuf, dst: &PathBuf) -> Result<(), String> {
        _ = Command::new(script)
            .current_dir(dst)
            .status()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
