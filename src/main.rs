use actix_files::Files;
use actix_web::web;
use actix_web::{App, HttpServer};
use server::auth;
use server::constant;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(error) = server::init() {
        panic!("init failed with error {}", error)
    }

    HttpServer::new(move || {
        App::new()
            .service(auth::login)
            .service(auth::refresh)
            .service(auth::all_users)
            .service(auth::upsert)
            .service(auth::delete)
            .service(Files::new("/", constant::path::STATIC).index_file("index.html"))
            .default_service(web::to(server::index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
