use serde::{Deserialize, Serialize};
use chrono::{Datelike, Utc};

use actix_web::{web, App, HttpRequest, Error, HttpResponse, HttpServer, Result, Responder};
use actix_multipart::Multipart;
use askama::Template;

use super::config;
use super::AppState;

use super::schema::pastes::dsl::*;
use super::diesel::prelude::*;
use super::models::*;



#[derive(Template)]
#[template(path = "newpaste.html", print = "none")]
struct NewPaste<'a> {
    direction: &'a str,
    year: String,
    cfg: &'a config::Config,
}
pub async fn new_paste_page<'a>(cfg: web::Data<AppState>) -> impl Responder {
    let s = NewPaste {
        direction: "ltr",
        year: Utc::now().year().to_string(),
        cfg: &cfg.cfg,
    }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

#[derive(Serialize, Deserialize)]
pub struct PasteParams {
    pub paste: String,
    pub ttl: f32,
    pub lexer: String,
    pub burn: i32,
    pub raw: bool,
}

pub async fn new_paste(mut payload: Multipart) -> impl Responder {
    unimplemented!();
    // this is broken in the examples? https://github.com/actix/examples/blob/master/multipart/src/main.rs#L9
    // while let Some(item) = payload.next().await {
    //     let mut field = item?;

    // }

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Params: "))

}

#[derive(Template)]
#[template(path = "viewpaste.html", print = "none")]
struct ViewPaste<'a> {
    direction: &'a str,
    year: String,
    cfg: &'a config::Config,
}
pub async fn view_paste_page<'a>(cfg: web::Data<AppState>, id: web::Path<(String)>) -> impl Responder {
    let results = pastes.filter(pasteid.eq(id.to_string()))
                        .limit(1)
                        .load::<Paste>(&cfg.db_pool.get().unwrap())
                        .expect("failed");
    let s = ViewPaste {
        direction: "ltr",
        year: Utc::now().year().to_string(),
        cfg: &cfg.cfg,
    }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}


#[derive(Template)]
#[template(path = "about.html", print = "none")]
struct AboutTemplate {
    direction: &'static str,
    year: String,
}
pub async fn about_page(_: HttpRequest) -> impl Responder {
    let s = AboutTemplate {
        direction: "ltr",
        year: Utc::now().year().to_string(),
    }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}


#[derive(Template)]
#[template(path = "api.html", print = "none")]
struct AboutApiTemplate<'a> {
    direction: &'a str,
    year: String,
}
pub async fn about_api_page(_: HttpRequest) -> impl Responder {
    let s = AboutApiTemplate {
        direction: "ltr",
        year: Utc::now().year().to_string(),
    }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}
