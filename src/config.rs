use std::fs;
use std::error::Error;

use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub ttl: Option<f32>,
    pub ttl_max: Option<f32>,
    pub ttl_min: Option<f32>,
    pub burn_max: Option<u32>,
    pub burn_min: Option<u32>,
    pub token_len: Option<u16>,
    pub max_content_length: Option<u32>,
    pub db_dsn: Option<String>,
    pub url_len: Option<u8>,
    pub url_alph: Option<String>,
    pub msg_empty_paste: Option<String>,
    pub msg_invalid_ttl: Option<String>,
    pub msg_invalid_burn: Option<String>,
    pub msg_err_404: Option<String>,
    pub msg_err_401: Option<String>,
    pub msg_paste_deleted: Option<String>,
}

impl Default for Config {
    // TODO: Figure out how to get this to work
    fn default() -> Config {
        Config {
            ttl: Some(1.0),
            ttl_max: Some(731.0),
            ttl_min: Some(0.0),
            burn_max: Some(1000),
            burn_min: Some(0),
            token_len: Some(6),
            max_content_length: Some(2097152),
            db_dsn: Some(String::from("postgres://pastebin:@localhost/pastebin")),
            url_len: Some(8),
            url_alph: Some(String::from("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghkmnpqrstuvwxyz")),
            msg_empty_paste: Some(String::from("empty paste received?\n")),
            msg_invalid_ttl: Some(String::from("ttl must be between {} and {} hours.\n")),
            msg_invalid_burn: Some(String::from("burn count must be between {} and {}.\n")),
            msg_err_404: Some(String::from("404 - Not Found.\n")),
            msg_err_401: Some(String::from("401 - Unauthorized - check your delete token.\n")),
            msg_paste_deleted: Some(String::from("OK - paste deleted successfully.\n")),
        }
    }
}


pub fn read_config() -> Result<Config, Box<dyn Error>> {
    let s: String = fs::read_to_string("config.yml")?;
    Ok(serde_yaml::from_str(&s)?)
}

