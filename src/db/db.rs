use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::db::models;
use crate::db::schema::users::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn db_get_users() -> Vec<models::User> {
    let connection = &mut establish_connection();
    let results = users
        .limit(10)
        .load::<models::User>(connection)
        .expect("Error loading posts");
    results
}
