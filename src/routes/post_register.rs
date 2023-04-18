use crate::routes::get_cookies::get_cookies;
use crate::{
    db::{
        db::establish_connection,
        models::{NewUser, User},
        schema::users,
    },
    state::auth::AuthState,
};
use diesel::RunQueryDsl;
use rocket::{
    http::CookieJar,
    serde::{json::Json, Deserialize, Serialize},
    State,
};

#[derive(Serialize, Deserialize)]
pub struct ResultMessage {
    status: String,
}
impl ResultMessage {
    pub fn new(status: String) -> Self {
        ResultMessage { status }
    }
}

#[post("/register", format = "json", data = "<user>")]
pub fn post_register(
    user: Json<NewUser>,
    cookies: &CookieJar<'_>,
    auth_state: &State<AuthState>,
) -> Result<Json<ResultMessage>, String> {
    let conn = &mut establish_connection();
    let new_user = NewUser {
        name: &user.name,
        email: &user.email,
        password: &user.password,
    };

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn);

    print!("{:?}", result);

    match result {
        Ok(_) => {
            let result = get_cookies(cookies, auth_state);
            match result {
                Ok(_) => Ok(Json(ResultMessage::new("success".to_string()))),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
