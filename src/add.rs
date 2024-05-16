use std::fs::{File, read_to_string};
use anyhow::Result;
use dirs::home_dir;
use rand::{Rng};
use rand::distributions::Alphanumeric;
use crate::{Config, CS_Error, Record};
use crate::CS_Error::TooManyIDRetries;

pub fn add(name: String, line: String) -> Result<()> {
    let home_dir = home_dir().ok_or(CS_Error::MissingHomeDir)?; // attempt to get home env-var
    let config_path = home_dir.join(".config").join("cheatsheet-cli.yaml"); // config path

    if !config_path.exists() {
        let config = File::options().create(true).write(true).open(&config_path)?;
        serde_yaml::to_writer(config, &Config::empty())?;
    }

    let mut config: Config = serde_yaml::from_str(&read_to_string(&config_path)?)?;
    let ids = get_ids(&config)?;
    let id = gen_id(&ids)?;

    let record = Record { id, line, name };
    config.data.push(record);

    let file = File::options().write(true).open(&config_path)?;
    serde_yaml::to_writer(file, &config)?;


    Ok(())
}

fn get_ids(config: &Config) -> Result<Vec<String>> {
    let mut ids: Vec<String> = Vec::with_capacity(config.data.len());
    for record in &config.data { ids.push(record.id.clone()) }
    Ok(ids)
}

fn gen_id(ids: &[String]) -> Result<String> {
    let mut id: String;
    for _ in 0..30 {
        id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect();

        if ids.contains(&id) { continue }
        else { return Ok(id); }
    }

    Err(TooManyIDRetries.into())
}