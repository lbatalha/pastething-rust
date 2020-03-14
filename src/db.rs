use std::thread;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };


use super::models::{Paste};





pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub fn init_db_pool(dsn: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(dsn);
    Pool::builder().max_size(32).build(manager).expect("Failed to create pool")
}

// pub fn newpaste(conn: &PgConnection, params) -> Paste {
//     use super::schema::pastes;

//     let new_paste = Paste {
//         pasteid: params.pasteid,
//         token: params.token,
//         lexer: Option<String>,
//         expiration: NaiveDateTime,
//         burn: i32,
//         paste: String,
//         paste_lexed: Option<String>,
//         size: Option<i32>,
//         lines: Option<i32>,
//         sloc: Option<i32>,
//     };

//     diesel::insert_into(pastes::table)
//         .values(&new_paste)
//         .get_result(conn)
//         .expect("Failed to insert paste")
// }