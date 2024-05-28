use std::fs::{File, read_to_string};

use anyhow::Result;
use crate::{index};
use crate::config::{Config, get_config_path};
use crate::cli::{web_sync::push, show};

pub async fn remove(ids: &[String]) -> Result<()> {
    // load
    let config_path = get_config_path()?;
    let config_file = File::options().write(true).open(&config_path)?;
    let mut config: Config = serde_yaml::from_str(&read_to_string(&config_path)?)?;

    println!("Clic - Cli Cheatsheet");
    for id in ids {
        let index = index(&id, &config)?;
        config.data.remove(index);
        println!("Success removing record {id}");
    }

    config_file.set_len(0)?;
    serde_yaml::to_writer(&config_file, &config)?;
    show::show().await?;
    push().await?;

    Ok(())
}
