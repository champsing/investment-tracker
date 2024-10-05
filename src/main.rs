use actix_files::Files;
use actix_web::{App, HttpServer};
use server::auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(auth::login::handler)
            .service(auth::check::handler)
            .service(Files::new("/", "./dist").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
