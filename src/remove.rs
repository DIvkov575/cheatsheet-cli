use std::fs::{File, read_to_string};

use anyhow::Result;
use crate::{get_ids, show};
use crate::config::{Config, get_config_path};
use crate::error::ClicError;
use crate::web_sync::push;

pub async fn remove(id: String) -> Result<()> {
    // load
    let config_path = get_config_path()?;
    let config_file = File::options().write(true).open(&config_path)?;
    let mut config: Config = serde_yaml::from_str(&read_to_string(&config_path)?)?;

    // remove
    let ids = get_ids(&config)?;
    let id = id.to_uppercase();
    let index = ids.iter().position(|x| x == &id).ok_or(ClicError::NonExistentId(id.clone()))?;
    config.data.remove(index);

    // write
    config_file.set_len(0)?;
    serde_yaml::to_writer(config_file, &config)?;

    // display
    println!("Clic - Cli Cheatsheet");
    println!("Success removing record");
    show::show().await?;

    push().await?;

    Ok(())
}
