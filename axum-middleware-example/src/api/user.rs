// use crate::model::user::DeleteUser;
use crate::{
    constants,
    model::db::Pool,
    model::{
        response::ResponseBody,
        user::{AddUser, LoginForm},
    },
    service::user::{self},
};

use async_std::sync::Arc;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_casbin_auth::casbin::CachedEnforcer;
use axum_macros::debug_handler;
use tokio::sync::RwLock;

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
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
        )
            .into_response(),
    }
}

// POST(api/auth/register)
#[debug_handler]
pub async fn register(
    Json(reg_form): Json<AddUser>,
    pool: Extension<Pool>,
    Extension(enforcer): Extension<Arc<RwLock<CachedEnforcer>>>,
) -> Response {
    let user = reg_form;
    match user::register(user, &pool, enforcer).await {
        Ok(message) => {
            Json(ResponseBody::new(&message, constants::EMPTY)).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_NEW_USER_ADD_PERMISSION_ERROR.to_string(),
        )
            .into_response(),
    }
}

// DELETE(api/admin/{:id})
// IT DOESNT DO ANY USER VALIDATION
#[debug_handler]
pub async fn delete_user(Path(id): Path<String>, pool: Extension<Pool>) -> Response {
    match user::delete_user(id.parse::<i32>().unwrap(), &pool) {
        Ok(delete) => {
            Json(ResponseBody::new(constants::MESSAGE_OK, delete)).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )
            .into_response(),
    }
}

// GET(api/user/{:id})
pub async fn get_user(Path(id): Path<String>, pool: Extension<Pool>) -> Response {
    match user::get_user(id.parse::<i32>().unwrap(), &pool) {
        Ok(user) => Json(ResponseBody::new(constants::MESSAGE_OK, user)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )
            .into_response(),
    }
}

// GET(api/users)
pub async fn get_all_user(pool: Extension<Pool>) -> Response {
    match user::get_all_user(&pool) {
        Ok(users) => {
            Json(ResponseBody::new(constants::MESSAGE_OK, users)).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )
            .into_response(),
    }
}

// PUT(api/admin/{:id})
pub async fn update_user(
    Path(id): Path<String>,
    Json(update_form): Json<AddUser>,
    pool: Extension<Pool>,
) -> Response {
    match user::update_user(id.parse::<i32>().unwrap(), update_form, &pool) {
        Ok(message) => {
            Json(ResponseBody::new(&message, constants::EMPTY)).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_UPDATE_USER_ERROR.to_string(),
        )
            .into_response(),
    }
}
