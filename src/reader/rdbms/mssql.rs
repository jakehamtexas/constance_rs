use crate::{
    constancerc::dto::{
        connection::Connection, connection_options::ConnectionOptions,
        key_column_type::KeyColumnType, table_option::TableOption,
    },
    reader::{read_db::ReadDb, value_with_description::ValueWithDescription},
};
use async_trait::async_trait;
use std::collections::HashMap;
use tiberius::{AuthMethod, Client, Config, FromSql, Row};
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

fn get_column<'a, ToStringable: ToString + FromSql<'a>>(
    row: &'a Row,
    column_name: &str,
) -> Option<String> {
    match row.get::<ToStringable, &str>(column_name) {
        Some(i) => Some(i.to_string()),
        None => None,
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

        let query = client
            .query(
                format!("SELECT {}, {}", key_column_name, value_column_name),
                &[],
            )
            .await
            .map(|query| query.into_first_result());

        let rows = match query {
            Ok(rows) => rows.await,
            _ => panic!("No results for given query parameters: [ key_column_name: {}, value_column_name: {} ]", key_column_name, value_column_name)
        };

        let key_column_type = KeyColumnType::from_option(&table_option.key_column_type);

        rows.map(|rows| {
            rows.into_iter().fold(HashMap::new(), |mut map, row| {
                let key_column = match &key_column_type {
                    KeyColumnType::Number => get_column::<i32>(&row, key_column_name),
                    KeyColumnType::String => get_column::<&str>(&row, key_column_name),
                };
                let value_column = get_column::<&str>(&row, value_column_name);
                if let (Some(key), Some(value)) = (key_column, value_column) {
                    map.insert(key, value);
                }
                map
            })
        })
        .unwrap()
    }

    async fn get_records_with_meta_description_column(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, ValueWithDescription> {
        todo!()
    }
}
