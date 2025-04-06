// Cargo.toml
// [dependencies]
// actix-web = "4"
// serde = { version = "1.0", features = ["derive"] }
// serde-xml-rs = "0.6"
// log = "0.4"
// env_logger = "0.10"

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use log::{error, info};

async fn get_sales() -> impl Responder {
    match fs::read_to_string("pivot.xml") {
        Ok(xml_data) => {
            info!("XML dosyası başarıyla okundu.");
            HttpResponse::Ok()
                .content_type("application/xml")
                .body(xml_data)
        }
        Err(e) => {
            error!("XML dosyası okunamadı: {}", e);
            HttpResponse::InternalServerError()
                .body("<error>Failed to read XML file</error>")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let server_address = "127.0.0.1:8080";
    info!("Server {} adresinde başlatılıyor...", server_address);

    HttpServer::new(|| {
        App::new()
            .route("/sales", web::get().to(get_sales))
    })
        .bind(server_address)?
        .run()
        .await
}