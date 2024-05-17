use std::fs::{File, read_to_string};
use anyhow::Result;
use dirs::home_dir;
use crate::{get_config_path, Config, CsError, gen_id, get_ids, Record, show};

pub fn add(name: String, line: String) -> Result<()> {
    let config_path = get_config_path()?;

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





// "sudo /usr/local/mysql/support-files/mysql.server start"