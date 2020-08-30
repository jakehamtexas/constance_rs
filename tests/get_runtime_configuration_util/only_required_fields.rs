use super::common::{option_assert, string_assert, vec_assert};
use constance::types::ConstanceRc;

pub fn assert(rc: ConstanceRc) {
    let output_options = &rc.output_options;
    option_assert(output_options);

    let table_option = rc.table_options.first().unwrap();

    let identifier = &table_option.identifier;
    let instance_name = &identifier.instance_name;
    let database_name = &identifier.database_name;
    let schema_name = &identifier.schema_name;
    let object_name = &identifier.object_name;

    let key_column_name = &table_option.key_column_name;
    let value_column_names = &table_option.value_column_names;
    let description_column_name = &table_option.description_column_name;

    string_assert(instance_name);
    string_assert(database_name);
    string_assert(schema_name);
    string_assert(object_name);

    string_assert(key_column_name);
    vec_assert(value_column_names);
    option_assert(description_column_name);

    let query_execution_options = rc.query_execution_options;
    let conn_string = &query_execution_options.conn_string;
    let rdbms = &query_execution_options.rdbms;
    let query_timeout_in_ms = &query_execution_options.query_timeout_in_ms;
    let should_parallelize = &query_execution_options.should_parallelize;

    string_assert(conn_string);
    string_assert(rdbms);
    option_assert(query_timeout_in_ms);
    option_assert(should_parallelize);
}
