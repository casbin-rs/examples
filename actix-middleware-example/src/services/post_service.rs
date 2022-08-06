use crate::{
    config::db::Pool,
    constants,
    errors::ServiceError,
    models::post::{DeletePost, NewPost, Post},
    models::user::User,
};
use actix_casbin_auth::CasbinVals;
use actix_web::{http::StatusCode, web, HttpMessage, HttpRequest};

pub fn find_all_public(pool: &web::Data<Pool>) -> Result<Vec<Post>, ServiceError> {
    match Post::find_all(false, &pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_all(
    req: HttpRequest,
    pool: &web::Data<Pool>,
) -> Result<Vec<Post>, ServiceError> {
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
    let user = User::find_user_by_username(username, &pool.get().unwrap())
        .map_err(|_| make_error())?;
    let mut is_admin = false;
    if user.role == 0 || user.role == 1 {
        is_admin = true;
    };
    match Post::find_all(is_admin, &pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_id(
    req: HttpRequest,
    id: i32,
    pool: &web::Data<Pool>,
) -> Result<Post, ServiceError> {
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
    let user = User::find_user_by_username(username, &pool.get().unwrap())
        .map_err(|_| make_error())?;
    let mut is_admin = false;
    if user.role == 0 || user.role == 1 {
        is_admin = true;
    };
    match Post::find_by_id(is_admin, id, &pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Post with id {} not found", id),
        )),
    }
}

pub fn find_by_id_public(id: i32, pool: &web::Data<Pool>) -> Result<Post, ServiceError> {
    match Post::find_by_id(false, id, &pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Post with id {} not found", id),
        )),
    }
}

pub fn insert(new_post: NewPost, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Post::insert(new_post, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn delete(
    id: i32,
    req: HttpRequest,
    delete_post: DeletePost,
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
    let user = User::find_user_by_username(username, &pool.get().unwrap())
        .map_err(|_| make_error())?;
    if user.role == 0 || user.role == 1 {
        match Post::find_by_id(true, id, &pool.get().unwrap()) {
            Ok(_) => match Post::delete(id, delete_post, &pool.get().unwrap()) {
                Ok(_) => return Ok(()),
                Err(_) => {
                    return Err(ServiceError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
                    ))
                }
            },
            Err(_) => {
                return Err(ServiceError::new(
                    StatusCode::NOT_FOUND,
                    format!("Post with id {} not found", id),
                ))
            }
        }
    }
    Err(ServiceError::new(
        StatusCode::FORBIDDEN,
        constants::MESSAGE_DELETE_POST_PERMISSION_ERROR.to_string(),
    ))
}
