use crate::{
    model::{db::Connection, user_token::UserToken},
    schema::users::{self, dsl::*},
    utils::bcrypt::compare_password,
};

use chrono::Utc;
use diesel::prelude::*;
use diesel::QueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
// #[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub login_session: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    // #[serde(default = "default_role")]
    pub role: String,
    pub login_session: String
}

// fn default_role() -> String {
//     DEFAULT_USER_ROLE
// }

// pub const DEFAULT_USER_ROLE: String = "patient";

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct LoginInfo {
    // WILL GET UNIQUE USERNAME BY EMAIL
    pub username: String,
    pub role: String,
    pub login_session: String,
}

impl User {
    pub fn add_user() {}

    pub fn get_user(i: i32, conn: &Connection) -> QueryResult<User> {
        users.find(i).get_result::<User>(conn)
    }

    pub fn get_all_user(conn: &Connection) -> QueryResult<Vec<User>> {
        users.order(id.asc()).load::<User>(conn)
    }
    
    pub fn signin(login: LoginForm, conn: &Connection) -> Option<LoginInfo> {
        if let Ok(user_to_verify) = users
            .filter(email.eq(&login.email))
            .get_result::<User>(conn)
        {
            if !user_to_verify.password.is_empty()
                && compare_password(&login.password, &user_to_verify.password).unwrap()
            {
                let login_session_str = User::generate_login_session();
                if User::update_login_session_to_db(
                    &user_to_verify.email,
                    &login_session_str,
                    conn,
                ) {
                    return Some(LoginInfo {
                        username: user_to_verify.username,
                        role: user_to_verify.role,
                        login_session: login_session_str,
                    });
                }
            }
        }
        None
    }

    pub fn update_user() {}

    pub fn delete_user() {}

    pub fn update_login_session_to_db(
        eml: &str,
        login_session_str: &str,
        conn: &Connection,
    ) -> bool {
        if let Ok(user) = User::get_user_by_email(eml, conn) {
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
    }

    pub fn is_valid_login_session(user_token: &UserToken, conn: &Connection) -> bool {
        users
            .filter(username.eq(&user_token.user_name))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn get_user_by_email(eml: &str, conn: &Connection) -> QueryResult<User> {
        users.filter(email.eq(eml)).get_result::<User>(conn)
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().simple().to_string()
    }
}
