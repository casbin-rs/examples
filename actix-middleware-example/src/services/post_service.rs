use crate::{
    config::db::Pool,
    constants,
    errors::ServiceError,
    models::post::{DeletePost, NewPost, Post},
    models::user::User,
    utils::token_utils,
};
use actix_web::{
    http::{header::HeaderValue, StatusCode},
    web,
};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Post>, ServiceError> {
    match Post::find_all(&pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Post, ServiceError> {
    match Post::find_by_id(id, &pool.get().unwrap()) {
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
    authen_header: &HeaderValue,
    delete_post: DeletePost,
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
                        if user.role == 0 || user.role == 1 {
                            match Post::find_by_id(id, &pool.get().unwrap()) {
                                Ok(_) => match Post::delete(
                                    id,
                                    delete_post,
                                    &pool.get().unwrap(),
                                ) {
                                    Ok(_) => return Ok(()),
                                    Err(_) => {
                                        return Err(ServiceError::new(
                                            StatusCode::INTERNAL_SERVER_ERROR,
                                            constants::MESSAGE_CAN_NOT_DELETE_DATA
                                                .to_string(),
                                        ))
                                    }
                                },
                                Err(_) => {
                                    return Err(ServiceError::new(
                                        StatusCode::NOT_FOUND,
                                        format!("Post with id {} not found", id),
                                    ))
                                }
                            };
                        }

                        return Err(ServiceError::new(
                            StatusCode::FORBIDDEN,
                            constants::MESSAGE_DELETE_POST_PERMISSION_ERROR.to_string(),
                        ));
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
