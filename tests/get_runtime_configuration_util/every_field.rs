use constance::testing_only::ConstanceRc;

use super::common::{bool_assert, num_assert, string_assert, vec_assert};

pub fn assert(rc: ConstanceRc) {
    let output_options = rc.output_options.unwrap();

    let output_options_path = &output_options.path.unwrap();
    let language_targets = &output_options.language_targets.unwrap();

    string_assert(output_options_path);
    vec_assert(language_targets);

    let table_option = rc.table_options.first().unwrap();

    let identifier = &table_option.identifier;
    let database_name = &identifier.database_name;
    let schema_name = &identifier.schema_name;
    let object_name = &identifier.object_name;

    let key_column_name = &table_option.key_column_name;
    let value_columns = &table_option.value_columns;
    let description_column_name = &table_option.description_column_name.as_ref().unwrap();

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
    string_assert(description_column_name);

    let query_execution_options = rc.query_execution_options;

    let connection = &query_execution_options.connection;
    let connection_string = connection.connection_string.as_ref().unwrap();

    let connection_options = connection.connection_options.as_ref().unwrap();
    let host = &connection_options.get_host();
    let port = connection_options.get_port();
    let user_name = &connection_options.get_user_name();
    let password = &connection_options.get_password();

    let rdbms = &query_execution_options.rdbms;
    let query_timeout_in_ms = query_execution_options.query_timeout_in_ms.unwrap();
    let should_parallelize = query_execution_options.should_parallelize.unwrap();

    string_assert(connection_string);

    string_assert(host);
    num_assert(i32::from(port));
    string_assert(user_name);
    string_assert(password);

    string_assert(rdbms);
    num_assert(query_timeout_in_ms);
    bool_assert(should_parallelize);
}
