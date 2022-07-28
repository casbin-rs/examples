use crate::models::*;
use crate::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use poem::web::{Data, Json};
use poem::{handler, web::Path, IntoResponse};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

#[handler]
pub fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[handler]
pub fn get_users(mut conn: Data<&Arc<Mutex<PgConnection>>>) -> impl IntoResponse {
    let results = users::table
        .limit(5)
        .load::<User>(conn.0.lock().unwrap().deref_mut())
        .expect("Error loading posts");
    Json(results)
}
