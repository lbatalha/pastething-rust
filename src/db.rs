use diesel::prelude::*;
use diesel::pg::PgConnection;


pub fn establish_connection(dsn: &String) -> PgConnection {
    PgConnection::establish(&dsn)
        .expect(&format!("Error connecting to {}", dsn))
}