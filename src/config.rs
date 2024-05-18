use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub gist_id: String,
    pub pat: String,
    pub data: Vec<Record>
}

impl Config {
    pub fn empty() -> Self {
        Self {
            gist_id: String::new(),
            pat: String::new(),
            data: vec![],
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub id: String,
    pub line: String,
    pub name: String,
}
