use constance::testing_only::ConnectionOptions;
use std::env;

pub mod mssql;

pub fn get_connection_options_from_env() -> ConnectionOptions {
    let port = env::var("PORT").map(|port| port.parse::<u16>().unwrap());
    let host = env::var("HOST");
    let password = env::var("PASSWORD");
    let user = env::var("USER");
    match (port, host, password, user) {
        (Ok(port), Ok(host), Ok(password), Ok(user)) => {
            ConnectionOptions::new(Some(host), Some(port), Some(user), password)
        }
        _ => ConnectionOptions::empty(),
    }
}
