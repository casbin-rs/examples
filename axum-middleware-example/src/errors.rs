// DEFINE ERROR HERE
use crate::model::response::ResponseBody;
use serde_json::json;
use axum::Json;
use axum::response::{IntoResponse, Response};
use axum::http::{StatusCode};
use axum::body::{self, BoxBody};
use http_body::Full;

#[derive(Debug)]
pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                message,
                data: String::new(), // try box body, in this see if you can implement HttpResponse like service, post on axum
            },
        }
    }
    
    // pub fn response(&self) -> http::Result<Response<&ResponseBody<String>>> {
    //     Response::builder().status(self.http_status).body(&self.body)
    // }

    
}

// impl IntoResponse for ServiceError {
//     fn into_response(self) -> Response {
        
//     }
// }
