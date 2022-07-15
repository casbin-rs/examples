#![allow(clippy::type_complexity)]
use crate::{
    config::db::Pool, constants, models::response::ResponseBody, utils::token_utils,
};

use actix_casbin_auth::CasbinVals;
use actix_service::{Service, Transform};
use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    http::{
        header::{HeaderName, HeaderValue},
        Method,
    },
    web::Data,
    Error, HttpMessage, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}
pub struct AuthenticationMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let mut authenticate_pass: bool = false;
        let mut public_route: bool = false;
        let mut authenticate_username: String = String::from("");

        // Bypass some account routes
        let headers = req.headers_mut();
        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            for ignore_route in constants::IGNORE_ROUTES.iter() {
                if req.path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    public_route = true;
                }
            }
            if !authenticate_pass {
                if let Some(pool) = req.app_data::<Data<Pool>>() {
                    info!("Connecting to database...");
                    if let Some(authen_header) =
                        req.headers().get(constants::AUTHORIZATION)
                    {
                        info!("Parsing authorization header...");
                        if let Ok(authen_str) = authen_header.to_str() {
                            if authen_str.starts_with("bearer")
                                || authen_str.starts_with("Bearer")
                            {
                                info!("Parsing token...");
                                let token = authen_str[6..authen_str.len()].trim();
                                if let Ok(token_data) =
                                    token_utils::decode_token(token.to_string())
                                {
                                    info!("Decoding token...");
                                    if token_utils::verify_token(&token_data, pool)
                                        .is_ok()
                                    {
                                        info!("Valid token");
                                        authenticate_username = token_data.claims.user;
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
            if public_route {
                let vals = CasbinVals {
                    subject: "anonymous".to_string(),
                    domain: None,
                };
                req.extensions_mut().insert(vals);
                Box::pin(async move { srv.call(req).await.map(|res| res.map_into_left_body()) })
            } else {
                let vals = CasbinVals {
                    subject: authenticate_username,
                    domain: None,
                };
                req.extensions_mut().insert(vals);
                Box::pin(async move { srv.clone().call(req).await.map(|res| res.map_into_left_body()) })
            }
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(ResponseBody::new(
                            constants::MESSAGE_INVALID_TOKEN,
                            constants::EMPTY,
                        ))
                        .map_into_right_body(),
                ))
            })
        }
    }
}
