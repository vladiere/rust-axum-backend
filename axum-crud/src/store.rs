use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{
    config::core_config,
    error::{Error, Result},
};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    let db_url = format!(
        "postgres://{}:{}@{}/{}",
        &core_config().DB_USER,
        &core_config().DB_PASS,
        &core_config().DB_HOST,
        &core_config().DB_NAME
    );

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|ex| Error::FailToConnectPool(ex.to_string()))
}
