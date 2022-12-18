use std::fs;
use crate::model::Movie;

static MOVIES_DB: &str = "data/movies.json";

fn _movies() -> Result<Vec<Movie>, serde_json::Error> {
    let data = fs::read_to_string(MOVIES_DB).expect("Error reading from this file");
    let movies: Result<Vec<Movie>, serde_json::Error> = serde_json::from_str(&data);

    movies
}

pub fn read_movies() -> Option<Vec<Movie>> {
    match _movies() {
        Ok(movies) => Some(movies),
        Err(_) => None
    }
}