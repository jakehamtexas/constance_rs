use constance::{
    testing_only::{
        Column, Connection, ConnectionOptions, ConstanceRc, QueryExecutionOptions, TableIdentifier,
        TableOption, MSSQL, NUMBER_TYPE, STRING_TYPE,
    },
    types::OutputOptions,
};

pub fn get_simple_enum_rc(options: ConnectionOptions) -> ConstanceRc {
    let table_options = get_table_options(
        "simple_enum",
        "name".to_string(),
        &[Column {
            name: "id".to_string(),
            data_type: NUMBER_TYPE.to_string(),
        }],
        None,
    );
    let base_rc = get_base_rc(options);
    ConstanceRc {
        table_options,
        ..base_rc
    }
}

pub fn get_string_enum_rc(options: ConnectionOptions) -> ConstanceRc {
    let table_options = get_table_options(
        "string_enum",
        "name".to_string(),
        &[Column {
            name: "string_id".to_string(),
            data_type: STRING_TYPE.to_string(),
        }],
        None,
    );
    let base_rc = get_base_rc(options);
    ConstanceRc {
        table_options,
        ..base_rc
    }
}

fn get_base_rc(options: ConnectionOptions) -> ConstanceRc {
    ConstanceRc {
        table_options: vec![],
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

fn get_table_options(
    table_name: &str,
    key_column_name: String,
    value_column: &[Column],
    description_column_name: Option<String>,
) -> Vec<TableOption> {
    let identifier = get_identifier(table_name);
    vec![TableOption {
        identifier,
        key_column_name,
        value_columns: value_column.to_vec(),
        description_column_name,
    }]
}

pub fn get_simple_enum_with_description_rc(options: ConnectionOptions) -> ConstanceRc {
    let table_options = get_table_options(
        "simple_enum_with_description",
        "name".to_string(),
        &[Column {
            name: "id".to_string(),
            data_type: NUMBER_TYPE.to_string(),
        }],
        Some("description".to_string()),
    );
    let base_rc = get_base_rc(options);
    ConstanceRc {
        table_options,
        ..base_rc
    }
}

pub fn get_string_enum_with_description_rc(options: ConnectionOptions) -> ConstanceRc {
    let table_options = get_table_options(
        "string_enum_with_description",
        "name".to_string(),
        &[Column {
            name: "string_id".to_string(),
            data_type: STRING_TYPE.to_string(),
        }],
        Some("description".to_string()),
    );
    let base_rc = get_base_rc(options);
    ConstanceRc {
        table_options,
        ..base_rc
    }
}
