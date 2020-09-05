use crate::{
    constancerc::dto::{
        connection::Connection, connection_options::ConnectionOptions, table_option::TableOption,
    },
    reader::{read_db::ReadDb, value_with_description::ValueWithDescription},
};
use async_trait::async_trait;
use std::collections::HashMap;
use tiberius::{error::Error, AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, Tokio02AsyncWriteCompatExt};

#[derive(Debug)]
pub struct Mssql {
    connection: Connection,
}

fn from_options(options: &ConnectionOptions) -> Config {
    let mut config = Config::new();
    config.host(&options.get_host());
    config.port(options.get_port());
    config.authentication(AuthMethod::sql_server(
        &options.get_user_name(),
        &options.get_password(),
    ));
    config
}

impl Mssql {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    pub async fn get_client(&self) -> Result<Client<Compat<TcpStream>>, Error> {
        let connection = &self.connection;
        let config = match connection {
            Connection {
                connection_string: Some(connection_string),
                connection_options: Some(options),
            } => {
                Config::from_ado_string(connection_string).unwrap_or_else(|_| from_options(options))
            }
            Connection {
                connection_options: Some(options),
                ..
            } => from_options(options),
            _ => panic!("No connection configuration is available!"),
        };

        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        Client::connect(config, tcp.compat_write()).await
    }
}

#[async_trait]
impl ReadDb for Mssql {
    async fn get_records_as_simple_key_value_pairs(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, String> {
        let client = self.get_client().await.unwrap();
        let key_column_name = &table_option.key_column_name;
        let value_column_name = table_option.value_column_names.first().unwrap();

        todo!()
    }

    async fn get_records_with_meta_description_column(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, ValueWithDescription> {
        todo!()
    }
}
