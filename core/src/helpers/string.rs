use std::env;

use argon2::Config;

pub fn password_hash(password: String) -> String {
    let salt = env::var("MAILER_APP_KEY").unwrap();
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
}

pub fn password_verify(hash: &str, password: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap()
}

pub fn string(str: &str) -> String {
    str.to_string()
}
