

// #[get("/")]
// fn get_cars() -> Json<Option<Vec<ReceivedData>>> {
//   Json(database::read_cars(connection))
// }

// // #[get("/<title>")]
// // fn get_movie(title: &RawStr) -> Json<Option<Movie>> {
// //     Json(db::read_movie(title.url_decode().expect("Failed to decode title.")))
// // }

// // #[post("/", data="<movie>")]
// // fn create_movie(movie: Json<Movie>) -> Json<Option<Movie>> {
// //     Json(db::insert_movie(movie.0))
// // }

// // #[delete("/<title>")]
// // fn delete_movie(title: &RawStr) -> Json<bool> {
// //     Json(db::delete_movie(title.url_decode().expect("Failed to decode title.")))
// // }

// fn rocket() -> rocket::Rocket {
//   rocket::ignite().mount(
//     "/movies",
//     routes![get_cars],
//     // get_movies, get_movie, create_movie, delete_movie
//   )
// }
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket_contrib::json::Json;
use models::ARAuto;


use carreg::*;

fn main() {
    rocket().launch();
}

#[get("/")]
fn get_cars() -> Json<Option<Vec<ARAuto>>> {
    Json(read_cars())
}

#[delete("/<id>")]
fn delete_car(id: i32) -> Json<bool> {
    Json(remove_car(id))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/api/cars",
        routes![get_cars, delete_car],
    )
}