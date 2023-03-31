fn main() {
    let creds: Credentials = Credentials {username: "thisisanewuser".to_string(), password: "aödslfkj".to_string()};
    authenticate(creds);
}
