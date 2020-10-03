use constance::testing_only::ConstanceRc;

use super::common::{option_assert, string_assert, vec_assert};

pub fn assert(rc: ConstanceRc) {
    let output_options = &rc.output_options;
    option_assert(output_options);

    let table_option = rc.table_options.first().unwrap();

    let identifier = &table_option.identifier;
    let database_name = &identifier.database_name;
    let schema_name = &identifier.schema_name;
    let object_name = &identifier.object_name;

    let key_column_name = &table_option.key_column_name;
    let value_columns = &table_option.value_columns;
    let description_column_name = &table_option.description_column_name;

    string_assert(database_name);
    string_assert(schema_name);
    string_assert(object_name);

    string_assert(key_column_name);
    vec_assert(
        &value_columns
            .iter()
            .map(|c| c.data_type.to_owned())
            .collect::<Vec<_>>(),
    );
    vec_assert(
        &value_columns
            .iter()
            .map(|c| c.name.to_owned())
            .collect::<Vec<_>>(),
    );
    option_assert(description_column_name);

    let query_execution_options = rc.query_execution_options;
    let password = &query_execution_options
        .connection
        .connection_options
        .unwrap()
        .get_password();
    let rdbms = &query_execution_options.rdbms;
    let query_timeout_in_ms = &query_execution_options.query_timeout_in_ms;
    let should_parallelize = &query_execution_options.should_parallelize;

    string_assert(password);
    string_assert(rdbms);
    option_assert(query_timeout_in_ms);
    option_assert(should_parallelize);
}
