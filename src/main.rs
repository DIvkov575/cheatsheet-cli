use std::fs::File;
use std::io::read_to_string;
use std::path::PathBuf;
use anyhow::Result;
use clap::Parser;
use dirs::home_dir;
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};
use crate::Command::Show;
use crate::CsError::TooManyIDRetries;

mod add;
mod show;
mod remove;
mod web_sync;

fn main() -> Result<()> {
    Args::parse().run()?;

    Ok(())
}

fn get_config_path() -> Result<PathBuf> {
    let home_dir = home_dir().ok_or(CsError::MissingHomeDir)?;
    let config_path = home_dir.join(".config").join("cheatsheet-cli.yaml");

    if !config_path.exists() ||
        read_to_string(File::options().read(true).open(&config_path)?)?.is_empty()
    {
        let config = File::options().create(true).write(true).open(&config_path)?;
        serde_yaml::to_writer(config, &Config::empty())?;
    }

    Ok(config_path)
}


pub fn get_ids(config: &Config) -> Result<Vec<String>> {
    let mut ids: Vec<String> = Vec::with_capacity(config.data.len());
    for record in &config.data { ids.push(record.id.clone()) }
    Ok(ids)
}

pub fn gen_id(ids: &[String]) -> Result<String> {
    let mut id: String;
    for _ in 0..30 {
        id = "".to_string();
        id.push(thread_rng().gen_range(65..=90).into());
        id.push(thread_rng().gen_range(65..=90).into());

        if ids.contains(&id) { continue }
        else { return Ok(id); }
    }

    Err(TooManyIDRetries.into())
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    pat: String,
    data: Vec<Record>
}
impl Config {
    pub fn empty() -> Self {
        Self {
            pat: "".to_string(),
            data: vec![],
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Record {
    id: String,
    line: String,
    name: String,
}


#[derive(Parser, Debug)]
pub enum Command {
    #[command(about="Show CheatSheet")]
    Show,
    #[command(about="Create gist to cloud save config and cheatsheet")]
    InitWeb,
    #[command(about="add a line to cheatsheet")]
    Add {
        name: String,
        line: String
    },
    #[command(about="deletes a line from cheatsheet")]
    Remove {
        id: String
    }
}

impl Command {
    pub fn run(self) -> Result<()> {
        use Command::*;
        match self {
            Show => show::show_command(),
            InitWeb=> web_sync::init_web_sync(),
            Add{name, line} => add::add(name, line),
            Remove{id} => remove::remove(id),
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None,)]
pub struct Args {
    #[clap(subcommand)]
    command: Option<Command>,
}

impl Args {
    pub fn run(self) -> Result<()> {
        match self.command {
            None => {Command::Show.run()?},
            Some(command) => command.run()?,
        }
        Ok(())
    }
}


#[derive(thiserror::Error, Debug)]
pub enum CsError {
    #[error("Too many consecutive id creation retry attempts")]
    TooManyIDRetries,
    #[error("please ensure $HOME environment variable is set")]
    MissingHomeDir,
    #[error("record id does not exist in clic")]
    NonExistentId(String)
}