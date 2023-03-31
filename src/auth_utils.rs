use crate::database;

pub fn login(creds: models::Credentials) {
    database::get_user();
    println!("logging in ...");
}

pub fn logout() {
    // log user out ...
}

pub mod models;
