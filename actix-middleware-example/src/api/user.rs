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
use actix::Addr;
use actix_casbin::CasbinActor;
use actix_casbin_auth::casbin::CachedEnforcer;
use actix_web::{web, HttpRequest, HttpResponse, Result};
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
    actor: web::Data<Addr<CasbinActor<CachedEnforcer>>>,
) -> Result<HttpResponse> {
    let mut user = signup_form.into_inner();
    user.created_at = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);

    match account_service::signup(user, &pool, actor).await {
        Ok(message) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}

//#[post("/user/logout")]
pub async fn logout(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account_service::logout(req, &pool) {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_LOGOUT_SUCCESS,
            constants::EMPTY,
        ))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
            constants::MESSAGE_LOGOUT_FAILED,
            constants::EMPTY,
        ))),
    }
}

//#[delete("/admin/user/{id}")]
pub async fn delete_user(
    req: HttpRequest,
    user_id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let delete_user = DeleteUser {
        is_deleted: true,
        deleted_at: Some(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
    };
    match account_service::delete_admin(
        req,
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
}

//#[delete("/user")]
pub async fn delete_self(
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let delete_user_dto = DeleteUser {
        is_deleted: true,
        deleted_at: Some(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
    };
    match account_service::delete_self(req, delete_user_dto, &pool) {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_DELETE_USER_SUCCESS,
            constants::EMPTY,
        ))),
        Err(err) => Ok(err.response()),
    }
}

//#[get("/admin/user/{id}")]
pub async fn find_by_id(
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match account_service::find_by_id(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(user) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, user)))
        }
        Err(err) => Ok(err.response()),
    }
}

//#[get("/admin/users")]
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account_service::find_all(&pool) {
        Ok(users) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, users)))
        }
        Err(err) => Ok(err.response()),
    }
}
