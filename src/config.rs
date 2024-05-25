use std::fs::File;
use std::io::read_to_string;
use std::path::PathBuf;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use crate::error::ClicError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub gist_id: String,
    pub pat: String,
    pub data: Vec<Record>
}

impl Config {
    pub fn empty() -> Self {
        Self {
            gist_id: String::new(),
            pat: String::new(),
            data: vec![],
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub id: String,
    pub line: String,
    pub name: String,
}


pub fn create_config(config_path: &PathBuf) -> anyhow::Result<()> {
    let config = File::options().create(true).write(true).open(&config_path)?;
    config.set_len(0)?;
    serde_yaml::to_writer(config, &Config::empty())?;
    Ok(())
}

pub fn get_config_path() -> anyhow::Result<PathBuf> {

    let home_dir = home_dir().ok_or(ClicError::MissingHomeDir)?;
    let config_path = home_dir.join(".config").join("cheatsheet-cli.yaml");
    let mut config_file = File::options().read(true).write(true).open(&config_path)?;

    if !config_path.exists() || read_to_string(&config_file)?.is_empty() { create_config(&config_path)?; }

    Ok(config_path)
}
