// POSTGRES
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgPoolOptions, Postgres};

use std::env;

pub async fn db_connection() -> Result<PoolConnection<Postgres>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await?;

    // Get a connection from the pool.
    let connection: PoolConnection<Postgres> = pool.acquire().await?;

    Ok(connection)
}
