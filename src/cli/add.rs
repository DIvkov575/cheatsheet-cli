use std::fs::{File, read_to_string};
use anyhow::Result;
use crate::config::{Config, get_config_path, Record};
use crate::{gen_id, get_ids};
use crate::cli::{web_sync::push, show};

pub async fn add(name: String, line: String) -> Result<()> {
    // load
    let config_path = get_config_path()?;
    let mut config: Config = serde_yaml::from_str(&read_to_string(&config_path)?)?;
    let config_file = File::options().write(true).open(&config_path)?;

    // add record
    let ids = get_ids(&config)?;
    let id = gen_id(&ids)?;
    let record = Record { id, value: line.trim_start().trim_end().to_string(), key: name.trim_start().trim_end().to_string() };
    config.data.push(record.clone());

    // write
    config_file.set_len(0)?;
    serde_yaml::to_writer(config_file, &config)?;

    // display
    println!("Clic - Cli Cheatsheet");
    println!("Success adding record");
    show::show().await?;

    // sync
    push().await?;

    Ok(())
}