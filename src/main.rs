use actix_files::Files;
use actix_web::web;
use actix_web::{App, HttpServer};
// use server::{auth, constant, investment};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    if let Err(error) = flexfolio::init() {
        panic!("Fail to initialize the server with error: {}", error)
    }

    HttpServer::new(move || {
        App::new()
            // .service(auth::login)
            // .service(auth::refresh)
            // .service(auth::all_users)
            // .service(auth::upsert)
            // .service(auth::delete)
            // .service(investment::account::query)
            // .service(investment::account::upsert)
            // .service(investment::account::delete)
            .service(Files::new("/", "dist/").index_file("index.html"))
            .default_service(web::to(flexfolio::index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
