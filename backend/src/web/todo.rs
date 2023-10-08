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

    // GET todo 'GET /todos/101
    let get = todos_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(todo_get);

    // CREATE todo 'POST /todos with body TodoPatch
    let create = todos_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json()) // ask warp to parse the body as JSON, and because PartialTodo derives Deserialize, warp will do the right thing and right deserialization will happen because of the todo_create signature
        .and_then(todo_create);

    // UPDATE todo 'PATCH /todos/100 with body PartialTodo
    let update = todos_path
        .and(warp::patch())
        .and(common.clone()) // 2 first arguments
        .and(warp::path::param()) // 3rd argument, the param
        .and(warp::body::json()) // 4th argument the body aka PartialTodo
        .and_then(todo_update); // function receives arguments in the order of the chaining

    // DELETE todo 'DELETE /todos/100
    let delete = todos_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(todo_delete);

    list.or(get).or(create).or(update).or(delete)
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

async fn todo_delete(
    database: Arc<PostgresDatabase>,
    user_ctx: UserContext,
    todo_id: i64,
) -> Result<WarpJSON, WarpRejection> {
    let todo = ModelAccessController::delete(&database, &user_ctx, todo_id).await?;

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
