use super::{ModelAccessController, PartialTodo};
use crate::{
    model::{db::initialize_database, todo},
    security::{user_context_from_token, UserContext},
};

#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {
    // ARRANGE
    let database = initialize_database().await?;

    let data_fixture = PartialTodo {
        title: Some(String::from("test - model_todo_create 1")),
        ..PartialTodo::default()
    };

    let user_context = user_context_from_token("123").await?;

    // ACT
    let todo_created =
        ModelAccessController::create(&database, &user_context, data_fixture.clone()).await?;

    assert!(todo_created.id >= 1000, "ID should be >= 1000");
    assert_eq!(data_fixture.title.unwrap(), todo_created.title);
    assert_eq!(todo_created.status, todo::Status::Open);

    Ok(())
}

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    // ARRANGE
    let database = initialize_database().await?;

    let user_context = user_context_from_token("123").await?;

    //ACT
    let result = ModelAccessController::list(&database, &user_context).await?;

    //ASSERT
    assert_eq!(2, result.len(), "number of seed todos");

    println!("\n\n-->> {result:#?}");

    assert_eq!(result[0].id, 101);
    assert_eq!(result[0].cid, 123);
    assert_eq!(result[0].title, "todo 101");
    // the other todo
    assert_eq!(result[0].id, 101);
    assert_eq!(result[0].cid, 123);
    assert_eq!(result[0].title, "todo 100");

    Ok(())
}

#[tokio::test]
async fn model_todo_get() -> Result<(), Box<dyn std::error::Error>> {
    // ARRANGE
    let database = initialize_database().await?;

    let user_context = user_context_from_token("123").await?;

    // ACT
    let todo = ModelAccessController::get(&database, &user_context, 100).await?;

    // ASSERT
    assert_eq!(todo.id, 100);
    assert_eq!(todo.title, "todo 100");
    assert_eq!(todo.status, todo::Status::Closed);
    Ok(())
}

#[tokio::test]
async fn model_todo_update_ok() -> Result<(), Box<dyn std::error::Error>> {
    // ARRANGE
    let database = initialize_database().await?;

    let data_fixture = PartialTodo {
        title: Some(String::from("test - model_todo_create 1")),
        ..PartialTodo::default()
    };

    let user_context = user_context_from_token("123").await?;

    let todo_created =
        ModelAccessController::create(&database, &user_context, data_fixture.clone()).await?;

    assert!(todo_created.id >= 1000, "ID should be >= 1000");
    assert_eq!(data_fixture.title.unwrap(), todo_created.title);
    assert_eq!(todo_created.status, todo::Status::Open);

    let update_data_fixture = PartialTodo {
        title: Some(String::from("test - model_todo_update_ok")),
        ..PartialTodo::default()
    };
    // ACT

    let todo_updated = ModelAccessController::update(
        &database,
        &user_context,
        todo_created.id,
        update_data_fixture.clone(),
    )
    .await?;

    let list_result = ModelAccessController::list(&database, &user_context).await?;

    assert_eq!(list_result.len(), 3);

    Ok(())
}
