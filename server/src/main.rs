mod models;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mongodb::Client;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/*#[get("/api/v1/teams")]
async fn getTeams(client: web::Data<Client>) -> impl Responder {

}*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server Start");
    let client = Client::with_uri_str("mongodb://0.0.0.0:27017")
        .await
        .expect("Failed to connect client");
    //let _db = client.database("sushictfx-db");
    HttpServer::new(move || 
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(hello)
    )
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
