use std::fs::File;
use anyhow::Result;
use prettytable::{row, Table};

use crate::config::{Config, get_config_path};

pub async fn show() -> Result<()> {
    let mut config: Config;
    let config_path = get_config_path()?;
    {
        let config_file_read = File::options().read(true).open(&config_path)?;
        config = serde_yaml::from_reader(&config_file_read)?;
    }


    if config.data.len() == 0 {
        println!("Empty ☹️");
    } else {
        let mut table = Table::new();
        for record in config.data { table.add_row(row![record.id, record.name, record.line]); }
        table.printstd();
    }


    Ok(())
}

pub async fn show_command() -> Result<()> {
    println!("Clic - Cli Cheatsheet");
    show().await?;
    Ok(())
}
