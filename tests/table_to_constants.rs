use std::collections::HashMap;

use constance::{
    functions::get_database, functions::get_table_constants, testing_only::TableConstant,
    testing_only::ValueWithDescription,
};
use table_to_constants_util::{get_connection_options_from_env, mssql};

mod common;
mod table_to_constants_util;

#[tokio::test]
#[ignore = "requires test script for setup/teardown"]
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
    } else {
        assert!(false);
    }
}

#[tokio::test]
#[ignore = "requires test script for setup/teardown"]
pub async fn table_to_constants_mssql_string_enum() {
    // arrange
    let connection_options = get_connection_options_from_env();
    let rc = mssql::get_string_enum_rc(connection_options);
    let db = get_database(&rc);
    let table_options = &rc.table_options;
    let mut expected: HashMap<String, String> = HashMap::new();
    expected.insert("test1".to_string(), "test1Id".to_string());
    expected.insert("test2".to_string(), "test2Id".to_string());

    // act
    let table_constants = get_table_constants(db, table_options).await;

    let table_constant = table_constants.first().unwrap();

    // assert
    if let TableConstant::StringEnum(actual) = table_constant {
        let has_deep_equality = actual.map.len() == expected.len()
            && actual
                .map
                .clone()
                .into_iter()
                .all(|(k, v)| expected.contains_key(&k) && *expected.get(&k).unwrap() == v);
        assert!(has_deep_equality);
    } else {
        assert!(false);
    }
}

#[tokio::test]
#[ignore = "requires test script for setup/teardown"]
pub async fn table_to_constants_mssql_simple_enum_with_description() {
    // arrange
    let connection_options = get_connection_options_from_env();
    let rc = mssql::get_simple_enum_with_description_rc(connection_options);
    let db = get_database(&rc);
    let table_options = &rc.table_options;
    let mut expected: HashMap<String, ValueWithDescription> = HashMap::new();
    expected.insert(
        "test1".to_string(),
        ValueWithDescription {
            value: "0".to_string(),
            description: "test1description".to_string(),
        },
    );
    expected.insert(
        "test2".to_string(),
        ValueWithDescription {
            value: "1".to_string(),
            description: "test2description".to_string(),
        },
    );

    // act
    let table_constants = get_table_constants(db, table_options).await;

    let table_constant = table_constants.first().unwrap();

    // assert
    if let TableConstant::SimpleEnumWithDescription(actual) = table_constant {
        let has_deep_equality = actual.map.len() == expected.len()
            && actual
                .map
                .clone()
                .into_iter()
                .all(|(k, v)| expected.contains_key(&k) && *expected.get(&k).unwrap() == v);
        assert!(has_deep_equality);
    } else {
        assert!(false);
    }
}

#[tokio::test]
#[ignore = "requires test script for setup/teardown"]
pub async fn table_to_constants_mssql_string_enum_with_description() {
    // arrange
    let connection_options = get_connection_options_from_env();
    let rc = mssql::get_string_enum_with_description_rc(connection_options);
    let db = get_database(&rc);
    let table_options = &rc.table_options;
    let mut expected: HashMap<String, ValueWithDescription> = HashMap::new();
    expected.insert(
        "test1".to_string(),
        ValueWithDescription {
            value: "test1Id".to_string(),
            description: "test1description".to_string(),
        },
    );
    expected.insert(
        "test2".to_string(),
        ValueWithDescription {
            value: "test2Id".to_string(),
            description: "test2description".to_string(),
        },
    );

    // act
    let table_constants = get_table_constants(db, table_options).await;

    let table_constant = table_constants.first().unwrap();

    // assert
    if let TableConstant::StringEnumWithDescription(actual) = table_constant {
        let has_deep_equality = actual.map.len() == expected.len()
            && actual
                .map
                .clone()
                .into_iter()
                .all(|(k, v)| expected.contains_key(&k) && *expected.get(&k).unwrap() == v);
        assert!(has_deep_equality);
    } else {
        assert!(false);
    }
}

#[tokio::test]
#[ignore = "requires test script for setup/teardown"]
pub async fn table_to_constants_mssql_object_like_enum() {
    // arrange
    let connection_options = get_connection_options_from_env();
    let rc = mssql::get_object_like_enum_rc(connection_options);
    let db = get_database(&rc);
    let table_options = &rc.table_options;
    let mut expected: HashMap<String, Vec<(String, String)>> = HashMap::new();
    expected.insert(
        "test1".to_string(),
        vec![
            ("first".to_string(), "first1".to_string()),
            ("second".to_string(), "1".to_string()),
        ],
    );
    expected.insert(
        "test2".to_string(),
        vec![
            ("first".to_string(), "first2".to_string()),
            ("second".to_string(), "2".to_string()),
        ],
    );

    // act
    let table_constants = get_table_constants(db, table_options).await;

    let table_constant = table_constants.first().unwrap();

    // assert
    if let TableConstant::ObjectLike(actual) = table_constant {
        let has_deep_equality = actual.map.len() == expected.len()
            && actual
                .map
                .clone()
                .into_iter()
                .all(|(k, v)| expected.contains_key(&k) && *expected.get(&k).unwrap() == v);
        assert!(has_deep_equality);
    } else {
        assert!(false);
    }
}

#[tokio::test]
#[ignore = "requires test script for setup/teardown"]
pub async fn table_to_constants_mssql_object_like_enum_with_description() {
    // arrange
    let connection_options = get_connection_options_from_env();
    let rc = mssql::get_object_like_enum_with_description_rc(connection_options);
    let db = get_database(&rc);
    let table_options = &rc.table_options;
    let mut expected: HashMap<ValueWithDescription, Vec<(String, String)>> = HashMap::new();
    expected.insert(
        ValueWithDescription {
            value: "test1".to_string(),
            description: "test1description".to_string(),
        },
        vec![
            ("first".to_string(), "first1".to_string()),
            ("second".to_string(), "1".to_string()),
        ],
    );
    expected.insert(
        ValueWithDescription {
            value: "test2".to_string(),
            description: "test2description".to_string(),
        },
        vec![
            ("first".to_string(), "first2".to_string()),
            ("second".to_string(), "2".to_string()),
        ],
    );

    // act
    let table_constants = get_table_constants(db, table_options).await;

    let table_constant = table_constants.first().unwrap();

    // assert
    if let TableConstant::ObjectLikeWithDescription(actual) = table_constant {
        let has_deep_equality = actual.map.len() == expected.len()
            && actual
                .map
                .clone()
                .into_iter()
                .all(|(k, v)| expected.contains_key(&k) && *expected.get(&k).unwrap() == v);
        assert!(has_deep_equality);
    } else {
        assert!(false);
    }
}
