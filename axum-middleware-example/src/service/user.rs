
#![allow(clippy::if_same_then_else)]
use crate::{
    model::{db::Pool, user::LoginForm},
    constants,
    errors::ServiceError,
    model::user::{NewUser, User},
    model::user_token::UserToken,
};

use axum::extract::Extension;
use axum::http::StatusCode;
use axum_casbin_auth::casbin::{CachedEnforcer};
use axum_casbin_auth::CasbinVals;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

// ADD USER
pub async fn register() {}

pub fn get_all_user(Extension(pool): &Extension<Pool>) -> Result<Vec<User>, ServiceError>{
    match User::get_all_user(&pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),))
    }
}

pub fn get_user(user_id: i32, Extension(pool): &Extension<Pool>) -> Result<User, ServiceError> {
    match User::get_user(user_id, &pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),))
    }
}

pub fn signin(login: LoginForm, Extension(pool): &Extension<Pool>) -> Result<TokenBodyResponse, ServiceError> {
    match User::signin(login, &pool.get().unwrap()) {
        Some(logged_user) => {
            match serde_json::from_value(
                json!({ "token": UserToken::generate_token(logged_user), "token_type": "bearer" }),
            ) {
                Ok(token_res) => Ok(token_res),
                Err(_) => Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
                )),
            }
        }
        None => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_SIGNIN_FAILED.to_string(),
        )),
    }
}

pub fn update_user() {}

pub fn delete_user() {}