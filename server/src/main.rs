mod models;
use actix_web::{
    get, 
    web, App, HttpResponse, HttpServer, Responder
};
use mongodb::Client;
use models::teams::Team;



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/*#[get("/api/v1/teams")]
async fn getTeams(client: web::Data<Client>){

}*/

/*#[post("/api/v1/teams")]
async fun postTeams(client: web::Data<Client>) { 

}*/

async fn migrate_database(client: &Client) {
    let db = client.database("sushictfx-db");
    let collection = db.collection::<Team>("teams");
    let teams = vec![
        Team{
            id: None,
            role: "NotAdmin".to_string(),
            name: "Yajirushi314".to_string(),
            password: "pass12345".to_string(),
            email: "yajirushi@test.com".to_string(),
            points: 100,
            solved_ids: vec![
                1, 2, 3, 4, 5
            ]
        },
        Team{
            id: None,
            role: "Admin".to_string(),
            name: "sush1st".to_string(),
            password: "pass67890".to_string(),
            email: "sush1st@test.com".to_string(),
            points: 300,
            solved_ids: vec![
                6, 7, 8, 9, 10
            ]
        },
    ];
    collection
        .insert_many(teams, None)
        .await
        .expect("insert migrate document");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server Start");
    let client = Client::with_uri_str("mongodb+srv://root:example@0.0.0.0:27017")
        .await
        .expect("Failed to connect client");
    //let _db = client.database("sushictfx-db");
    migrate_database(&client).await;
    HttpServer::new(move || 
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(hello)
    )
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}