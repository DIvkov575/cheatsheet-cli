use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::ptr::write;

use anyhow::Result;
use prettytable::{row, Table};

use crate::config::{Config, get_config_path};
use crate::get_input;
use crate::web_sync::{pull, push};

pub async fn show() -> Result<()> {
    let mut config: Config;
    let config_path = get_config_path()?;
    {
        let config_file_read = File::options().read(true).open(&config_path)?;
        config = serde_yaml::from_reader(&config_file_read)?;
    }

    let local_config = serde_yaml::to_string(&config)?;

    if let Ok(mut online_config) = pull().await {
        online_config = online_config.replace("\n  ", "\n").to_string();
        online_config = online_config[2..online_config.len()-1].to_string();

        if online_config != local_config {
            let input = get_input("local config doesnt match cloud config. Choose an option: 'pull','push','None': ")?;
            match input.to_lowercase().replace("\n", "").as_str() {
                "pull" => {
                    {
                        let mut config_file = File::options().truncate(true).write(true).open(&config_path)?;
                        config_file.write_all(online_config.as_bytes())?;
                    }
                    {
                        let config_file_read = File::options().read(true).open(&config_path)?;
                        config = serde_yaml::from_reader(&config_file_read)?;
                    }
                },
                "push" => { push().await? },
                _ => {},
            }
        }
    };

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