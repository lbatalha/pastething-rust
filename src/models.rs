use chrono::prelude::{NaiveDateTime, Utc};
use super::schema::pastes;
use super::config;




#[derive(Debug, Queryable, Insertable)]
#[table_name="pastes"]
pub struct Paste {
    pub pasteid: String,
    pub token: String,
    pub lexer: Option<String>,
    pub expiration: NaiveDateTime,
    pub burn: i32,
    pub paste: String,
    pub paste_lexed: Option<String>,
    pub size: Option<i32>,
    pub lines: Option<i32>,
    pub sloc: Option<i32>,
}

// impl Default for Paste {
//     fn default() -> Paste {
//         let config: config::Config = match config::read_config(){
//             Ok(config) => config::Config::default(),
//             Err(e) => panic!(format!("Error reading config file: {}", e.to_string())),
//         };
//         Paste {
//             pasteid: String::from(""),
//             token: String::from(""),
//             lexer: config.defaults.get(&"lexer"),
//             expiration: Utc::now().naive_utc() + Duration::hours(config.defaults.get(&"ttl")),
//             burn: -1,
//             paste: String::from(""),
//             paste_lexed: None,
//             size: None,
//             lines: None,
//             sloc: None,
//         }
//     }
// }


pub struct Stats {
    metric: String,
    counter: u32,
}

struct Dailystats {
    date: NaiveDateTime,
    pastecount: u16,
    pasteviews : u16,
}
