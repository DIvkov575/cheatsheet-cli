use anyhow::Result;
use std::fs::File;
use prettytable::{row, Table};
use crate::config::{Config, get_config_path};
use crate::index;

pub fn get(id: &str) -> Result<()> {
    let config_path = get_config_path()?;
    let config_file_read = File::options().read(true).open(&config_path)?;
    let config = serde_yaml::from_reader(&config_file_read)?;

    let index = index(&id, &config)?;
    let value = config.data[index].line.clone();

    cli_clipboard::set_contents(value).unwrap();

    Ok(())
}
