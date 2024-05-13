use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let cs_config = Path::new("$HOME").join(".config").join("cheetsheet-cli.yaml");

    if cs_config.exists() {
        serde_yaml::Deserializer::from_str(&read_to_string(cs_config)?);
    }

    Ok(())
}

// sudo /usr/local/mysql/support-files/mysql.server start (or restart or stop)



