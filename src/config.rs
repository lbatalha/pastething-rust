use std::fs;
use std::error::Error;
use std::collections::HashMap; 

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub defaults: Option<HashMap<String, String>>,
    pub paste_limits: Option<HashMap<String, u16>>,
    pub token_len: Option<u32>,
    pub max_content_length: Option<u32>,
    pub db_dsn: Option<String>,
    pub url_len: Option<u8>,
    pub url_alph: Option<String>,
    pub return_messages: Option<HashMap<String, String>>,
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
    let s: String = fs::read_to_string("config.yml")?;
    Ok(serde_yaml::from_str(&s)?)
}

