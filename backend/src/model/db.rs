use std::{fs, time::Duration};

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

async fn seed_database(database: &PostgresDatabase, file: &str) -> Result<(), sqlx::Error> {
    // Read the file
    let content = fs::read_to_string(file).map_err(|error| {
        println!("Error reading: {}, (cause :{:?}", file, error);
    });

    // comments in the sql seed files will break this code
    let sqls_seed_files_statements: Vec<&str> = content.split(";").collect();

    for sql in sqls_seed_files_statements {
        sqlx::query(&sql).execute(database).await
    }

    Ok(())
}

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;
