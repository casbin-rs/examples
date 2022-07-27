// use crate::model::user::DeleteUser;
use crate::{
    constants,
    model::db::Pool,
    model::{
        response::ResponseBody,
        user::{LoginForm, NewUser},
    },
    service::user::{self, TokenBodyResponse},
};
// use actix::Addr;
// use actix_casbin::CasbinActor;
// use actix_casbin_auth::casbin::CachedEnforcer;
// use actix_web::{web, HttpRequest, HttpResponse, Result};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_casbin_auth::casbin::CachedEnforcer;
// use serde_json::json;
// use http::{StatusCode, Response};
use chrono::{NaiveDateTime, Utc};
use tower::balance::pool;

// IMPLEMENT INTORESPONSE FOR SERVICE ERROR AND UPDATE ALL ERRORS OR MATCH THE ERROR WITH SERVICE ERROR (STATUSCODE, CONTANTSMSG).INTO_RESPONSE()
// POST(api/auth/signin)
pub async fn signin(
    Json(login_form): Json<LoginForm>,
    pool: Extension<Pool>,
) -> Response {
    // output should be a json response
    match user::signin(login_form, &pool) {
        Ok(token_res) => Json(ResponseBody::new(
            constants::MESSAGE_SIGNIN_SUCCESS,
            token_res,
        ))
        .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()).into_response(),
    }
}

// POST(api/auth/register)
pub async fn register() {}

// DELETE(api/admin/{:id})
pub async fn delete_user() {}

// GET(api/user/{:id})
pub async fn get_user(Path(id): Path<String>, pool: Extension<Pool>) -> Response{
    match user::get_user(id.parse::<i32>().unwrap(), &pool){
        Ok(user) => {Json(ResponseBody::new(constants::MESSAGE_OK, user)).into_response()}
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string()).into_response()
    }
}

// GET(api/users)
pub async fn get_all_user(pool: Extension<Pool>) -> Response {
    match user::get_all_user(&pool) {
        Ok(users) => {
            Json(ResponseBody::new(constants::MESSAGE_OK, users)).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string()).into_response(),
    }
}

// PUT(api/admin/{:id})
pub async fn update_user() {}
