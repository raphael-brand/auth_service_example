#![allow(dead_code, unused_variables)]

mod database;
use database::database::*;

mod auth_utils {
    use crate::database::database;

    pub fn login(creds: models::Credentials) {
        database::get_user();
    }

    pub fn logout() {
        // log user out ...
    }

    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}
use auth_utils::models::Credentials;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        auth_utils::login(creds);
    }
}
