#[macro_use]
extern crate diesel;

mod models;
mod schema;

use crate::models::*;
use crate::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let mut connection = establish_connection();
    let results = users::table
        .limit(5)
        .load::<User>(&mut connection)
        .expect("Error loading posts");
    println!("Displaying {} users", results.len());
    for user in results {
        println!("{} {} {} {}", user.id, user.name, user.password, user.is_admin);
    }
}
