use std::fs::{File, read_to_string};
use anyhow::Result;
use dirs::home_dir;
use crate::{check_for_config_existence, Config, CsError, gen_id, get_ids, Record, show};

pub fn remove(id: String) -> Result<()> {
    let home_dir = home_dir().ok_or(CsError::MissingHomeDir)?; // attempt to get home env-var
    let config_path = home_dir.join(".config").join("cheatsheet-cli.yaml"); // config path
    check_for_config_existence(&config_path)?;

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
