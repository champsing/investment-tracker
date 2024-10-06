use actix_files::Files;
use actix_web::web;
use actix_web::{App, HttpServer};
use server::auth;
use server::constant;
use std::fs;

fn setup() {
    let _ = fs::create_dir_all(constant::path::CACHE);
    let _ = fs::create_dir_all(constant::path::DATA);
    let _ = fs::create_dir_all(constant::path::WEB);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup();

    HttpServer::new(move || {
        App::new()
            .service(auth::login)
            .service(auth::refresh)
            .service(Files::new("/", constant::path::WEB).index_file("index.html"))
            .default_service(web::to(server::index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
