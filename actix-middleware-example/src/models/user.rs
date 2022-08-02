#![allow(clippy::extra_unused_lifetimes)]

use crate::{
    config::db::Connection,
    constants,
    models::user_token::UserToken,
    schema::users::{self, dsl::*},
    utils::hash_utils::{hash_password, verify_password},
};
use chrono::Utc;
use diesel::prelude::*;
use diesel::QueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: i32,
    pub is_deleted: bool,
    pub created_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub login_session: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(default = "default_role")]
    pub role: i32,
    #[serde(default = "chrono_now")]
    pub created_at: chrono::NaiveDateTime,
}

fn chrono_now() -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)
}

fn default_role() -> i32 {
    DEFAULT_USER_ROLE
}

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct LoginInfo {
    pub username: String,
    pub role: i32,
    pub login_session: String,
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "users"]
pub struct DeleteUser {
    pub is_deleted: bool,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub const DEFAULT_USER_ROLE: i32 = 2;

impl User {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<User>> {
        users.order(id.asc()).load::<User>(conn)
    }

    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<User> {
        users.find(i).get_result::<User>(conn)
    }

    pub fn signup(user: NewUser, conn: &Connection) -> Result<String, String> {
        if Self::find_user_by_username(&user.username, conn).is_err() {
            let hashed_pwd = hash_password(&user.password).unwrap();
            let user = NewUser {
                password: hashed_pwd,
                //role: DEFAULT_USER_ROLE,
                //created_at: chrono::Local::now().naive_local(),
                ..user
            };
            diesel::insert_into(users)
                .values(&user)
                .execute(conn)
                .map_err(|e| e.to_string())?;
            Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
        } else {
            Err(format!("User '{}' is already registered", &user.username))
        }
    }

    pub fn login(login: LoginForm, conn: &Connection) -> Option<LoginInfo> {
        if let Ok(user_to_verify) = users
            .filter(is_deleted.eq(false))
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(conn)
        {
            if !user_to_verify.password.is_empty()
                && verify_password(&login.password, &user_to_verify.password).unwrap()
            {
                let login_session_str = User::generate_login_session();
                if User::update_login_session_to_db(
                    &user_to_verify.username,
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

    pub fn logout(user_id: i32, conn: &Connection) {
        if let Ok(user) = users.find(user_id).get_result::<User>(conn) {
            Self::update_login_session_to_db(&user.username, "", conn);
        }
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().simple().to_string()
    }

    // pub fn get_user_role(
    //     i: i32,
    //     conn: &Connection,
    // ) -> Result<i32, diesel::result::Error> {
    //     users.find(i).get_result::<User>(conn).map(|u| u.role)
    // }

    pub fn is_valid_login_session(user_token: &UserToken, conn: &Connection) -> bool {
        users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn find_user_by_username(un: &str, conn: &Connection) -> QueryResult<User> {
        users.filter(username.eq(un)).get_result::<User>(conn)
    }

    pub fn update_login_session_to_db(
        un: &str,
        login_session_str: &str,
        conn: &Connection,
    ) -> bool {
        if let Ok(user) = User::find_user_by_username(un, conn) {
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
    }

    pub fn delete(
        i: i32,
        delete_user: DeleteUser,
        conn: &Connection,
    ) -> QueryResult<usize> {
        Self::logout(i, conn);
        diesel::update(users.find(i))
            .set(&delete_user)
            .execute(conn)
    }
}
