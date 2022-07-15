#![allow(clippy::if_same_then_else)]
use crate::{
    config::db::Pool,
    constants,
    errors::ServiceError,
    models::user::{DeleteUser, LoginForm, NewUser, User},
    models::user_token::UserToken,
};
use actix::Addr;
use actix_casbin::{CasbinActor, CasbinCmd, CasbinResult};
use actix_casbin_auth::casbin::{CachedEnforcer};
use actix_casbin_auth::CasbinVals;
use actix_web::{http::StatusCode, web, HttpRequest, HttpMessage};

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub async fn signup(
    user: NewUser,
    pool: &web::Data<Pool>,
    actor: web::Data<Addr<CasbinActor<CachedEnforcer>>>,
) -> Result<String, ServiceError> {
    let casbin_actor = actor.into_inner();
    let username = user.clone().username;
    let g_policies = vec![
        vec![username.clone(), "user_role_post_publish".to_string()],
        vec![username.clone(), "user_role_user".to_string()],
    ];
    let result_g = match casbin_actor
        .send(CasbinCmd::AddGroupingPolicies(g_policies))
        .await
    {
        Ok(Ok(CasbinResult::AddGroupingPolicies(result))) => result,
        _ => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_NEW_USER_ADD_PERMISSION_ERROR.to_string(),
            ))
        }
    };
    if result_g {
        match User::signup(user, &pool.get().unwrap()) {
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

pub fn login(
    login: LoginForm,
    pool: &web::Data<Pool>,
) -> Result<TokenBodyResponse, ServiceError> {
    match User::login(login, &pool.get().unwrap()) {
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
            constants::MESSAGE_LOGIN_FAILED.to_string(),
        )),
    }
}

pub fn logout(req: HttpRequest, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    fn make_error() -> ServiceError {
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FIND_USER.to_string(),
        )
    }
    let option_vals = req.extensions().get::<CasbinVals>().map(|x| x.to_owned());
    let vals = match option_vals {
        Some(value) => value,
        None => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_TOKEN_MISSING.to_string(),
            ))
        }
    };
    let username = &vals.subject;
    let user = User::find_user_by_username(&username, &pool.get().unwrap())
        .map_err(|_| make_error())?;
    User::logout(user.id, &pool.get().unwrap());
    Ok(())
}

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<User>, ServiceError> {
    match User::find_all(&pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_id(user_id: i32, pool: &web::Data<Pool>) -> Result<User, ServiceError> {
    match User::find_by_id(user_id, &pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn delete_admin(
    req: HttpRequest,
    delete_user_id: i32,
    delete_user: DeleteUser,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    fn make_error() -> ServiceError {
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FIND_USER.to_string(),
        )
    }
    let option_vals = req.extensions().get::<CasbinVals>().map(|x| x.to_owned());
    let vals = match option_vals {
        Some(value) => value,
        None => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_TOKEN_MISSING.to_string(),
            ))
        }
    };
    let username = &vals.subject;
    let user = User::find_user_by_username(&username, &pool.get().unwrap())
        .map_err(|_| make_error())?;
    let delete_user_role = user.role;

    if user.role == 0 && delete_user_id != user.id {
        match User::delete(delete_user_id, delete_user, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_DELETE_USER_ERROR.to_string(),
            )),
        }
    } else if user.role == 1
        && delete_user_role != 0
        && delete_user_role != 1
        && delete_user_id != user.id
    {
        match User::delete(delete_user_id, delete_user, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_DELETE_USER_ERROR.to_string(),
            )),
        }
    } else {
        Err(ServiceError::new(
            StatusCode::FORBIDDEN,
            constants::MESSAGE_DELETE_USER_PERMISSION_ERROR.to_string(),
        ))
    }
}

pub fn delete_self(
    req: HttpRequest,
    delete_user: DeleteUser,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    fn make_error() -> ServiceError {
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FIND_USER.to_string(),
        )
    }
    let option_vals = req.extensions().get::<CasbinVals>().map(|x| x.to_owned());
    let vals = match option_vals {
        Some(value) => value,
        None => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_TOKEN_MISSING.to_string(),
            ))
        }
    };
    let username = &vals.subject;
    let user = User::find_user_by_username(&username, &pool.get().unwrap())
        .map_err(|_| make_error())?;

    if user.role == 0 {
        Err(ServiceError::new(
            StatusCode::FORBIDDEN,
            constants::MESSAGE_DELETE_USER_PERMISSION_ERROR.to_string(),
        ))
    } else if user.role == 1 || user.role == 2 {
        match User::delete(user.id, delete_user, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_DELETE_USER_ERROR.to_string(),
            )),
        }
    } else {
        unreachable!()
    }
}
