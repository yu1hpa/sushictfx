use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub role: String,
    pub name: String,
    pub password: String,
    pub email: String,
    pub points: u32,
    pub solved_ids: Vec<u32>,
}
