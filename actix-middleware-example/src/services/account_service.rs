use crate::{
    config::db::Pool,
    constants,
    errors::ServiceError,
    models::user::{DeleteUser, LoginForm, NewUser, User},
    models::user_token::UserToken,
    utils::token_utils,
};
use actix_web::{
    http::{header::HeaderValue, StatusCode},
    web,
};

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn signup(user: NewUser, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    match User::signup(user, &pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            message,
        )),
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

pub fn logout(
    authen_header: &HeaderValue,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    if let Ok(authen_str) = authen_header.to_str() {
        if authen_str.starts_with("bearer") {
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                if let Ok(username) = token_utils::verify_token(&token_data, pool) {
                    if let Ok(user) =
                        User::find_user_by_username(&username, &pool.get().unwrap())
                    {
                        User::logout(user.id, &pool.get().unwrap());
                        return Ok(());
                    }
                }
            }
        }
    }

    Err(ServiceError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
    ))
}

pub fn delete_admin(
    authen_header: &HeaderValue,
    delete_user_id: i32,
    delete_user: DeleteUser,
    pool: &web::Data<Pool>,
    ext:
) -> Result<(), ServiceError> {
    fn make_error() -> ServiceError {
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
        )
    }

    let authen_str = authen_header.to_str().map_err(|_| make_error())?;
    if authen_str.starts_with("bearer") != true {
        return Err(make_error());
    }
    let token = authen_str[6..authen_str.len()].trim();
    let token_data =
        token_utils::decode_token(token.to_string()).map_err(|_| make_error())?;

    let username =
        token_utils::verify_token(&token_data, pool).map_err(|_| make_error())?;
    let user = User::find_user_by_username(&username, &pool.get().unwrap())
        .map_err(|_| make_error())?;
    let delete_user_role = User::get_user_role(delete_user_id, &pool.get().unwrap())
        .map_err(|_| make_error())?;

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
    authen_header: &HeaderValue,
    delete_user: DeleteUser,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    fn make_error() -> ServiceError {
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
        )
    }

    let authen_str = authen_header.to_str().map_err(|_| make_error())?;

    if authen_str.starts_with("bearer") == false {
        return Err(make_error());
    }

    let token = authen_str[6..authen_str.len()].trim();

    let token_data =
        token_utils::decode_token(token.to_string()).map_err(|_| make_error())?;

    let username =
        token_utils::verify_token(&token_data, pool).map_err(|_| make_error())?;
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
