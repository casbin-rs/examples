#[macro_use]
extern crate diesel;

use std::env;
use std::sync::{Arc, Mutex};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use poem::{get, listener::TcpListener, EndpointExt, Route, Server};

use poem_casbin_auth::casbin::function_map::key_match2;
use poem_casbin_auth::casbin::{CoreApi, DefaultModel, FileAdapter};
use poem_casbin_auth::CasbinService;

use crate::models::*;
use crate::schema::*;

mod apis;
mod auth;
mod models;
mod schema;

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "poem=debug");
    }
    let connection = establish_connection();

    let m = DefaultModel::from_file("rbac_with_pattern_model.conf")
        .await
        .unwrap();
    let a = FileAdapter::new("rbac_with_pattern_policy.csv");

    let casbin_middleware = CasbinService::new(m, a).await.unwrap();

    casbin_middleware
        .write()
        .await
        .get_role_manager()
        .write()
        .matching_fn(Some(key_match2), None);

    let app = Route::new()
        .at("/hello", get(apis::hello))
        .at("/users", get(apis::get_users))
        .at("/todos", get(apis::get_todos))
        .at(
            "/todo/:id",
            get(apis::get_todo)
                .put(apis::update_todo)
                .delete(apis::delete_todo),
        )
        .at(
            "/user/todos",
            get(apis::get_self_todos).post(apis::create_todo),
        )
        .at("/user/:name/todos", get(apis::get_user_todos))
        .with(casbin_middleware)
        .with(auth::BasicAuth)
        .data(Arc::new(Mutex::new(connection)));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("poem-todo")
        .run(app)
        .await
}
