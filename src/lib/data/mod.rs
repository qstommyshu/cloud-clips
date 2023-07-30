use derive_more::{ Display, From };
use serde::{ Deserialize, Serialize };
use sqlx::Sqlite;
use uuid::Uuid;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")] Database(#[from] sqlx::Error),
}

// These type aliases are similar to interface, we can easily modify the
// aliases when we want to move to a new DB in the future;
pub type AppDatabase = Database<Sqlite>;
// Pool of agents
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect(connection_str).await;
        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{}\n", e);
                eprintln!(
                    "If the database has not yet been created, run :\n
                    $sqlx database setup\n"
                );
                panic!("database connection error");
            }
        }
    }
}

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    // TODO: explain what does this nil() does?
    // https://www.bilibili.com/video/BV1tm4y1Z7HX/?p=186&spm_id_from=pageDriver&vd_source=412d07365b85aaf0da45845a65078eae
    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    // uuid::Error is a enum of ClipError
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(id)?))
    }
}
