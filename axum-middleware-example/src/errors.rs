// DEFINE ERROR HERE
use crate::model::response::ResponseBody;
use axum::http::StatusCode;

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
                data: String::new(),
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
