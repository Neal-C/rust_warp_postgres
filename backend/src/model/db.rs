use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type PostgresDatabase = Pool<Postgres>;

const POSTGRES_HOST: &str = "localhost";
const POSTGRES_ROOT_DATABASE: &str = "postgres";
const POSTGRES_ROOT_USER: &str = "postgres";
const POSTGRES_ROOT_PASSWORD: &str = "postgres";

pub async fn initialize_database() -> Result<PostgresDatabase, sqlx::Error> {
    new_database_pool(
        POSTGRES_HOST,
        POSTGRES_ROOT_DATABASE,
        POSTGRES_ROOT_USER,
        POSTGRES_ROOT_PASSWORD,
        1,
    )
    .await
}

async fn new_database_pool(
    host: &str,
    database: &str,
    user: &str,
    password: &str,
    max_connections: u32,
) -> Result<PostgresDatabase, sqlx::Error> {
    let connection_string = format!("postgres://{user}:{password}@{host}/{database}");

    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_millis(500))
        .connect(&connection_string)
        .await
}

#[cfg(test)]
#[path ="../_tests/model_db.rs"]
mod tests;
