use std::fs::File;
use std::io::{BufRead, read_to_string, stdin, stdout, Write};
use std::path::PathBuf;
use anyhow::Result;
use clap::Parser;
use dirs::home_dir;
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};
use crate::config::Config;

mod error;
mod add;
mod show;
mod remove;
mod web_sync;
mod cli;
mod config;

use error::ClicError;

fn main() -> Result<()> {
    cli::Args::parse().run()?;

    Ok(())
}

fn get_config_path() -> Result<PathBuf> {
    let home_dir = home_dir().ok_or(ClicError::MissingHomeDir)?;
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

    Err(ClicError::TooManyIDRetries.into())
}
pub fn get_input(message: &str) -> Result<String> {
    let mut token = String::new();
    print!("{}", message);
    stdout().lock().flush()?;
    stdin().lock().read_line(&mut token)?;
    Ok(token)
}
