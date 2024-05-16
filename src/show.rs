use serde::{Deserialize, Serialize};
use std::fs::{File, read_to_string};
use std::io::Write;
use anyhow::Result;
use dirs::home_dir;
use crate::{check_for_config_existence, Config, CsError, Record};
// #[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, row};

pub fn show() -> Result<()> {

    let home_dir = home_dir().ok_or(CsError::MissingHomeDir)?; // attempt to get home env-var
    let config_path = home_dir.join(".config").join("cheatsheet-cli.yaml"); // config path
    check_for_config_existence(&config_path)?;

    let content: Config = serde_yaml::from_str(&read_to_string(&config_path)?)?;

    if content.data.len() == 0 {
        println!("Empty ☹️");
    } else {
        let mut table = Table::new();
        for record in content.data { table.add_row(row![record.id, record.name, record.line]); }
        table.printstd();
    }


    Ok(())
}

pub fn show_command() -> Result<()> {
    println!("Clic - Cli Cheatsheet");
    show()?;
    Ok(())
}