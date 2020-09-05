use super::super::constancerc::dto::table_option::TableOption;
use super::table_constant::TableConstant;
use crate::reader::rdbms::Rdbms;
use futures::future::join_all;

pub async fn get_table_constants(db: Rdbms, table_options: &[TableOption]) -> Vec<TableConstant> {
    join_all(
        table_options
            .iter()
            .map(|option| TableConstant::from_option(option, &db)),
    )
    .await
}
