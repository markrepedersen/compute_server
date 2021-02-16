use std::io;

use actix_files::Files;
use actix_web::{App, HttpServer};

mod routes;
mod validate;

const STATIC_FILES_ENDPOINT: &'static str = "/static";
const STATIC_FILES_DIR: &'static str = "./static";
const DEFAULT_INDEX_FILE: &'static str = "/static/home/index.html";

#[actix_web::main]
async fn main() -> io::Result<()> {
    let host = String::from("localhost");
    let port: u16 = 8080;

    println!("Running on {}:{}.", host, port);

    HttpServer::new(|| {
        let static_content_service =
            Files::new(STATIC_FILES_ENDPOINT, STATIC_FILES_DIR).index_file(DEFAULT_INDEX_FILE);

        App::new()
            .service(static_content_service)
            .service(routes::auth::login)
            .service(routes::files::import)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
