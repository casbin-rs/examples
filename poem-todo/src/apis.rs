use crate::models::*;
use crate::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use poem::web::{Data, Json};
use poem::{handler, web::Path};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

#[handler]
pub fn hello(user: Data<&User>) -> String {
    format!("hello: {}", user.name)
}

#[handler]
pub fn get_users(mut conn: Data<&Arc<Mutex<PgConnection>>>) -> Json<Vec<User>> {
    let results = users::table
        .load::<User>(conn.0.lock().unwrap().deref_mut())
        .expect("Error loading users");
    Json(results)
}

#[handler]
pub fn get_todos(mut conn: Data<&Arc<Mutex<PgConnection>>>) -> Json<Vec<Todo>> {
    let results = todos::table
        .load::<Todo>(conn.0.lock().unwrap().deref_mut())
        .expect("Error loading todos");
    Json(results)
}

#[handler]
pub fn get_todo(
    user: Data<&User>,
    Path(todo_id): Path<i32>,
    mut conn: Data<&Arc<Mutex<PgConnection>>>,
) -> Json<Todo> {
    let mut result =
        Todo::find_by_id(todo_id, user.0, conn.0.lock().unwrap().deref_mut()).unwrap();
    Json(result)
}

#[handler]
pub fn update_todo(
    user: Data<&User>,
    Path(todo_id): Path<i32>,
    mut req: Json<NewTodo>,
    mut conn: Data<&Arc<Mutex<PgConnection>>>,
) -> Json<Todo> {
    let _ =
        Todo::find_by_id(todo_id, user.0, conn.0.lock().unwrap().deref_mut()).unwrap();
    req.user_id = user.id;
    let _ = Todo::update(todo_id, req.0, conn.0.lock().unwrap().deref_mut()).unwrap();
    let mut result =
        Todo::find_by_id(todo_id, user.0, conn.0.lock().unwrap().deref_mut());
    Json(result.unwrap())
}

#[handler]
pub fn delete_todo(
    user: Data<&User>,
    Path(todo_id): Path<i32>,
    mut conn: Data<&Arc<Mutex<PgConnection>>>,
) -> Json<usize> {
    let _ =
        Todo::find_by_id(todo_id, user.0, conn.0.lock().unwrap().deref_mut()).unwrap();
    let result = Todo::delete(todo_id, conn.0.lock().unwrap().deref_mut());
    Json(result.unwrap())
}

#[handler]
pub fn get_self_todos(
    user: Data<&User>,
    mut conn: Data<&Arc<Mutex<PgConnection>>>,
) -> Json<Vec<Todo>> {
    let results =
        User::find_self_todos(user.id, conn.0.lock().unwrap().deref_mut()).unwrap();
    Json(results)
}

#[handler]
pub fn create_todo(
    user: Data<&User>,
    mut req: Json<NewTodo>,
    mut conn: Data<&Arc<Mutex<PgConnection>>>,
) -> Json<Todo> {
    req.user_id = user.id;
    let result = Todo::insert(req.0, conn.0.lock().unwrap().deref_mut()).unwrap();
    Json(result)
}

#[handler]
pub fn get_user_todos(
    Path(name): Path<String>,
    mut conn: Data<&Arc<Mutex<PgConnection>>>,
) -> Json<Vec<Todo>> {
    let user = User::find_by_name(name, conn.0.lock().unwrap().deref_mut());
    if user.is_err() {
        return Json(vec![]);
    }
    let user = user.unwrap();
    if user.is_none() {
        return Json(vec![]);
    }
    let results =
        User::find_self_todos(user.unwrap().id, conn.0.lock().unwrap().deref_mut())
            .unwrap();
    Json(results)
}
