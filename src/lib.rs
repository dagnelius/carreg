pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use serde::{Serialize, Deserialize};

use self::models::*;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn read_cars() -> Option<Vec<ARAuto>> {
    use self::schema::ar_auto::dsl::*;

    let connection = &mut establish_connection();
    let results = ar_auto
        .limit(5)
        .load::<ARAuto>(connection)
        .expect("Error loading posts");

    let serialized_car = serde_json::to_string(&results).unwrap();
    let cars: Result<Vec<ARAuto>, serde_json::Error> = serde_json::from_str(&serialized_car);
    match cars {
        Ok(movies) => Some(movies),
        Err(_) => None
    }
}

pub fn remove_car(id: i32) -> bool {
    use self::schema::ar_auto::dsl::*;

    let connection = &mut establish_connection();
    diesel::delete(ar_auto.filter(id.eq(id))).execute(connection);
    true
}