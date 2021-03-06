use crate::constancerc::dto::column::Column;
use crate::{
    constancerc::dto::{
        column_type::ColumnType, connection::Connection, connection_options::ConnectionOptions,
        table_option::TableOption,
    },
    reader::{read_db::ReadDb, value_with_description::ValueWithDescription},
};
use async_trait::async_trait;
use std::collections::HashMap;
use tiberius::{error::Error, AuthMethod, Client, Config, EncryptionLevel, FromSql, Row};
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
    config.encryption(EncryptionLevel::NotSupported);
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
    column_name: Option<&String>,
) -> Option<String> {
    match column_name {
        Some(name) => row
            .get::<ToStringable, &str>(name)
            .map(|to_stringable| to_stringable.to_string()),
        None => None,
    }
}

fn get_select_statement(columns: &[&str], table_identifier: &str) -> String {
    format!("SELECT {} FROM {}", columns.join(", "), table_identifier)
}

fn get_panic_message(columns: &[&str]) -> String {
    format!(
        "No results for given query parameters: [ {} ]",
        columns.join(", ")
    )
}

fn get_column_of_unknown_type(row: &Row, column: &Column) -> Option<String> {
    let column_type = &column.data_type;
    let column_name = &column.name;
    match ColumnType::from_string(column_type) {
        ColumnType::Number => get_column::<i32>(&row, Some(&column_name)),
        ColumnType::String => get_column::<&str>(&row, Some(&column_name)),
    }
}

fn get_table_identifier(table_option: &TableOption) -> String {
    let identifier = &table_option.identifier;
    vec![
        identifier.database_name.clone(),
        identifier.schema_name.clone(),
        identifier.object_name.clone(),
    ]
    .join(".")
}

async fn get_rows(
    mut client: Client<Compat<TcpStream>>,
    table_option: &TableOption,
    columns: &[&str],
) -> Result<Vec<Row>, Error> {
    let identifier = get_table_identifier(table_option);
    let query = client
        .query(get_select_statement(columns, &identifier), &[])
        .await
        .map(|query| query.into_first_result());
    match query {
        Ok(query) => query.await,
        Err(_) => panic!(get_panic_message(columns)),
    }
}

struct ColumnsDto<'a> {
    pub key_column_name: &'a String,
    pub value_columns: &'a Vec<Column>,
    pub description_column_name: &'a Option<String>,
    pub column_names: Vec<&'a str>,
}
fn get_columns<'a>(table_option: &'a TableOption) -> ColumnsDto<'a> {
    let key_column_name = &table_option.key_column_name;
    let description_column_name = match &table_option.description_column_name {
        Some(ref d) => d,
        None => "",
    };
    let value_columns = &table_option.value_columns;
    let value_column_names = value_columns
        .iter()
        .map(|Column { name, .. }| name.as_str())
        .collect::<Vec<&str>>();
    let nested_columns: Vec<Vec<&str>> = vec![
        vec![&key_column_name],
        vec![&description_column_name],
        value_column_names,
    ];
    let column_names = nested_columns
        .into_iter()
        .flatten()
        .filter(|str| !str.is_empty())
        .collect::<Vec<&str>>();

    ColumnsDto {
        key_column_name,
        value_columns,
        description_column_name: &table_option.description_column_name,
        column_names,
    }
}

#[async_trait]
impl ReadDb for Mssql {
    async fn get_records_as_simple_key_value_pairs(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, ValueWithDescription> {
        let columns_dto = get_columns(table_option);
        let value_column = columns_dto.value_columns.first().unwrap();

        let client = self.get_client(table_option).await;
        let rows = get_rows(client, &table_option, &columns_dto.column_names).await;

        rows.map(|rows| {
            rows.iter().fold(HashMap::new(), |mut map, row| {
                let key_column = get_column::<&str>(row, Some(columns_dto.key_column_name));
                let value_column = get_column_of_unknown_type(row, &value_column);
                let description =
                    get_column::<&str>(row, columns_dto.description_column_name.as_ref());
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
        table_option: &TableOption,
    ) -> HashMap<ValueWithDescription, Vec<(Column, String)>> {
        let columns_dto = get_columns(table_option);

        let client = self.get_client(table_option).await;
        let rows = get_rows(client, &table_option, &columns_dto.column_names).await;
        rows.map(|rows| {
            rows.iter().fold(HashMap::new(), |mut map, row| {
                let key_column = get_column::<&str>(row, Some(columns_dto.key_column_name));
                let description_column =
                    get_column::<&str>(row, columns_dto.description_column_name.as_ref());
                let value_columns = columns_dto
                    .value_columns
                    .iter()
                    .map(|column| {
                        get_column_of_unknown_type(row, column)
                            .map(|column_value| (column.to_owned(), column_value))
                    })
                    .filter_map(|v| v)
                    .collect::<Vec<(Column, String)>>();
                if let Some(key) = key_column {
                    let value_with_description = ValueWithDescription {
                        value: key,
                        description: description_column,
                    };
                    map.insert(value_with_description, value_columns);
                }
                map
            })
        })
        .unwrap()
    }
}
