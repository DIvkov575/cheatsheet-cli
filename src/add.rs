use std::fs::{File, read_to_string};
use anyhow::Result;
use dirs::home_dir;
use crate::CS_Error;

pub fn add(line: String) -> Result<()> {
    let home_dir = home_dir().ok_or(CS_Error::MissingHomeDir)?; // attempt to get home env-var
    let config = home_dir.join(".config").join("cheatsheet-cli.yaml"); // config path

    if !config.exists() { File::create(&config)?; }

    let val = serde_yaml::from_str(&read_to_string(&config)?)?;
    print!("{:?}", val);



    Ok(())
}