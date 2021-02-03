use actix_files::{Files, NamedFile};
use actix_web::{get, App, HttpServer, Result};
use std::path::Path;

mod routes;

#[get("/")]
async fn index() -> Result<NamedFile> {
    let path = Path::new("static").join("login").join("login.html");

    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host: String = String::from("localhost");
    let port: u16 = 8080;
    println!("Running on {}:{}.", host, port);

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", ".").show_files_listing())
            .service(index)
            .service(routes::import::import)
            .service(routes::export::export)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
