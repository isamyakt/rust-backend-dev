#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket_contrib::json::Json;
use lib::db;
use lib::model::Movie;

#[get("/")]
fn get_movies() -> Json<Option<Vec<Movie>>> {
    Json(db::read_movies())
}

#[get("/<title>")]
fn get_movie(title: &RawStr) -> Json<Option<Movie>> {
    Json(db::read_movie(title.url_decode().expect("Failed to decode title.")))
}

#[post("/", data="<movie>")]
fn create_movie(movie: Json<Movie>) -> Json<Option<Movie>> {
    Json(db::insert_movie(movie.0))
}

#[delete("/<title>")]
fn delete_movies(title: &RawStr) -> Json<bool> {
    Json(db::delete_movie(title.url_decode().expect("Failed to decode title.")))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/movies",
        routes![get_movies, get_movie, create_movie, delete_movies],
    )
}


fn main() {
    rocket().launch();
}
