use crate::constancerc::dto::{
    connection::Connection, query_execution_options::QueryExecutionOptions,
};
use mssql::Mssql;

pub mod mssql;

const MSSQL: &str = "mssql";

#[derive(Debug)]
pub enum Rdbms {
    Mssql(Mssql),
}

impl Default for Rdbms {
    fn default() -> Self {
        Rdbms::Mssql(Mssql::new(Connection::default()))
    }
}

impl Rdbms {
    pub fn to_rc_string(&self) -> String {
        let val = match self {
            Rdbms::Mssql(_) => MSSQL,
        };
        val.to_owned()
    }
    pub fn from_options(options: &QueryExecutionOptions) -> Self {
        let connection = options.connection.clone();
        let rdbms = &*options.rdbms;
        match rdbms {
            MSSQL => Rdbms::Mssql(Mssql::new(connection)),
            _ => panic!("Unrecognized rdbms: {}", rdbms),
        }
    }
}
