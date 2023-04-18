#[macro_use]
extern crate rocket;

pub mod db;
pub mod routes;
pub mod state;

#[launch]
fn rocket() -> _ {
    // let rocket = rocket::build();
    let rocket = rocket::build().manage(state::auth::AuthState {
        session_id: "".to_owned(),
    });
    rocket.mount("/", routes![routes::index::index]).mount(
        "/api",
        routes![
            routes::get_user::get_user,
            routes::get_users::get_users,
            routes::post_register::post_register,
            routes::get_cookies::get_cookies,
            routes::post_login::post_login,
        ],
    )
}
