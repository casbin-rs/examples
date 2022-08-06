use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
}

// impl<T> IntoResponse for ResponseBody<T> {
//     fn into_response(self) -> Response {
//         Response::new(ReponseBody<T>)
//     }
// }
