use mssql::Mssql;
use ole_db::OleDb;
use oracle::Oracle;
use postgres::Postgres;

pub mod mssql;
pub mod ole_db;
pub mod oracle;
pub mod postgres;

#[derive(Debug)]
pub enum Rdbms {
    Mssql(Mssql),
    Postgres(Postgres),
    Oracle(Oracle),
    OleDb(OleDb),
}

impl Default for Rdbms {
    fn default() -> Self {
        Rdbms::Mssql(Mssql {})
    }
}

impl Rdbms {
    pub fn to_rc_string(&self) -> String {
        let val = match self {
            Rdbms::Mssql(_) => "mssql",
            Rdbms::Postgres(_) => "postgres",
            Rdbms::Oracle(_) => "oracle",
            Rdbms::OleDb(_) => "oledb",
        };
        val.to_owned()
    }
    pub fn from_string(from: &str) -> Self {
        match from {
            "mssql" => Rdbms::Mssql(Mssql {}),
            "postgres" => Rdbms::Postgres(Postgres {}),
            "oracle" => Rdbms::Oracle(Oracle {}),
            "oledb" => Rdbms::OleDb(OleDb {}),
            _ => panic!("Unrecognized rdbms: {}", from),
        }
    }
}
