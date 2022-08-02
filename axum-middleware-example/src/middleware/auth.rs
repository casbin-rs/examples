use crate::{
    constants,
    model::{db::Pool, response::ResponseBody},
    utils::token_utils,
};

use axum::{
    body::{self, BoxBody},
    extract::Extension,
    http::{
        header::{HeaderName, HeaderValue},
        Method,
    },
    response::{IntoResponse, Response},
    BoxError, Json,
};
use axum_casbin_auth::CasbinVals;
use bytes::Bytes;
use futures::future::BoxFuture;
use http::{self, Request};
use http_body::Body as HttpBody;
use std::{
    boxed::Box,
    convert::Infallible,
    task::{Context, Poll},
};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct AuthLayer;

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for AuthMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    Infallible: From<<S as Service<Request<ReqBody>>>::Error>,
    ResBody: HttpBody<Data = Bytes> + Send + 'static,
    ResBody::Error: Into<BoxError>,
{
    type Response = Response<BoxBody>;
    type Error = Infallible;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);
        let mut authenticate_pass: bool = false;
        let mut authenticate_username: String = String::from("");

        // Bypass account routes
        let headers = req.headers_mut();
        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );

        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            for ignore_route in constants::IGNORE_ROUTES.iter() {
                if req.uri().path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }
            if !authenticate_pass {
                if let Some(pool) = req.extensions().get::<Extension<Pool>>() {
                    info!("Connecting to database...");
                    if let Some(auth_header) =
                        req.headers().get(constants::AUTHORIZATION)
                    {
                        info!("Parsing authorization header...");
                        if let Ok(auth_str) = auth_header.to_str() {
                            if auth_str.starts_with("bearer")
                                || auth_str.starts_with("Bearer")
                            {
                                info!("Parsing token...");
                                let token = auth_str[6..auth_str.len()].trim();
                                if let Ok(token_data) =
                                    token_utils::decode_token(token.to_string())
                                {
                                    info!("Decoding token...");
                                    if token_utils::validate_token(&token_data, pool)
                                        .is_ok()
                                    {
                                        info!("Valid token");
                                        authenticate_username =
                                            token_data.claims.user_name;
                                        authenticate_pass = true;
                                    } else {
                                        error!("Invalid token");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if authenticate_pass {
            let vals = CasbinVals {
                subject: authenticate_username,
                domain: None,
            };
            req.extensions_mut().insert(vals);
            Box::pin(async move { Ok(inner.call(req).await?.map(body::boxed)) })
        } else {
            Box::pin(async move {
                Ok(Json(ResponseBody::new(
                    constants::MESSAGE_INVALID_TOKEN,
                    constants::EMPTY,
                ))
                .into_response())
            })
        }
    }
}
