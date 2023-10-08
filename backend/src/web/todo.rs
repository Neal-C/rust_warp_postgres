use std::{convert::Infallible, sync::Arc};

use serde::Serialize;
use warp::{reject::Rejection as WarpRejection, reply::Json as WarpJSON, Filter};

use crate::{
    model::{self, ModelAccessController, PartialTodo, PostgresDatabase},
    security::{user_context_from_token, UserContext},
};

use super::filter_utils::{do_auth, with_db};

pub fn rest_filters(
    base_path: &'static str,
    database: Arc<model::PostgresDatabase>,
) -> impl Filter<Extract = impl warp::Reply, Error = WarpRejection> + Clone {
    let todos_path = warp::path(base_path).and(warp::path("todos")); // base_path = api/v1 and todos -> api/v1/todos

    let common = with_db(Arc::clone(&database)).and(do_auth(Arc::clone(&database)));

    // LIST todos 'GET todos/'
    let list = todos_path
        .and(warp::get())
        .and(warp::path::end()) // must end there to be GET todos/ because if there's more, then it will be GET todos/1
        .and(common.clone())
        .and_then(todo_list);

    list
}

// because common extracts the PostgresDatabase clone and the utx, it will be provided to the function in that order
async fn todo_list(
    database: Arc<PostgresDatabase>,
    utx: UserContext,
) -> Result<WarpJSON, warp::Rejection> {
    // Arc implements the AsRef trait, so Arc<PostgresDatabase> can be given to expecting &PostgresDatabase
    let todos = model::ModelAccessController::list(&database, &utx).await?;

    let response = serialize_to_warpjson(todos);
    Ok(response)
}

async fn todo_get(
    database: Arc<PostgresDatabase>,
    user_ctx: UserContext,
    id: i64,
) -> Result<WarpJSON, WarpRejection> {
    let todo = ModelAccessController::get(&database, &user_ctx, id).await?;
    let response = serialize_to_warpjson(todo);
    Ok(response)
}
async fn todo_create(
    database: Arc<PostgresDatabase>,
    user_ctx: UserContext,
    patch_data: PartialTodo,
) -> Result<WarpJSON, WarpRejection> {
    let todo = ModelAccessController::create(&database, &user_ctx, patch_data).await?;
    let response = serialize_to_warpjson(todo);
    Ok(response)
}

async fn todo_update(
    database: Arc<PostgresDatabase>,
    user_ctx: UserContext,
    todo_id: i64,
    patch_data: PartialTodo,
) -> Result<WarpJSON, WarpRejection> {
    let todo = ModelAccessController::update(&database, &user_ctx, todo_id, patch_data).await?;

    let response = serialize_to_warpjson(todo);

    Ok(response)
}

fn serialize_to_warpjson<S: Serialize>(data: S) -> WarpJSON {
    let response = serde_json::json!({"data": data});
    warp::reply::json(&response)
}

#[cfg(test)]
#[path = "../_tests/web_todo.rs"]
mod tests;
