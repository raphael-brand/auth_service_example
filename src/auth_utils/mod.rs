use crate::database;

pub fn login(creds: models::Credentials) {
    database::get_user();
}

pub fn logout() {
    // log user out ...
}

pub mod models;
