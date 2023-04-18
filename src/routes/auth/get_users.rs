use crate::db::{db::db_get_users, models::User};
use rocket::serde::json::Json;

#[get("/users")]
pub fn get_users() -> Json<Vec<User>> {
    let users: Vec<User> = db_get_users();
    Json(users)
}
