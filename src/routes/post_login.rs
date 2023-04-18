use crate::{
    db::{
        db::establish_connection,
        models::{NewUser, User},
    },
    state::auth::AuthState,
};

use crate::routes::{get_cookies::get_cookies, post_register::ResultMessage};
use diesel::prelude::*;
use rocket::{http::CookieJar, serde::json::Json, State};

#[post("/login", format = "json", data = "<user_data>")]
pub fn post_login(
    user_data: Json<NewUser>,
    cookies: &CookieJar<'_>,
    auth_state: &State<AuthState>,
) -> Result<Json<ResultMessage>, String> {
    use crate::db::schema::users::dsl::*;

    let conn = &mut establish_connection();
    let found_user = users.filter(email.eq(&user_data.email)).first::<User>(conn);
    match found_user {
        Ok(user) => {
            if compare_users(&user, &user_data) {
                let result = get_cookies(cookies, auth_state);
                match result {
                    Ok(_) => return Ok(Json(ResultMessage::new("success".to_string()))),
                    Err(e) => return Err(e),
                }
            } else {
                return Err("Wrong credentials".to_string());
            }
        }
        Err(e) => return Err(e.to_string()),
    }
}

fn compare_users(user: &User, new_user: &NewUser) -> bool {
    user.email == new_user.email && user.password == new_user.password
    // todo!("Decrypt password then compare");
}
