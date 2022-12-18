#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use lib::db;
use lib::model::Movie;

#[get("/")]
fn get_movies() -> Json<Option<Vec<Movie>>> {
    Json(db::read_movies())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/movies",
        routes![get_movies],
    )
}


fn main() {
    rocket().launch();
}
