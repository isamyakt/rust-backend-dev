use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub genre: String,
}