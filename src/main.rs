#[macro_use]
extern crate diesel;
use self::models::*;
use self::diesel::prelude::*;

use chrono::NaiveDateTime;
use listenfd::ListenFd;

use actix_web::{web, App, HttpRequest, Error, HttpResponse, HttpServer, Result, Responder};
use actix_files;

mod templates;
mod config;
mod db;
mod schema;
mod models;

pub struct AppState {
    pub cfg: config::Config,
    pub db_pool: db::PgPool,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let cfg: config::Config = match config::read_config() {
        Ok(cfg) => cfg
        ,
        Err(e) => panic!(format!("Error reading config file: {}", e.to_string())),
    };
    //let cfg = cfg::config::Default::default();
    println!("Config = {:#?}", &cfg);
    //use schema::pastes::dsl::*;
    let pool = db::init_db_pool(&cfg.db_dsn.clone().unwrap());
    // let results = pastes.load::<Paste>(&conn).expect("failed");
    // println!("Displaying {} pastes", results.len());
    // for l in results {
    //     println!("{}", l.pasteid);
    //     println!("----------\n");
    //     println!("{}", l.paste);
    // }    
    
    
    // At some point get rid of all this crap, auto-builds only needed during early development
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(AppState {
                cfg: cfg.clone(),
                db_pool: pool.clone(),
            })
            .route("/", web::get().to(templates::new_paste_page))
            .route("/about", web::get().to(templates::about_page))
            .route("/about/api", web::get().to(templates::about_api_page))
            .service(actix_files::Files::new("/static", "./static"))
            .route("/{id}", web::get().to(templates::view_paste_page))
            .route("/newpaste", web::post().to(templates::new_paste))

    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
