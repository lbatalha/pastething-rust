//use std::collections::HashMap;
use listenfd::ListenFd;
use actix_web::{web, App, HttpRequest, Error, HttpResponse, HttpServer, Result, Responder};
use actix_files;

mod templates;
mod config;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config: config::Config = match config::read_config(){
        Ok(config) => config,
        Err(e) => panic!(format!("Error reading config file: {}", e.to_string())),
    };
    println!("deserialized = {:#?}", config);
    println!("{:?}", config.token_len);
    // At some point get rid of all this crap, auto-builds only needed during early development
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .route("/about", web::get().to(templates::about_page))
            .route("/about/api", web::get().to(templates::about_api_page))
            .service(actix_files::Files::new("/static", "./static"))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
