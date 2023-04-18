use argon2::Config;
use dotenvy::dotenv;
use std::env;

pub fn encrypt(input: &str) -> String {
    dotenv().ok();
    let salt = env::var("SALT").expect("SALT must be set");
    let config = Config::default();
    let hash = argon2::hash_encoded(input.as_bytes(), salt.as_bytes(), &config).unwrap();

    hash
}

pub fn decrypt(encoded: &str, pwd: &[u8]) -> bool {
    argon2::verify_encoded(encoded, pwd)
        .map_err(|e| println!("Error: {}", e))
        .unwrap()
}
