
// use axum_sqlite::*;

use axum::Extension;
use r2d2::{Pool, Error, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Clone)]
pub struct Database {
    pool: Pool<SqliteConnectionManager>,
}

impl Database {
    pub fn new(path: &str) -> Result<Extension<Self>, Error> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager)?;
        Ok(Extension(Self { pool }))
    }

    pub fn connection(
        &self,
    ) -> Result<PooledConnection<SqliteConnectionManager>, Error> {
        Ok(self.pool.get()?)
    }
}

pub fn init_db()->  Result<Extension<Database>, Error>{
    let path = ":memory:" ;
    return Database::new(path)
}