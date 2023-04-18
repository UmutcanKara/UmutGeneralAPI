use crate::{constants::SESSION_COOKIE, routes::auth::post_register::ResultMessage};
use rocket::{
    http::{Cookie, CookieJar},
    serde::json::Json,
};

#[post("/logout")]
pub fn post_logout(cookies: &CookieJar<'_>) -> Result<Json<ResultMessage>, String> {
    cookies.remove_private(Cookie::named(SESSION_COOKIE));
    Ok(Json(ResultMessage::new("success".to_string())))
}
