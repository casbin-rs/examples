use crate::{
    config::db::Pool,
    constants,
    models::{
        post::{DeletePost, NewPost},
        response::ResponseBody,
    },
    services::post_service,
};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use chrono::{NaiveDateTime, Utc};

//#[get("/posts")]
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match post_service::find_all(&pool) {
        Ok(people) => Ok(
            HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))
        ),
        Err(err) => Ok(err.response()),
    }
}

//#[get("/posts/{id}")]
pub async fn find_by_id(
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match post_service::find_by_id(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(person) => Ok(
            HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, person))
        ),
        Err(err) => Ok(err.response()),
    }
}

//#[post("/post")]
pub async fn insert(
    new_post: web::Json<NewPost>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let mut post = new_post.into_inner();
    post.created_at = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
    match post_service::insert(post, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

//#[delete("/post/{id}")]
pub async fn delete(
    id: web::Path<String>,
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let delete_post = DeletePost {
        is_deleted: true,
        deleted_at: Some(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
    };
    match post_service::delete(
        id.into_inner().parse::<i32>().unwrap(),
        req,
        delete_post,
        &pool,
    ) {
        Ok(()) => Ok(HttpResponse::Ok()
            .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

//#[get("/post/{id}")]
pub async fn find_by_id_admin(
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match post_service::find_by_id(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(person) => Ok(
            HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, person))
        ),
        Err(err) => Ok(err.response()),
    }
}
