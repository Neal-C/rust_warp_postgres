use std::{convert::Infallible, sync::Arc};

use warp::{reject::Rejection, reply::Json, Filter};

use crate::{
    model::{self, PostgresDatabase},
    security::{user_context_from_token, UserContext},
};

use super::filter_utils::{do_auth, with_db};

pub fn rest_filters(
    base_path: &'static str,
    database: Arc<model::PostgresDatabase>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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
) -> Result<Json, warp::Rejection> {
    let todos = model::ModelAccessController::list(&database, &utx)
        .await
        .unwrap();

    let response = serde_json::json!({
        "data": todos
    });
    Ok(warp::reply::json(&response))
}

#[cfg(test)]
#[path = "../_tests/web_todo.rs"]
mod tests;
