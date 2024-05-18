use std::fs::{File, read_to_string};
use serde_json::{json, Map, Value};
use std::io::{BufRead, stdin, stdout, Write};
use std::process::exit;
use anyhow::Result;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Response;
use serde::Serialize;
use crate::{get_config_path, error, config, get_ids, gen_id, get_input};
use crate::config::Config;
use crate::error::ClicError;


#[tokio::main]
pub async fn init_web_sync() -> Result<()> {
    let config_path = get_config_path()?;
    let mut config: Config = serde_yaml::from_str(&read_to_string(&config_path)?)?;
    let mut config_file = File::options().write(true).open(&config_path)?;

    if config.pat.is_empty() { config.pat = get_input("enter pat: ")?; }

    let _body_ast: Map<String, Value> = wrap_body("tmp")?;
    let body = serde_json::to_string(&_body_ast)?;
    let res= request(&config.pat, "https://api.github.com/gists", body).await?;


    if res.status().is_success() {
        config.gist_id = id_from_res(res).await?;
        print!("Success: created gist w/ id {}\n", config.gist_id)
    } else {
        println!("Error: {:?}\n", res.text().await?);
    };

    serde_yaml::to_writer(config_file, &config)?;
    push().await?;
    Ok(())
}

pub async fn push() -> Result<()> {
    let config_path = get_config_path()?;
    let mut config: Config = serde_yaml::from_str(&read_to_string(&config_path)?)?;

    if config.pat.is_empty() { config.pat = get_input("enter pat: ")?; }
    if config.gist_id.is_empty() { return Err(ClicError::NoGistId.into()) }

    let url = format!("https://api.github.com/gists/{}", config.gist_id);

    let config_file_contents = serde_yaml::to_string(&config)?;
    let _body_ast = wrap_body(&config_file_contents)?;
    let body = serde_json::to_string(&_body_ast)?;
    let res= request(&config.pat, &url, body).await?;


    if res.status().is_success() {
        println!("Success: {:?}\n", res.text().await?);
    } else {
        println!("Error: {:?}\n", res.text().await?);
    };

    Ok(())
}


async fn id_from_res(res: Response) -> Result<String> {
    // split response string -> get id -> clean id string
    Ok(res.text().await?
        .split(|a| a==',')
        .filter(|x| x.contains("id"))
        .take(1)
        .map(|x| x.split(|x| x==':').collect::<Vec<&str>>()[1])
        .map(|x| x.to_string())
        .collect::<Vec<String>>()[0].replace("\"", ""))
}


fn wrap_body(body: &str) -> Result<Map<String, Value>>{
    let body_ast = Map::from_iter([
        ("description".to_string(), "online backup for clic - cheatsheet cli config".to_string().into()),
        ("public".to_string(), "false".to_string().into()),
        ("files".to_string(), Value::Object(Map::from_iter([
            ("cheatsheet.yaml".to_string(), Value::Object(Map::from_iter([
                ("content".to_string(), Value::String(body.to_string()))
            ])))
        ])))
    ]);

    Ok(body_ast)

}



async fn request(pat: &str, url: &str, body: String) -> Result<Response> {
    let client = reqwest::Client::new();
    let res = client.post(url)
        .header("Authorization", format!("Bearer {}", pat))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header(USER_AGENT, "ClicApp/1.0")
        .body(body)
        .send()
        .await?;

    Ok(res)
}

