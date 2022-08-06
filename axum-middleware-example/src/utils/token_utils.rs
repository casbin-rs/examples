use crate::model::{
    db::Pool,
    user::User,
    user_token::{UserToken, KEY},
};
use axum::extract::Extension;
use jsonwebtoken::{DecodingKey, TokenData, Validation};

// DECODE THE GOT TOKEN
pub fn decode_token(
    token: String,
) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

// VALIDATE TOKEN
pub fn validate_token(
    token_data: &TokenData<UserToken>,
    Extension(pool): &Extension<Pool>,
) -> Result<String, String> {
    if User::is_valid_login_session(&token_data.claims, &pool.get().unwrap()) {
        Ok(token_data.claims.user_name.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}
