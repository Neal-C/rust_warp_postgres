use crate::model::db::PostgresDatabase;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64,
    pub title: String,
}

pub struct ModelAccessController;

impl ModelAccessController {
    pub async fn list(database: &PostgresDatabase) -> Result<Vec<Todo>, sqlx::Error> {
        let sql_statement = "SELECT id, cid, title FROM todo ORDER BY id DESC";

        let query = sqlx::query_as::<_, Todo>(sql_statement);

        let todos = query.fetch_all(database).await?;

        Ok(todos)
    }
}

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
