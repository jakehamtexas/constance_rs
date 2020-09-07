use crate::{
    constancerc::dto::{
        connection::Connection, connection_options::ConnectionOptions,
        key_column_type::KeyColumnType, table_option::TableOption,
    },
    reader::{read_db::ReadDb, value_with_description::ValueWithDescription},
};
use async_trait::async_trait;
use std::collections::HashMap;
use tiberius::{error::Error, AuthMethod, Client, Config, FromSql, Row};
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

    async fn get_client(&self, table_option: &TableOption) -> Client<Compat<TcpStream>> {
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

fn get_select_statement(columns: &[&str]) -> String {
    format!("SELECT {}", columns.join(", "))
}

fn get_panic_message(columns: &[&str]) -> String {
    format!(
        "No results for given query parameters: [ {} ]",
        columns.join(", ")
    )
}

async fn get_rows(
    mut client: Client<Compat<TcpStream>>,
    columns: &[&str],
) -> Result<Vec<Row>, Error> {
    let query = client
        .query(get_select_statement(columns), &[])
        .await
        .map(|query| query.into_first_result());
    match query {
        Ok(query) => query.await,
        _ => panic!(get_panic_message(columns)),
    }
}

fn get_key_column(
    row: &Row,
    key_column_type: &KeyColumnType,
    key_column_name: &str,
) -> Option<String> {
    match key_column_type {
        KeyColumnType::Number => get_column::<i32>(&row, key_column_name),
        KeyColumnType::String => get_column::<&str>(&row, key_column_name),
    }
}

#[async_trait]
impl ReadDb for Mssql {
    async fn get_records_as_simple_key_value_pairs(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, String> {
        let key_column_name = &table_option.key_column_name;
        let value_column_name = table_option.value_column_names.first().unwrap();
        let columns: [&str; 2] = [key_column_name, value_column_name];

        let client = self.get_client(table_option).await;
        let rows = get_rows(client, &columns).await;

        let key_column_type = KeyColumnType::from_option(&table_option.key_column_type);

        rows.map(|rows| {
            rows.iter().fold(HashMap::new(), |mut map, row| {
                let key_column = get_key_column(row, &key_column_type, key_column_name);
                let value_column = get_column::<&str>(row, value_column_name);
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
        let key_column_name = &table_option.key_column_name;
        let value_column_name = table_option.value_column_names.first().unwrap();
        let description_column_name = match &table_option.description_column_name {
            Some(ref d) => d,
            None => "",
        };
        let columns = [key_column_name, value_column_name, description_column_name];

        let client = self.get_client(table_option).await;
        let rows = get_rows(client, &columns).await;

        let key_column_type = KeyColumnType::from_option(&table_option.key_column_type);

        rows.map(|rows| {
            rows.iter().fold(HashMap::new(), |mut map, row| {
                let key_column = get_key_column(row, &key_column_type, key_column_name);
                let value_column = get_column::<&str>(row, value_column_name);
                let description =
                    get_column::<&str>(row, description_column_name).unwrap_or_default();
                if let (Some(key), Some(value)) = (key_column, value_column) {
                    let value_with_description = ValueWithDescription { value, description };
                    map.insert(key, value_with_description);
                }
                map
            })
        })
        .unwrap()
    }

    async fn get_records_as_object_like(
        &self,
        _table_option: &TableOption,
    ) -> HashMap<String, Vec<(String, String)>> {
        todo!()
    }

    async fn get_records_as_object_like_with_descriptions(
        &self,
        _table_option: &TableOption,
    ) -> HashMap<String, Vec<(String, ValueWithDescription)>> {
        todo!()
    }
}
