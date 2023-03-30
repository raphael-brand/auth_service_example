#![allow(dead_code, unused_variables)]

mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }
    pub fn connect_to_database() -> Status {
        return Status::Connected;
    }
    pub fn get_user() {
        // get user form database ...
    }
}

mod auth_utils {
    pub fn login(creds: models::Credentials) {
        crate::database::get_user();
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
use database::Status;
pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
