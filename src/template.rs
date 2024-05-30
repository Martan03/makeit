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
use shell_words::split;
use utf8_chars::BufReadCharsExt;

use crate::{
    args::Args,
    config::Config,
    err::{error::Error, template_err::TemplateErr},
    file_options::{FileAction, FileOptions},
    parse::parser::Parser,
    prompt::yes_no,
};

/// Represents makeit template
#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(skip)]
    path: PathBuf,
    pre: Option<String>,
    post: Option<String>,
    #[serde(alias = "fileOptions")]
    file_options: HashMap<String, FileOptions>,
    vars: HashMap<String, String>,
}

impl Template {
    /// Creates new template with given name
    pub fn create(
        config: &Config,
        args: &Args,
        name: &str,
    ) -> Result<(), Error> {
        let dir = config.template_dir.join(name);
        if dir.exists() && !args.yes {
            println!("Template '{name}' already exists.");
            if !args.yes && !yes_no("Do you want to replace it?") {
                return Ok(());
            }
            remove_dir_all(&dir)?;
        }

        let dst = dir.join("template");
        create_dir_all(&dst)?;

        Template::copy_files_raw(&args.dst, &dst)?;

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
            if !args.yes && !yes_no("Do you want to continue anyway?") {
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

        let src = tmplt.get_template_dir();
        tmplt.copy_files(&src, &args.dst)?;

        tmplt.post_exec(&args.dst)
    }

    /// Removes template
    pub fn remove(config: &Config, name: &str) -> Result<(), Error> {
        let dir = config.template_dir.join(name);
        if !dir.exists() {
            return Err(TemplateErr::NotFound(name.to_string()).into());
        }

        Ok(remove_dir_all(&dir)?)
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

    /// Executes pre script
    fn pre_exec(&self, dst: &PathBuf) -> Result<(), Error> {
        let Some(pre) = &self.pre else {
            return Ok(());
        };
        self.exec_script(pre, dst)
            .map_err(|_| TemplateErr::PreExec.into())
    }

    /// Executes post script
    fn post_exec(&self, dst: &PathBuf) -> Result<(), Error> {
        let Some(post) = &self.post else {
            return Ok(());
        };
        self.exec_script(post, dst)
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
    fn exec_script(
        &self,
        script: &String,
        dst: &PathBuf,
    ) -> Result<(), String> {
        let mut pcmd = String::new();
        Parser::string(&mut script.chars().map(Ok), &self.vars, &mut pcmd)
            .map_err(|e| e.to_string())?;

        let args = split(&pcmd).map_err(|e| e.to_string())?;
        if args.is_empty() {
            return Ok(());
        }

        let cmd = &args[0];
        let args = &args[1..];

        let command = Command::new(cmd)
            .args(args)
            .current_dir(dst)
            .envs(&self.vars)
            .output()
            .map_err(|e| e.to_string())?;
        if !command.status.success() {
            return Err(String::from_utf8_lossy(&command.stderr).to_string());
        }
        Ok(())
    }

    /// Makes file - follows options stored in template config
    fn make_file(&self, src: &PathBuf, dst: &PathBuf) -> Result<(), Error> {
        let rel_path = src
            .strip_prefix(self.get_template_dir())
            .map(|p| p.to_path_buf())
            .map_err(|e| e.to_string())?;
        let path = rel_path.to_string_lossy().to_string();
        let item = match self.file_options.get(&path) {
            Some(i) => i,
            None => return Template::copy_file(src, dst),
        };

        let mut dst = dst.to_owned();
        if let Some(name) = &item.name {
            let mut filename = String::new();
            let mut iter = name.chars().map(Ok);
            Parser::string(&mut iter, &self.vars, &mut filename)?;
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
