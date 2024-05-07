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

use crate::{
    config::Config,
    err::template_err::TemplateErr,
    file_options::{FileAction, FileOptions},
    parser::Parser,
};

/// Represents makeit template
#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(skip)]
    path: PathBuf,
    pre: Option<PathBuf>,
    post: Option<PathBuf>,
    files_options: HashMap<String, FileOptions>,
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
        Template::copy_files_raw(src, &dst)
            .map_err(|_| TemplateErr::Creating(name.to_string()))?;

        let tmplt = Self {
            path: dir,
            pre: None,
            post: None,
            files_options: HashMap::new(),
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

    fn copy(&mut self, to: &PathBuf) -> Result<(), TemplateErr> {
        let src = self.get_template_dir();
        if let Err(e) = self.copy_files(&src, to) {
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
        &mut self,
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

            let dst_path = dest.join(filename);
            if file_type.is_dir() {
                create_dir(&dst_path)?;
                self.copy_files(&path, &dst_path)?;
            } else {
                self.make_file(&path, &dst_path)?;
            }
        }
        Ok(())
    }

    /// Copies files raw - without parsing
    fn copy_files_raw(
        src: &PathBuf,
        dst: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        for entry in read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            let path = entry.path();
            let Some(filename) = path.file_name() else {
                continue;
            };

            let dst_path = dst.join(filename);
            if file_type.is_dir() {
                create_dir(&dst_path)?;
                Template::copy_files_raw(&path, &dst_path)?;
            } else {
                Template::copy_file(&path, &dst_path)?;
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

    /// Makes file - follows options stored in template config
    fn make_file(
        &self,
        src: &PathBuf,
        dst: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let path = src.to_string_lossy().into_owned();
        let item = match self.files_options.get(&path) {
            Some(i) => i,
            None => return Template::parse_file(src, dst),
        };

        let mut dst = dst.to_owned();
        if let Some(name) = &item.name {
            let mut filename = String::new();
            let mut iter = name.chars().map(Ok);
            let mut parser =
                Parser::string(&mut iter, HashMap::new(), &mut filename);
            parser.parse()?;
            println!("{}", filename)
            // dst.set_file_name(fil);
        }

        match &item.action {
            FileAction::Copy => Template::copy_file(src, &dst),
            FileAction::Make => Template::parse_file(src, &dst),
            FileAction::Ignore => Ok(()),
        }
    }

    /// Copies file from `src` to `dst` without parsing it
    fn copy_file(src: &PathBuf, dst: &PathBuf) -> Result<(), Box<dyn Error>> {
        copy(src, dst)?;
        Ok(())
    }

    /// Copies file from `src` to `dst` with parsing it
    fn parse_file(src: &PathBuf, dst: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut buf = BufReader::new(File::open(src)?);
        let mut chars = buf.chars();
        let mut parser = Parser::file(&mut chars, HashMap::new(), dst)?;
        Ok(parser.parse()?)
    }
}
