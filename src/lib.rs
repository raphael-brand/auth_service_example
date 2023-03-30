#![allow(dead_code, unused_variables)]

pub struct Credentials {
    username: String,
    password: String,
}

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

pub fn authenticate(creds: Credentials) {
    use crate::database::connect_to_database;
    if let database::Status::Connected = connect_to_database() {
        login(creds);
    }
}

pub fn login(creds: Credentials) {
    get_user();
}

pub fn logout() {
    // log user out ...
}
