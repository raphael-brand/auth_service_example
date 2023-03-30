#![allow(dead_code, unused_variables)]

mod auth_utils;
mod database;

use auth_utils::*;
use database::*;

pub fn authenticate(creds: models::Credentials) {
    if let Status::Connected = connect_to_database() {
        auth_utils::login(creds);
    }
}
