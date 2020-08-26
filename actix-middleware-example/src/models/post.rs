use crate::{
    config::db::Connection,
    schema::posts::{self, dsl::*},
};
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub is_deleted: bool,
    pub created_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    #[serde(default = "chrono_now")]
    pub created_at: chrono::NaiveDateTime,
}

fn chrono_now() -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "posts"]
pub struct DeletePost {
    pub is_deleted: bool,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl Post {
    pub fn find_all(is_admin: bool, conn: &Connection) -> QueryResult<Vec<Post>> {
        if is_admin {
            return posts::table.order(id.asc()).load::<Post>(conn);
        }
        posts::table
            .filter(is_deleted.eq(false))
            .order(id.asc())
            .load::<Post>(conn)
    }

    pub fn find_by_id(is_admin: bool, i: i32, conn: &Connection) -> QueryResult<Post> {
        if is_admin {
            return posts::table.find(i).get_result::<Post>(conn);
        }
        posts::table
            .filter(is_deleted.eq(false))
            .find(i)
            .get_result::<Post>(conn)
    }

    pub fn insert(new_post: NewPost, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(posts).values(&new_post).execute(conn)
    }

    // pub fn update(
    //     i: i32,
    //     update_post: NewPost,
    //     conn: &Connection,
    // ) -> QueryResult<usize> {
    //     diesel::update(posts.find(i))
    //         .set(&update_post)
    //         .execute(conn)
    // }

    pub fn delete(
        delete_id: i32,
        delete_post: DeletePost,
        conn: &Connection,
    ) -> QueryResult<usize> {
        diesel::update(posts.find(delete_id))
            .set(&delete_post)
            .execute(conn)
    }
}
