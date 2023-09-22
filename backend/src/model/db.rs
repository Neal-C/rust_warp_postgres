use std::{fs, path::PathBuf, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type PostgresDatabase = Pool<Postgres>;

// Refactor to use .env variables
const POSTGRES_HOST: &str = "localhost";
const POSTGRES_ROOT_DATABASE: &str = "postgres";
const POSTGRES_ROOT_USER: &str = "postgres";
const POSTGRES_ROOT_PASSWORD: &str = "postgres";

// Refactor to use .env variables
const POSTGRES_APP_DATABASE: &str = "app";
const POSTGRES_APP_USER: &str = "app";
const POSTGRES_APP_PASSWORD: &str = "app";
const POSTGRES_APP_MAX_CONNECTIONS: u32 = 5;

// Refactor to use .env variables
const SQL_DIRECTORY: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";

pub async fn initialize_database() -> Result<PostgresDatabase, sqlx::Error> {
    // -- Create the database with POSTGRES_ROOT --> in development only
    {
        let root_db = new_database_pool(
            POSTGRES_HOST,
            POSTGRES_ROOT_DATABASE,
            POSTGRES_ROOT_USER,
            POSTGRES_ROOT_PASSWORD,
            1,
        )
        .await?;
        execute_sql_file(&root_db, SQL_RECREATE).await?;
    }

    let app_database = new_database_pool(
        POSTGRES_HOST,
        POSTGRES_APP_DATABASE,
        POSTGRES_APP_USER,
        POSTGRES_APP_PASSWORD,
        POSTGRES_APP_MAX_CONNECTIONS,
    )
    .await?;

    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIRECTORY)?
        .filter_map(|element| element.ok().map(|e| e.path()))
        .collect::<Vec<PathBuf>>();

    paths.sort();

    for path in paths {
        if let Some(path) = path.to_str() {
            if std::path::Path::new(path)
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("sql")) && path != SQL_RECREATE {
                execute_sql_file(&app_database, path).await?;
            }
        }
    }

    // returning the app db
    new_database_pool(
        POSTGRES_HOST,
        POSTGRES_APP_DATABASE,
        POSTGRES_APP_USER,
        POSTGRES_APP_PASSWORD,
        POSTGRES_APP_MAX_CONNECTIONS,
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

async fn execute_sql_file(database: &PostgresDatabase, file: &str) -> Result<(), sqlx::Error> {
    // Read the file
    let content = fs::read_to_string(file).map_err(|error| {
        println!("Error reading: {file}, (cause :{error:?}");
        error
    })?;

    // comments in the sql seed files will break this code
    let sqls_seed_files_statements: Vec<&str> = content.split(';').collect();

    for sql in sqls_seed_files_statements {
        match sqlx::query(sql).execute(database).await {
            Ok(_) => (),
            Err(error) => println!("Error seeding database, (cause :{error:?}",),
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;
