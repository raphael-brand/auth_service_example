# Rust Authentication Demo
This module has been created to learn Rust - so nothing fancy.
In order to use it and call the create_credentials function you need to checkout the `dev` branch.

## How to include this crate (no, it is not public on crates.io)
1. Open your Cargo.toml file in your rust project.
2. Add the following line to the [dependencies] section:
   `auth_service_example = {path = „../auth_service_example“}`
   (Adjust the path string value, depending on where you have it)

3. Run the command `cargo install —-path .`
4. Run the command `cargo build`
5. Then go, run the compiled thingie with:
   `./target/debug/yourauthserviceproject
   And you should see the output:
```sh
$ ./target/debug/yourauthserviceproject
fetching user from db ...
logging in ...
```
