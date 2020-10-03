use std::collections::HashMap;

use constance::{
    functions::get_database, functions::get_table_constants, testing_only::TableConstant,
};
use table_to_constants_util::{get_connection_options_from_env, mssql};

mod common;
mod table_to_constants_util;

#[tokio::test]
pub async fn table_to_constants_mssql_simple_enum() {
    // arrange
    let connection_options = get_connection_options_from_env();
    let rc = mssql::get_simple_enum_rc(connection_options);
    let db = get_database(&rc);
    let table_options = &rc.table_options;
    let mut expected: HashMap<String, String> = HashMap::new();
    expected.insert("test1".to_string(), "0".to_string());
    expected.insert("test2".to_string(), "1".to_string());

    // act
    let table_constants = get_table_constants(db, table_options).await;

    let table_constant = table_constants.first().unwrap();

    // assert
    if let TableConstant::SimpleEnum(actual) = table_constant {
        println!("Actual: {:?}", actual.map);
        println!("Expected: {:?}", expected);
        let has_deep_equality = actual.map.len() == expected.len()
            && actual
                .map
                .clone()
                .into_iter()
                .all(|(k, v)| expected.contains_key(&k) && *expected.get(&k).unwrap() == v);
        assert!(has_deep_equality);
    };
}
