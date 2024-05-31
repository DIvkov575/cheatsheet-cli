use std::fs::{File, read_to_string};
use anyhow::Result;
use crate::config::{Config, get_config_path};
use crate::{get_ids, index_ids, index_keys};
use crate::cli::show;
use crate::cli::web_sync::push;
use crate::error::ClicError;

pub async fn update_id(input: &str, new_id: &str) -> Result<()> {
    // load
    let config_path = get_config_path()?;
    let mut config: Config = serde_yaml::from_str(&read_to_string(&config_path)?)?;
    let config_file = File::options().write(true).open(&config_path)?;

    // validate input
    if new_id.len() > 3 { return Err(ClicError::InvalidInput.into()) }

    // update record
    let try_key_index = index_keys(&input.replace("-", "_").to_uppercase(), &config);
    let try_id_index = index_ids(&input.to_uppercase(), &config);

    if let Ok(index) = try_id_index {
        config.data[index].id = new_id.to_uppercase();
    } else {
        let index = try_key_index?;
        config.data[index].id = new_id.to_uppercase();
    }

    // write
    config_file.set_len(0)?;
    serde_yaml::to_writer(config_file, &config)?;

    // display
    println!("Clic - Cli Cheatsheet");
    println!("Success adding record");
    show::show().await?;
    push().await?;

    Ok(())
}

pub async fn update_key(input: &str, new_key: &str) -> Result<()> {

    // load
    let config_path = get_config_path()?;
    let mut config: Config = serde_yaml::from_str(&read_to_string(&config_path)?)?;
    let config_file = File::options().write(true).open(&config_path)?;


    // update record
    let try_key_index = index_keys(&input.replace("-", "_").to_uppercase(), &config);
    let try_id_index = index_ids(&input.to_uppercase(), &config);

    if let Ok(index) = try_id_index {
        config.data[index].key= new_key.to_uppercase();
    } else {
        let index = try_key_index?;
        config.data[index].key= new_key.to_uppercase();
    }

    // write
    config_file.set_len(0)?;
    serde_yaml::to_writer(config_file, &config)?;

    // display
    println!("Clic - Cli Cheatsheet");
    println!("Success adding record");
    show::show().await?;
    push().await?;

    Ok(())
}
pub async fn update_val(input: &str, new_value: &str) -> Result<()> {
    // load
    let config_path = get_config_path()?;
    let mut config: Config = serde_yaml::from_str(&read_to_string(&config_path)?)?;
    let config_file = File::options().write(true).open(&config_path)?;


    // update record
    let try_key_index = index_keys(&input.replace("-", "_").to_uppercase(), &config);
    let try_id_index = index_ids(&input.to_uppercase(), &config);

    if let Ok(index) = try_id_index {
        config.data[index].value = new_value.to_string();
    } else {
        let index = try_key_index?;
        config.data[index].value = new_value.to_string();
    }

    // write
    config_file.set_len(0)?;
    serde_yaml::to_writer(config_file, &config)?;

    // display
    println!("Clic - Cli Cheatsheet");
    println!("Success adding record");
    show::show().await?;
    push().await?;

    Ok(())
  }
