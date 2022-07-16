use crate::schema::*;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = todos)]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub finished: bool,
}
