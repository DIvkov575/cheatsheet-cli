use std::fs::File;
use anyhow::Result;
use std::process::{Command, Stdio};
use crate::config::get_config_path;
use crate::{index_ids, index_keys};

pub fn run(input: &str) -> Result<()> {
    let value: String;
    let config_path = get_config_path()?;
    let config_file_read = File::options().read(true).open(&config_path)?;
    let config = serde_yaml::from_reader(&config_file_read)?;

    let try_key_index = index_keys(&input.replace("-", "_").to_uppercase(), &config);
    let try_id_index = index_ids(&input.to_uppercase(), &config);

    if let Ok(index) = try_id_index {
        value = config.data[index].value.clone();
    } else {
        let index = try_key_index?;
        value = config.data[index].value.clone();
    }

    let mut cmd: Vec<&str> = value.split(|x| x==' ').collect();
    let args= cmd.split_off(1);

    let child = Command::new(cmd[0]).args(args)
        .stdout(Stdio::inherit())
        .spawn()?;

    Ok(())
}
