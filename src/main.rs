// Cargo.toml
// [dependencies]
// actix-web = "4"
// serde = { version = "1.0", features = ["derive"] }
// serde-xml-rs = "0.6"

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;
// Dosya okuma
async fn get_sales() -> impl Responder {
    match fs::read_to_string("pivot.xml") {
        Ok(xml_data) => HttpResponse::Ok()
            .content_type("application/xml")
            .body(xml_data),
        Err(_) => HttpResponse::InternalServerError()
            .body("<error>Failed to read XML file</error>"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/sales", web::get().to(get_sales))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}