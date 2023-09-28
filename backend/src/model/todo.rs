use std::default;

use crate::model;
use crate::model::db::PostgresDatabase;
use crate::security::UserContext;

#[derive(sqlx::FromRow, Debug, Clone, Default)]
pub struct Todo {
    pub id: i64,
    pub cid: i64,
    pub title: String,
    pub status: Status,
}

// we need the sqlx macro to map the database enum type to that struct
// it needs to be the same name than in the sql file
// The Rust's side of enum must be Uppercase
// Since the schema uses lowercase, we're using another sqlx macro for that conversion
#[derive(sqlx::Type, Default, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "todo_status")]
#[sqlx(rename_all = "lowercase")]
pub enum Status {
    Open,
    #[default]
    Closed,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Default, Clone)]
pub struct PartialTodo {
    pub cid: Option<i64>,
    pub title: Option<String>,
    pub status: Option<Status>,
}

pub struct ModelAccessController;
impl ModelAccessController {
    pub async fn create(
        database: &PostgresDatabase,
        utx: &UserContext,
        data: PartialTodo,
    ) -> Result<Todo, model::Error> {
        let sql = "INSERT INTO todo (cid, title) VALUES ($1, $2) RETURNING id, cid, title, status";

        let query = sqlx::query_as::<_, Todo>(sql)
            .bind(123_i64) // FIXME : should come from user context
            .bind(data.title.unwrap_or_else(|| "untitled".into()));

        let todo = query.fetch_one(database).await?;

        Ok(todo)
    }

    pub async fn list(
        database: &PostgresDatabase,
        _utx: &UserContext,
    ) -> Result<Vec<Todo>, model::Error> {
        let sql_statement = "SELECT id, cid, title, status FROM todo ORDER BY id DESC";

        let query = sqlx::query_as::<_, Todo>(sql_statement);

        let todos = query.fetch_all(database).await?;

        Ok(todos)
    }

    pub async fn get(
        database: &PostgresDatabase,
        _utx: &UserContext,
        id: i64,
    ) -> Result<Todo, model::Error> {
        let sql_statement = "SELECT id, cid, title, status FROM todo WHERE id = $1";

        let sql_query = sqlx::query_as::<_, Todo>(sql_statement);

        let todo = sql_query.fetch_one(database).await;

        handle_fetch_one_result(todo, id)
    }

    pub async fn update(
        database: &PostgresDatabase,
        utx: &UserContext,
        id: i64,
        data: PartialTodo,
    ) -> Result<Todo, model::Error> {
        let sql_statement =
            "UPDATE todo SET (title, status) = ($2, $3) WHERE id = $1 RETURNING id, title, status";

        let sql_query = sqlx::query_as::<_, Todo>(sql_statement);

        let todo = sql_query.fetch_one(database).await;

        handle_fetch_one_result(todo, id)
    }

    pub async fn delete(
        database: &PostgresDatabase,
        _utx: &UserContext,
        id: i64,
    ) -> Result<Todo, model::Error> {
        let sql_statement = "DELETE FROM todo WHERE id = $1 RETURNING *";

        let sql_query = sqlx::query_as::<_, Todo>(sql_statement);

        let todo = sql_query.fetch_one(database).await;

        handle_fetch_one_result(todo, id)
    }
}

// Utils

fn handle_fetch_one_result(
    result: Result<Todo, sqlx::Error>,
    id: i64,
) -> Result<Todo, model::Error> {
    result.map_err(|sqlx_error| match sqlx_error {
        sqlx::Error::RowNotFound => model::Error::EntityNotFound("todo", id.to_string()),
        other => model::Error::SqlxError(other),
    })
}

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
