#![allow(clippy::if_same_then_else)]

use crate::{
    constants,
    errors::ServiceError,
    model::user::{AddUser, User},
    model::user_token::UserToken,
    model::{db::Pool, user::LoginForm},
};

use axum::extract::Extension;
use axum::http::StatusCode;
use axum_casbin_auth::casbin::{CachedEnforcer, MgmtApi};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

// ADD USER
pub async fn register(
    user: AddUser,
    Extension(pool): &Extension<Pool>,
    enforcer: Arc<RwLock<CachedEnforcer>>,
) -> Result<String, ServiceError> {
    let username = user.clone().username;
    let email = user.clone().email;
    let clone_role = user.role.clone();
    let g_policies = vec![username.clone(), clone_role.to_string()];

    // CHECK IF USER IS ALREADY PRESENT
    if User::get_user_by_email(&email, &pool.get().unwrap()).is_err() {
        match enforcer
            .write()
            .await
            .add_named_grouping_policy("g", g_policies)
            .await
        {
            Ok(_) => info!("Preset policies(p) add successfully"),
            Err(err) => error!("Preset policies(g) add error: {}", err.to_string()),
        };

        match User::register(user, &pool.get().unwrap()) {
            Ok(message) => Ok(message),
            Err(message) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                message,
            )),
        }
    } else {
        Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_NEW_USER_ADD_PERMISSION_ERROR.to_string(),
        ))
    }
}

pub fn get_all_user(
    Extension(pool): &Extension<Pool>,
) -> Result<Vec<User>, ServiceError> {
    match User::get_all_user(&pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn get_user(
    user_id: i32,
    Extension(pool): &Extension<Pool>,
) -> Result<User, ServiceError> {
    match User::get_user(user_id, &pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn signin(
    login: LoginForm,
    Extension(pool): &Extension<Pool>,
) -> Result<TokenBodyResponse, ServiceError> {
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

pub fn update_user(
    update_id: i32,
    user_data: AddUser,
    Extension(pool): &Extension<Pool>,
) -> Result<String, ServiceError> {
    match User::update_user(update_id, user_data, &pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            message,
        )),
    }
}

pub fn delete_user(
    delete_id: i32,
    Extension(pool): &Extension<Pool>,
) -> Result<usize, ServiceError> {
    match User::delete_user(delete_id, &pool.get().unwrap()) {
        Ok(delete) => Ok(delete),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FIND_USER.to_string(),
        )),
    }
}
