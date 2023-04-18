#[macro_use]
extern crate rocket;

pub mod constants;
pub mod db;
pub mod routes;
pub mod state;

#[launch]
fn rocket() -> _ {
    // let rocket = rocket::build();
    let rocket = rocket::build().manage(state::auth::AuthState {
        session_id: "".to_owned(),
    });
    routes::auth::crypt::encrypt("test");
    rocket.mount("/", routes![routes::index::index]).mount(
        "/api",
        routes![
            routes::auth::get_user::get_user,
            routes::auth::get_users::get_users,
            routes::auth::post_register::post_register,
            routes::auth::get_cookies::get_cookies,
            routes::auth::post_login::post_login,
            routes::auth::post_logout::post_logout,
        ],
    )
}
