use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use mongodb::Client;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str("mongodb://0.0.0.0:27017")
        .await
        .expect("Failed to connect client");
    let _db = client.database("sushictfx-db");
    HttpServer::new(move || App::new())
        .bind("0.0.0.0::8888")?
        .run()
        .await
}
