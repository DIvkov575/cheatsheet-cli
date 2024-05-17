use std::fs::{File, read_to_string};
use anyhow::Result;
use dirs::home_dir;
use crate::{get_config_path, Config, CsError, gen_id, get_ids, Record, show};

pub fn remove(id: String) -> Result<()> {
    let config_path = get_config_path()?;

    let mut config: Config = serde_yaml::from_str(&read_to_string(&config_path)?)?;
    let ids = get_ids(&config)?;
    let id = id.to_uppercase();

    let index = ids.iter().position(|x| x == &id).ok_or(CsError::NonExistentId(id.clone()))?;
    config.data.remove(index);

    let file = File::options().write(true).truncate(true).open(&config_path)?;
    serde_yaml::to_writer(file, &config)?;

    println!("Clic - Cli Cheatsheet");
    println!("Success removing record");
    show::show()?;

    Ok(())
}
