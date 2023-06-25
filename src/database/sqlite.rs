use std::{collections::HashMap, fs::File};

use chrono::{DateTime, Local, NaiveTime};
use sqlx::{
    sqlite::{SqlitePool, SqlitePoolOptions, SqliteRow},
    Column, Row, TypeInfo,
};

use super::model::ColType;

#[derive(Debug, Clone, Default)]
pub struct Sqlite {
    pub connection: Option<SqlitePool>,
    pub err: Option<(String, String)>,
}

impl Sqlite {
    #[tokio::main]
    pub async fn new(dbpath: &str) -> Self {
        match File::open(dbpath) {
            Err(_) => match File::create(dbpath) {
                Err(_) => {
                    return Self {
                        connection: None,
                        err: Some(("1".to_string(), "Error creating file".to_string())),
                    };
                }
                _ => {}
            },
            _ => {}
        }

        let opt_connection = SqlitePoolOptions::new().connect(dbpath).await;

        match opt_connection {
            Ok(connection) => {
                let query = "
                    CREATE TABLE
                        IF NOT EXISTS users(
                            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                            email VARCHAR(100) UNIQUE NOT NULL,
                            password VARCHAR(255) NOT NULL,
                            role VARCHAR(20)
                        );";

                let q = sqlx::query(query);
                let _ = q.execute(&connection).await.unwrap();

                Self {
                    connection: Some(connection),
                    err: None,
                }
            }
            Err(err) => {
                let code = err.as_database_error().unwrap().code().unwrap().to_string();
                let msg = err.as_database_error().unwrap().message().to_string();

                Self {
                    connection: None,
                    err: Some((code, msg)),
                }
            }
        }
    }

    pub async fn query_all(&self, query: &str, args: Vec<ColType>) -> Vec<SqliteRow> {
        let mut q = sqlx::query(query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::Real(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
                ColType::Bool(t) => q.bind(t),
                ColType::Date(t) => q.bind(t),
                ColType::Time(t) => q.bind(t),
                ColType::Datetime(t) => q.bind(t),
                _ => panic!("wrong type"),
            };
        }

        let conn = match &self.connection {
            Some(conn) => conn,
            None => panic!("query all: error while getting connection string"),
        };

        q.fetch_all(conn).await.unwrap()
    }

    pub async fn execute(&self, query: &str, args: Vec<ColType>) -> u64 {
        let mut q = sqlx::query(query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::Real(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
                ColType::Bool(t) => q.bind(t),
                ColType::Date(t) => q.bind(t),
                ColType::Time(t) => q.bind(t),
                ColType::Datetime(t) => q.bind(t),
                _ => panic!("wrong type"),
            };
        }

        let conn = match &self.connection {
            Some(conn) => conn,
            None => panic!("query all: error while getting connection string"),
        };

        let out = q.execute(conn).await.unwrap();

        out.rows_affected()
    }

    pub fn parse_all(&self, rows: Vec<SqliteRow>) -> Vec<HashMap<String, ColType>> {
        let mut table_data = vec![];

        for row in rows {
            dbg!(&row.columns());

            let mut map: HashMap<String, ColType> = HashMap::new();

            for i in 0..row.len() {
                let row_value = match row.column(i).type_info().name() {
                    "TEXT" | "VARCHAR" => {
                        let t = row.get::<Option<String>, _>(i);
                        ColType::String(t)
                    }
                    "INTEGER" => {
                        let t = row.get::<Option<i64>, _>(i);
                        ColType::Integer(t)
                    }
                    "REAL" | "NUMERIC" => {
                        let t = row.get::<Option<f64>, _>(i);
                        ColType::Real(t)
                    }
                    "BOOLEAN" => {
                        let t = row.get::<Option<bool>, _>(i);
                        ColType::Bool(t)
                    }
                    "DATETIME" => {
                        let t = row.get::<Option<DateTime<Local>>, _>(i);
                        ColType::Datetime(t)
                    }
                    "DATE" => {
                        let t = row.get::<Option<DateTime<Local>>, _>(i);
                        ColType::Date(t)
                    }
                    "TIME" => {
                        let t = row.get::<Option<NaiveTime>, _>(i);
                        ColType::Time(t)
                    }
                    _ => panic!("wrong type found!"),
                };

                map.insert(row.column(i).name().to_string(), row_value);
            }

            table_data.push(map);
        }

        table_data
    }
}
