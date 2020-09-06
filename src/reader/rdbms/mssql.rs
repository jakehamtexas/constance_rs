use crate::{
    constancerc::dto::{
        connection::Connection, connection_options::ConnectionOptions, table_option::TableOption,
    },
    reader::{read_db::ReadDb, value_with_description::ValueWithDescription},
};
use async_trait::async_trait;
use std::collections::HashMap;
use tiberius::{AuthMethod, Client, Config};
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

    fn get_config(&self, database_name: &str) -> Config {
        let mut config = match &self.connection {
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
        config.database(database_name);
        config
    }

    pub async fn get_client(&self, table_option: &TableOption) -> Client<Compat<TcpStream>> {
        let config = self.get_config(&table_option.identifier.database_name);
        let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
        tcp.set_nodelay(true).unwrap();

        Client::connect(config, tcp.compat_write()).await.unwrap()
    }
}

#[async_trait]
impl ReadDb for Mssql {
    async fn get_records_as_simple_key_value_pairs(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, String> {
        let mut client = self.get_client(table_option).await;
        let key_column_name = &table_option.key_column_name;
        let value_column_name = table_option.value_column_names.first().unwrap();

        let query_result = client
            .query(
                format!("SELECT {}, {}", key_column_name, value_column_name),
                &[],
            )
            .await;
        let rows = match query_result {
            Ok(query) => query.into_first_result().await.unwrap(),
            _ => panic!("No results for given query parameters: [ key_column_name: {}, value_column_name: {} ]", key_column_name, value_column_name)
        };

        rows.into_iter().fold(HashMap::new(), |mut map, row| {
            let key_column = row.get::<i32, &str>(key_column_name);
            let value_column = row.get::<&str, &str>(value_column_name);
            if let (Some(key), Some(value)) = (key_column, value_column) {
                map.insert(key.to_string().to_owned(), value.to_owned());
            }
            map
        })
    }

    async fn get_records_with_meta_description_column(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, ValueWithDescription> {
        todo!()
    }
}
