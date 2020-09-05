use serde::Deserialize;

const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: u16 = 1433;
const DEFAULT_USERNAME: &str = "SA";

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionOptions {
    host: Option<String>,
    port: Option<u16>,
    user_name: Option<String>,
    password: String,
}

impl ConnectionOptions {
    pub fn get_host(&self) -> String {
        match &self.host {
            Some(host) => host.to_owned(),
            None => DEFAULT_HOST.to_string(),
        }
    }

    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or(DEFAULT_PORT)
    }

    pub fn get_user_name(&self) -> String {
        match &self.user_name {
            Some(user_name) => user_name.to_owned(),
            None => DEFAULT_USERNAME.to_string(),
        }
    }

    pub fn get_password(&self) -> String {
        self.password.to_owned()
    }
}
