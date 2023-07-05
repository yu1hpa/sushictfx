mod models;
mod migrate;
use actix_web::{
    get, 
    web, App, HttpResponse, HttpServer, Responder
};
use mongodb::{Client, bson::doc};
use models::teams::{Team, Teams};
//use migrate::teams::migrate_database;
use futures::stream::TryStreamExt;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/api/v1/teams")]
async fn get_teams(client: web::Data<Client>) -> HttpResponse {
    let db = client.database("sushictfx-db");
    let collection = db.collection::<Team>("teams");
    let mut cursor = collection.find(doc! {}, None)
        .await
        .expect("Error getting teams detail");
    let mut teams = Teams{teams: vec![]};
    while let Some(_team) = cursor.try_next().await.unwrap() {
        teams.teams.push(_team);
    }
    HttpResponse::Ok().json(teams)
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use super::*;

    #[actix_web::test]
    async fn test_get_hello() {
        let mut app = test::init_service(
            App::new()
                .service(hello)
        ).await;
        let resp = test::TestRequest::get().uri("/").send_request(&mut app).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_teams() {
        let client = Client::with_uri_str("mongodb://root:example@db:27017")
            .await
            .expect("Failed to connect client");
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(client.clone()))
                .service(hello)
                .service(get_teams)
        ).await;
        let resp = test::TestRequest::get().uri("/api/v1/teams").send_request(&mut app).await;
        assert!(resp.status().is_success());
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server Start");
    let client = Client::with_uri_str("mongodb://root:example@db:27017")
        .await
        .expect("Failed to connect client");
    //migrate_database(&client).await;
    HttpServer::new(move || 
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(get_teams)
    )
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}