// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
// extern crate rocket;

// use rocket_contrib::json::Json;
// use rocket::http::RawStr;
// use lib::database;
// use lib::models::ReceivedData;

// fn main() {
//   let connection = &mut lib::database::establish_connection();
//   rocket().launch();
// }

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

use self::models::*;
use diesel::prelude::*;
use carreg::*;

fn main() {
    use self::schema::ar_auto::dsl::*;

    let connection = &mut establish_connection();
    let results = ar_auto
        .limit(5)
        .load::<ARAuto>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{:?}", post);
        println!("-----------");
    }
}