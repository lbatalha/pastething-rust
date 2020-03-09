use actix_web::{web, App, HttpRequest, Error, HttpResponse, HttpServer, Result, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "about.html", print = "none")]
struct AboutTemplate {
    direction: &'static str,
    year: &'static str,
}

pub async fn about_page(_: HttpRequest) -> impl Responder {
    let s = AboutTemplate {
        direction: "ltr",
        year: "2020",
    }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

#[derive(Template)]
#[template(path = "api.html", print = "none")]
struct AboutApiTemplate<'a> {
    direction: &'a str,
    year: &'a str,
}


pub async fn about_api_page(_: HttpRequest) -> impl Responder {
    let s = AboutApiTemplate {
        direction: "ltr",
        year: "2020",
    }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}
