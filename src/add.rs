use std::fs::{File, read_to_string};
use std::path::PathBuf;
use anyhow::Result;
use dirs::home_dir;
use rand::{Rng};
use rand::distributions::Alphanumeric;
use crate::{check_for_config_existence, Config, CsError, gen_id, get_ids, Record, show};
use crate::CsError::TooManyIDRetries;

pub fn add(name: String, line: String) -> Result<()> {
    let home_dir = home_dir().ok_or(CsError::MissingHomeDir)?; // attempt to get home env-var
    let config_path = home_dir.join(".config").join("cheatsheet-cli.yaml"); // config path

    check_for_config_existence(&config_path)?;

    let mut config: Config = serde_yaml::from_str(&read_to_string(&config_path)?)?;
    let ids = get_ids(&config)?;
    let id = gen_id(&ids)?;

    let record = Record { id, line, name };
    config.data.push(record.clone());

    let file = File::options().write(true).open(&config_path)?;
    serde_yaml::to_writer(file, &config)?;

    println!("Clic - Cli Cheatsheet");
    println!("Success adding record");
    show::show()?;

    Ok(())
}





// sudo /usr/local/mysql/support-files/mysql.server