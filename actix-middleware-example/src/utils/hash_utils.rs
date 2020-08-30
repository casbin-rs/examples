use crate::constants;
use crate::errors::ServiceError;
use actix_web::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::env;

pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    // get the hashing cost from the env variable or use default
    let hashing_cost: u32 = match env::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    hash(plain, hashing_cost).map_err(|_| {
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
        )
    })
}

pub fn verify_password(plain: &str, hash: &str) -> Result<bool, ServiceError> {
    verify(plain, hash).map_err(|_| {
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
        )
    })
}
