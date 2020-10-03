use constance::{
    testing_only::{
        Column, Connection, ConnectionOptions, ConstanceRc, QueryExecutionOptions, TableIdentifier,
        TableOption, MSSQL, NUMBER_TYPE,
    },
    types::OutputOptions,
};

pub fn get_simple_enum_rc(options: ConnectionOptions) -> ConstanceRc {
    let identifier = get_identifier("simple_enum");
    ConstanceRc {
        table_options: vec![TableOption {
            identifier,
            key_column_name: "name".to_string(),
            value_columns: vec![Column {
                name: "id".to_string(),
                data_type: NUMBER_TYPE.to_string(),
            }],
            description_column_name: None,
        }],
        output_options: Some(OutputOptions::default()),
        query_execution_options: QueryExecutionOptions {
            connection: Connection {
                connection_options: Some(options),
                connection_string: None,
            },
            rdbms: MSSQL.to_string(),
            query_timeout_in_ms: None,
            should_parallelize: None,
        },
    }
}

fn get_identifier(table_name: &str) -> TableIdentifier {
    TableIdentifier {
        database_name: "test".to_string(),
        schema_name: "dbo".to_string(),
        object_name: table_name.to_string(),
    }
}
