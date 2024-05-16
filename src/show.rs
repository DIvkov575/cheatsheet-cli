use serde::{Deserialize, Serialize};
use std::fs::{File, read_to_string};
use std::io::Write;
use anyhow::Result;
use dirs::home_dir;
use crate::{Config, CS_Error, Record};
// #[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, row};

pub fn show() -> Result<()> {

    let home_dir = home_dir().ok_or(CS_Error::MissingHomeDir)?; // attempt to get home env-var
    let config = home_dir.join(".config").join("cheatsheet-cli.yaml"); // config path
    if !config.exists() { File::create(&config)?; }

    let content: Config = serde_yaml::from_str(&read_to_string(&config)?)?;

    println!("Clic - Cli Cheatsheet");
    if content.data.len() == 0 {
        println!("Empty ☹️");
    } else {
        let mut table = Table::new();
        for record in content.data { table.add_row(row![record.name, record.line]); }
        table.printstd();
    }


    Ok(())
}
