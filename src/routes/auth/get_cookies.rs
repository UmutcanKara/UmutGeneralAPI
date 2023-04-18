use crate::state::auth;
use rocket::{
    http::{Cookie, CookieJar},
    State,
};
use uuid::Uuid;

#[get("/cookies")]
pub fn get_cookies(
    cookies: &CookieJar<'_>,
    auth_state: &State<auth::AuthState>,
) -> Result<String, String> {
    let session_id = cookies.get_private("session_id");
    let result = match session_id {
        Some(id) => validate_session_id(id.value(), auth_state),
        None => create_session_id(cookies),
    };

    print!("I am fired");

    if result {
        return Ok("Session id is valid".to_owned());
    } else {
        return Err("Session id is invalid or an unexpected error".to_owned());
    }
}

fn validate_session_id(cookie_session_id: &str, auth_state: &State<auth::AuthState>) -> bool {
    cookie_session_id == auth_state.session_id
}

fn create_session_id(cookies: &CookieJar<'_>) -> bool {
    let id = Uuid::new_v4();
    cookies.add_private(Cookie::new("session_id", id.to_string()));
    true
}
