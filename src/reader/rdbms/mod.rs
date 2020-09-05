use crate::constancerc::dto::{
    connection::Connection, query_execution_options::QueryExecutionOptions,
};
use mssql::Mssql;
use ole_db::OleDb;
use oracle::Oracle;
use postgres::Postgres;

pub mod mssql;
pub mod ole_db;
pub mod oracle;
pub mod postgres;

const MSSQL: &str = "mssql";
const POSTGRES: &str = "postgres";
const ORACLE: &str = "oracle";
const OLEDB: &str = "oledb";

#[derive(Debug)]
pub enum Rdbms {
    Mssql(Mssql),
    Postgres(Postgres),
    Oracle(Oracle),
    OleDb(OleDb),
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
            Rdbms::Postgres(_) => POSTGRES,
            Rdbms::Oracle(_) => ORACLE,
            Rdbms::OleDb(_) => OLEDB,
        };
        val.to_owned()
    }
    pub fn from_options(options: &QueryExecutionOptions) -> Self {
        let connection = options.connection.clone();
        let rdbms = &*options.rdbms;
        match rdbms {
            MSSQL => Rdbms::Mssql(Mssql::new(connection)),
            POSTGRES => Rdbms::Postgres(Postgres {}),
            ORACLE => Rdbms::Oracle(Oracle {}),
            OLEDB => Rdbms::OleDb(OleDb {}),
            _ => panic!("Unrecognized rdbms: {}", rdbms),
        }
    }
}
