use crate::models::user::DeleteUser;
use crate::{
    config::db::Pool,
    constants,
    models::{
        response::ResponseBody,
        user::{LoginForm, NewUser},
    },
    services::account_service,
};
use actix_web::dev::ServiceRequest;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};
use chrono::{NaiveDateTime, Utc};

//#[post("/auth/login")]
pub async fn login(
    login_form: web::Json<LoginForm>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match account_service::login(login_form.0, &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_LOGIN_SUCCESS,
            token_res,
        ))),
        Err(err) => Ok(err.response()),
    }
}
//#[post("/auth/signup")]
pub async fn signup(
    signup_form: web::Json<NewUser>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let mut user = signup_form.into_inner();
    user.created_at = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);

    match account_service::signup(user, &pool) {
        Ok(message) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}

//#[post("/user/logout")]
pub async fn logout(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match account_service::logout(authen_header, &pool) {
            Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_LOGOUT_SUCCESS,
                constants::EMPTY,
            ))),
            Err(_) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
                constants::MESSAGE_LOGOUT_FAILED,
                constants::EMPTY,
            ))),
        }
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(
            constants::MESSAGE_TOKEN_MISSING,
            constants::EMPTY,
        )))
    }
}

//#[delete("/users/{id}")]
pub async fn delete_user(
    req: HttpRequest,
    user_id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        let delete_user = DeleteUser {
            is_deleted: true,
            deleted_at: Some(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
        };
        match account_service::delete_admin(
            authen_header,
            user_id.into_inner().parse::<i32>().unwrap(),
            delete_user,
            &pool,
        ) {
            Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_DELETE_USER_SUCCESS,
                constants::EMPTY,
            ))),
            Err(err) => Ok(err.response()),
        }
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(
            constants::MESSAGE_TOKEN_MISSING,
            constants::EMPTY,
        )))
    }
}

//#[delete("/user")]
pub async fn delete_self(
    req: HttpRequest,
    pool: web::Data<Pool>,
    ext: ServiceRequest,
) -> Result<HttpResponse> {
    let extension = ext.extensions().get().unwrap();
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        let delete_user_dto = DeleteUser {
            is_deleted: true,
            deleted_at: Some(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
        };
        match account_service::delete_self(authen_header, delete_user_dto, &pool) {
            Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_DELETE_USER_SUCCESS,
                constants::EMPTY,
            ))),
            Err(err) => Ok(err.response()),
        }
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(
            constants::MESSAGE_TOKEN_MISSING,
            constants::EMPTY,
        )))
    }
}
