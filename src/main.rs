//use std::collections::HashMap;
use listenfd::ListenFd;
use actix_web::{web, App, HttpRequest, Error, HttpResponse, HttpServer, Result, Responder};
use actix_files;
use askama::Template;


#[derive(Template)]
#[template(path = "about.html", print = "none")]
struct AboutTemplate {
    direction: &'static str,
    year: &'static str,
}

async fn about_page(_: HttpRequest) -> impl Responder {
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


async fn about_api_page(_: HttpRequest) -> impl Responder {
    let s = AboutApiTemplate {
        direction: "ltr",
        year: "2020",
    }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // At some point get rid of all this crap, auto-builds only needed during early development
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .route("/about", web::get().to(about_page))
            .route("/about/api", web::get().to(about_api_page))
            .service(actix_files::Files::new("/static", "./static"))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
