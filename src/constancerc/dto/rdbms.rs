use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Rdbms {
    MSSQL,
    Postgres,
    Oracle,
    OleDb,
}

impl Default for Rdbms {
    fn default() -> Self {
        Rdbms::MSSQL
    }
}

impl Rdbms {
    pub fn to_string(&self) -> String {
        let val = match self {
            Rdbms::MSSQL => "mssql",
            Rdbms::Postgres => "postgres",
            Rdbms::Oracle => "oracle",
            Rdbms::OleDb => "oledb",
        };
        val.to_owned()
    }
    pub fn from_string(from: &str) -> Self {
        match from {
            "mssql" => Rdbms::MSSQL,
            "postgres" => Rdbms::Postgres,
            "oracle" => Rdbms::Oracle,
            "oledb" => Rdbms::OleDb,
            _ => panic!("Unrecognized rdbms: {}", from),
        }
    }
}
