use mongodb::Client;
use crate::models::teams::Team;

pub async fn migrate_database(client: &Client) {
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