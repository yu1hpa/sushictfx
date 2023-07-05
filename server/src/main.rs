mod models;
mod migrate;
mod router;
use actix_web::{web, App,  HttpServer};
use mongodb::Client;
use router::default::hello;
use router::teams::get_teams;

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