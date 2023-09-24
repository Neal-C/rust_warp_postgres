use std::default;

use crate::model;
use crate::model::db::PostgresDatabase;

#[derive(sqlx::FromRow, Debug, Clone, Default)]
pub struct Todo {
    pub id: i64,
    pub cid: i64,
    pub title: String,
    pub status: Status,
}

#[derive(sqlx::Type, Default, Debug, Clone)]
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
        data: PartialTodo,
    ) -> Result<Todo, model::Error> {
        let sql = "INSERT INTO todo (cid, title) VALUES ($1, $2) RETURNING id, cid, title";

        let query = sqlx::query_as::<_, Todo>(sql)
            .bind(123_i64)
            .bind(data.title.unwrap_or_else(|| "untitled".into()));

        let todo = query.fetch_one(database).await?;

        Ok(todo)
    }

    pub async fn list(database: &PostgresDatabase) -> Result<Vec<Todo>, model::Error> {
        let sql_statement = "SELECT id, cid, title FROM todo ORDER BY id DESC";

        let query = sqlx::query_as::<_, Todo>(sql_statement);

        let todos = query.fetch_all(database).await?;

        Ok(todos)
    }
}

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
