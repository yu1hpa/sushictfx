use actix_web::{get, web, HttpResponse};
use mongodb::{Client, bson::doc};
use crate::models::teams::{Team, Teams};
use futures::stream::TryStreamExt;

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