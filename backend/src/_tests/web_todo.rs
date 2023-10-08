use crate::web::{handle_rejection, HEADER_XAUTH};
use std::sync::Arc;
use warp::hyper;
use warp::hyper::body;
use warp::reply;
// use crate::security::user_from_token;
// use crate::web::handle_rejection;
use anyhow::{Context, Result as AnyhowResult};
use serde::Deserialize;
use serde_json::{from_str, from_value, json, Value};
use std::str::from_utf8;

use warp::Filter;

use crate::model::{initialize_database, ModelAccessController, Status, Todo};

use super::rest_filters;
#[tokio::test]
async fn web_todo_list() -> AnyhowResult<()> {
    // ARRANGE
    let database = initialize_database().await?;
    let database = Arc::new(database);

    let todo_apis = rest_filters("api", Arc::clone(&database)).recover(handle_rejection);

    // ACT

    let response = warp::test::request()
        .method("GET")
        .header(HEADER_XAUTH, "123.user_info_base64.signature_base64")
        .path("api/todos")
        .reply(&todo_apis)
        .await;

    // ASSERT

    assert_eq!(response.status(), 200, "https status");

    //extract the response data
    let todos: Vec<Todo> = extract_body_data(response)?;

    assert_eq!(todos.len(), 2, "number of todos");
    assert_eq!(todos[0].id, 101);
    assert_eq!(todos[0].title, "todos 101");
    assert_eq!(todos[0].status, Status::Open);

    Ok(())
}

// Web test utils

fn extract_body_data<Deserializable>(
    response: hyper::Response<body::Bytes>,
) -> AnyhowResult<Deserializable>
where
    for<'de> Deserializable: Deserialize<'de>,
{
    // parse the body as serde_json::Value
    let body = from_utf8(response.body())?;
    let mut body: Value = from_str(body)
        .with_context(|| format!("Cannot parse response body to JSON. response body '{body}'"))?;

    // extract the data
    let data = body["data"].take();

    // deserialize the data to Deserializable
    // that's why, we need the generic to have the Deserialize trait
    let data: Deserializable = from_value(data)?;

    Ok(data)
}
