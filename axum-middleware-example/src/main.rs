#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

mod model;
mod schema;
mod utils;
mod errors;
mod service;
mod middleware;
mod repository;
mod route;
mod constants;
mod api;

use crate::model::*;
use crate::schema::*;
use crate::user::User;
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
        println!("{} {} {} {} {}", user.id, user.username, user.email, user.password, user.role);
    }
}
