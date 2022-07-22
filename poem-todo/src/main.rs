#[macro_use]
extern crate diesel;

mod auth;
mod models;
mod schema;

use crate::models::*;
use crate::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use poem::web::{Data, Json};
use poem::{
    get, handler, listener::TcpListener, web::Path, EndpointExt, IntoResponse, Route,
    Server,
};
use std::env;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[handler]
fn get_users(mut conn: Data<&Arc<Mutex<PgConnection>>>) -> impl IntoResponse {
    let results = users::table
        .limit(5)
        .load::<User>(conn.0.lock().unwrap().deref_mut())
        .expect("Error loading posts");
    Json(results)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "poem=debug");
    }
    let connection = establish_connection();
    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/users", get(get_users))
        .with(auth::BasicAuth)
        .data(Arc::new(Mutex::new(connection)));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("poem-todo")
        .run(app)
        .await
}
