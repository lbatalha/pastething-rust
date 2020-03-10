use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Paste {
    pub pasteid: String,
    pub token: String,
    pub lexer: String,
    pub expiration: NaiveDateTime,
    pub burn: u16,
    pub paste: String,
    pub paste_lexed: Option<String>,
    pub size: Option<u32>,
    pub lines: Option<u32>,
    pub sloc: Option<u32>,
}

pub struct Stats {
    metric: String,
    counter: u32,
}

struct Dailystats {
    date: NaiveDateTime,
    pastecount: u16,
    pasteviews : u16,
}
