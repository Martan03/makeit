use std::{
    collections::HashMap,
    fs::{
        copy, create_dir, create_dir_all, read_dir, read_to_string,
        remove_dir_all, File,
    },
    io::{self, BufReader, Write},
    path::PathBuf,
    process::Command,
};

use serde::{Deserialize, Serialize};
use utf8_chars::BufReadCharsExt;

use crate::{
    args::Args,
    config::Config,
    err::{error::Error, template_err::TemplateErr},
    file_options::{FileAction, FileOptions},
    parser::Parser,
    prompt::yes_no,
};

/// Represents makeit template
#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(skip)]
    path: PathBuf,
    pre: Option<PathBuf>,
    post: Option<PathBuf>,
    file_options: HashMap<String, FileOptions>,
    vars: HashMap<String, String>,
}

impl Template {
    /// Creates new template with given name
    pub fn create(
        config: &Config,
        src: &PathBuf,
        name: &str,
    ) -> Result<(), Error> {
        let dir = config.template_dir.join(name);
        if dir.exists() {
            println!("Template '{name}' already exists.");
            if !yes_no("Do you want to replace it?") {
                return Ok(());
            }
            remove_dir_all(&dir)?;
        }

        let dst = dir.join("template");
        create_dir_all(&dst)?;

        Template::copy_files_raw(src, &dst)?;

        let tmplt = Self {
            path: dir,
            pre: None,
            post: None,
            file_options: HashMap::new(),
            vars: HashMap::new(),
        };
        tmplt.save()
    }

    /// Loads template by given name
    pub fn load(
        config: &Config,
        args: &Args,
        name: &str,
    ) -> Result<(), Error> {
        let dir = config.template_dir.join(name);
        if !dir.exists() {
            return Err(TemplateErr::NotFound(name.to_string()).into());
        }

        if args.dst.exists() && args.dst.read_dir()?.next().is_some() {
            println!("Directory is not empty.");
            if !yes_no("Do you want to continue anyway?") {
                return Ok(());
            }
        }

        let path = dir.join("makeit.json");
        let json = read_to_string(&path).unwrap_or(String::new());
        let mut tmplt = serde_json::from_str::<Template>(&json)?;
        tmplt.path = dir;

        for (name, value) in args.vars.iter() {
            if !tmplt.vars.contains_key(name) {
                tmplt.vars.insert(name.to_string(), value.to_string());
            }
        }

        create_dir_all(&args.dst)?;
        tmplt.pre_exec(&args.dst)?;
        tmplt.copy(&args.dst)?;
        tmplt.post_exec(&args.dst)
    }

    /// Lists all templates
    pub fn list(config: &Config) -> Result<(), Error> {
        Template::list_tmplts(&config.template_dir)
    }

    /// Saves the template
    fn save(&self) -> Result<(), Error> {
        let path = self.path.join("makeit.json");
        let mut file = File::create(&path)?;

        let json_string = serde_json::to_string_pretty(self)?;
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }

    fn copy(&mut self, to: &PathBuf) -> Result<(), Error> {
        let src = self.get_template_dir();
        self.copy_files(&src, to)
    }

    /// Executes pre script
    fn pre_exec(&self, dst: &PathBuf) -> Result<(), Error> {
        let Some(pre) = &self.pre else {
            return Ok(());
        };
        Template::exec_script(pre, dst)
            .map_err(|_| TemplateErr::PreExec.into())
    }

    /// Executes post script
    fn post_exec(&self, dst: &PathBuf) -> Result<(), Error> {
        let Some(post) = &self.post else {
            return Ok(());
        };
        Template::exec_script(post, dst)
            .map_err(|_| TemplateErr::PostExec.into())
    }

    /// Gets template directory path
    fn get_template_dir(&self) -> PathBuf {
        self.path.join("template")
    }

    /// Copies files recursively
    fn copy_files(
        &mut self,
        src: &PathBuf,
        dst: &PathBuf,
    ) -> Result<(), Error> {
        for entry in read_dir(src)? {
            let path = entry?.path();
            let Some(filename) = path.file_name() else {
                continue;
            };

            let dst_path = dst.join(filename);
            if path.is_dir() {
                Template::create_dir(&dst_path)?;
                self.copy_files(&path, &dst_path)?;
            } else {
                self.make_file(&path, &dst_path)?;
            }
        }
        Ok(())
    }

    /// Copies files raw - without parsing
    fn copy_files_raw(src: &PathBuf, dst: &PathBuf) -> Result<(), Error> {
        for entry in read_dir(src)? {
            let path = entry?.path();
            let Some(filename) = path.file_name() else {
                continue;
            };

            let dst_path = dst.join(filename);
            if path.is_dir() {
                Template::create_dir(&dst_path)?;
                Template::copy_files_raw(&path, &dst_path)?;
            } else {
                Template::copy_file(&path, &dst_path)?;
            }
        }
        Ok(())
    }

    fn list_tmplts(dir: &PathBuf) -> Result<(), Error> {
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
    fn make_file(&self, src: &PathBuf, dst: &PathBuf) -> Result<(), Error> {
        let path = src.to_string_lossy().into_owned();
        let item = match self.file_options.get(&path) {
            Some(i) => i,
            // TODO: deside which option will be default (copy/make)
            None => return Template::copy_file(src, dst),
        };

        let mut dst = dst.to_owned();
        if let Some(name) = &item.name {
            let mut filename = String::new();
            let mut iter = name.chars().map(Ok);
            let mut parser =
                Parser::string(&mut iter, &self.vars, &mut filename);
            parser.parse()?;
            dst.set_file_name(filename);
        }

        match &item.action {
            FileAction::Copy => Template::copy_file(src, &dst),
            FileAction::Make => self.parse_file(src, &dst),
            FileAction::Ignore => Ok(()),
        }
    }

    /// Copies file from `src` to `dst` without parsing it
    fn copy_file(src: &PathBuf, dst: &PathBuf) -> Result<(), Error> {
        copy(src, dst)?;
        Ok(())
    }

    /// Copies file from `src` to `dst` with parsing it
    fn parse_file(&self, src: &PathBuf, dst: &PathBuf) -> Result<(), Error> {
        let mut buf = BufReader::new(File::open(src)?);
        let mut chars = buf.chars();
        let mut parser = Parser::file(&mut chars, &self.vars, dst)?;
        Ok(parser.parse()?)
    }

    /// Creates dir when doesn't exist
    fn create_dir(path: &PathBuf) -> io::Result<()> {
        match create_dir(path) {
            Ok(()) => Ok(()),
            Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(()),
            Err(e) => Err(e),
        }
    }
}
