use crate::model::user::LoginInfo;

use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

static THREE_HOUR: i64 = 60 * 60 * 3;
pub static KEY: [u8; 16] = *include_bytes!("../secret.key");

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // userID
    pub user_name: String,
    pub role: String,
    pub login_session: String,
}

impl UserToken {
    pub fn generate_token(login: LoginInfo) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000;
        let payload = UserToken {
            iat: now,
            exp: now + THREE_HOUR,
            user_name: login.username,
            role: login.role,
            login_session: login.login_session,
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        )
        .unwrap()
    }
}
