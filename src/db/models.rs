use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: i32,
    pub name: String,
    pub status: String,
}
