use crate::{
    constants,
    model::{db::Connection, user_token::UserToken},
    schema::users::{self, dsl::*},
    utils::bcrypt::{compare_password, hash_password},
};

use diesel::prelude::*;
use diesel::QueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use axum_macros::FromRequest;

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

#[derive(Insertable, Serialize, Deserialize, AsChangeset, Clone, FromRequest)]
#[table_name = "users"]
pub struct AddUser {
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(default = "default_role")]
    pub role: String,
    pub login_session: String,
}

fn default_role() -> String {
    "patient".to_owned()
}

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
    pub fn register(user: AddUser, conn: &Connection) -> Result<String, String> {
        if Self::get_user_by_email(&user.email, conn).is_err() {
            let hashed_pwd = hash_password(&user.password).unwrap();
            let user_upd = AddUser {
                password: hashed_pwd,
                ..user
            };
            diesel::insert_into(users)
                .values(&user_upd)
                .execute(conn)
                .map_err(|e| e.to_string())?;

            Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
        } else {
            Err(format!("User {} is already registered", &user.username))
        }
    }

    pub fn update_user(i: i32, user_data: AddUser, conn: &Connection) -> Result<String, String> {
        if Self::get_user_by_email(&user_data.email, conn).is_err() {
            Err(format!("User is not present"))
        } else {
            let hashed_pwd = hash_password(&user_data.password).unwrap();
            let user_update = AddUser {
                password: hashed_pwd,
                ..user_data
            };
            diesel::update(users.find(i)).set(&user_update).execute(conn).map_err(|e| e.to_string())?;

            Ok(constants::MESSAGE_UPDATE_USER_SUCCESS.to_string())
        }
    }

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

    

    pub fn delete_user(delete_id: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(users.filter(users::id.eq(delete_id))).execute(conn)
    }

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
