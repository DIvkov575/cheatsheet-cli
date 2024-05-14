use serde::{Deserialize, Serialize};
use std::fs::{File, read_to_string};
use std::io::Write;
use anyhow::Result;
use dirs::home_dir;
use crate::CS_Error;

pub fn show() -> Result<()> {

    let home_dir = home_dir().ok_or(CS_Error::MissingHomeDir)?; // attempt to get home env-var
    let config = home_dir.join(".config").join("cheatsheet-cli.yaml"); // config path

    if !config.exists() { File::create(&config)?; }

    let content = serde_yaml::from_str(&read_to_string(&config)?)?;

    // let rec = Record {
    //     line: "sudo /usr/local/mysql/support-files/mysql.server start".into(),
    //     name: "start mysql".into(),
    //     date: "asd".into(),
    // };
    //
    // let conf = Config {
    //     data: vec![rec]
    // };

    // let a  =serde_yaml::to_string(&conf)?;
    // let mut file = File::options().append(true).open(&config)?;
    // file.write_all(a.as_bytes())?;


    Ok(())
}
//


#[derive(Serialize, Deserialize)]
struct Config {
    data: Vec<Record>
}


#[derive(Serialize, Deserialize)]
struct Record {
    line: String,
    name: String,
    date: String,
}