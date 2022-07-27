// DEFINE ROUTES HERE
use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::api::user;

pub fn routes() -> Router {
    let router = Router::new()
        .route("/api/auth/register", post(user::register))
        .route("/api/auth/signin", post(user::signin))
        .route("/api/users", get(user::get_all_user))
        .route("/api/user/:id", get(user::get_user))
        .route("/api/admin/:id", put(user::update_user))
        .route("/api/admin/:id", delete(user::delete_user));

    // NEED TO ADD MIDDLEWARE HERE, BEFORE CALLING IN MAIN.RS
    router
}
