#![allow(dead_code, unused_variables)]
struct Credentials {
    username: String,
    password: String,
}

fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds);
    }
}

fn connect_to_database() -> Status {
    return Status::Connected;
}

fn login(creds: Credentials) {
    get_user();
}

fn logout() {
    // log user out ...
}
fn get_user() {
    // get user form database ...
}

