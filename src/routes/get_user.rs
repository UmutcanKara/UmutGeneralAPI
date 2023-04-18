use crate::db::db::establish_connection;
use crate::db::models::User;
use rocket::serde::json::Json;

use diesel::prelude::*;

#[get("/user/<mail>")]
pub fn get_user(mail: String) -> Result<Json<User>, String> {
    use crate::db::schema::users::dsl::*;
    let conn = &mut establish_connection();
    let result = users
        .filter(email.eq(mail))
        .first::<User>(conn)
        .map_err(|e| e.to_string());

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e.to_string()),
    }
}
