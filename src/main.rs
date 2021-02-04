use actix_files::Files;
use actix_web::{App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host: String = String::from("localhost");
    let port: u16 = 8080;
    
    println!("Running on {}:{}.", host, port);

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static").show_files_listing())
            .service(routes::auth::login)
            .service(routes::files::import)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
