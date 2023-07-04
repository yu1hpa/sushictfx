use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Team {
    id: String,
    role: String,
    name: String,
    password: String,
    email: String,
    points: uint32,
    solvedIds: Vec<uint32>,
}
