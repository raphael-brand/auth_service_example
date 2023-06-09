#![allow(dead_code, unused_variables)]

mod auth_utils;
mod database;

use auth_utils::models::Credentials;
use database::*;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        auth_utils::login(creds);
    }
}
