use std::io::{BufRead, stdin, stdout, Write};
use anyhow::Result;
use clap::Parser;
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};
use error::ClicError;

use crate::config::Config;
mod error;
mod config;
mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    cli::Args::parse().run().await?;
    Ok(())
}

pub fn index_keys(key: &str, config: &Config) -> Result<usize> {
    let mut keys: Vec<String> = Vec::with_capacity(config.data.len());
    for record in &config.data { keys.push(record.key.clone()) }
    let index = keys.iter()
        .position(|x| &x.to_uppercase() == key)
        .ok_or(ClicError::NonExistentKey(key.to_uppercase().clone()))?;
    Ok(index)
}

pub fn index_ids(id: &str, config: &Config) -> Result<usize> {
    let ids = get_ids(&config)?;
    let index = ids.iter().position(|x| x == &id).ok_or(ClicError::IdNotFound)?;
    Ok(index)
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
