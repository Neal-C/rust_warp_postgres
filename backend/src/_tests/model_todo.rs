use super::{ModelAccessController, PartialTodo};
use crate::model::{db::initialize_database, todo};

#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {
    // ARRANGE
    let database = initialize_database().await?;

    let data_fixture = PartialTodo {
        title: Some("test - model_todo_create 1".into()),
        ..PartialTodo::default()
    };

    // ACT
    let todo_created = ModelAccessController::create(&database, data_fixture.clone()).await?;

    assert!(todo_created.id >= 1000, "ID should be >= 1000");
    assert_eq!(data_fixture.title.unwrap(), todo_created.title);
    assert_eq!(todo::Status::Open, todo_created.status);

    Ok(())
}

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    // ARRANGE
    let database = initialize_database().await?;

    //ACT
    let result = ModelAccessController::list(&database).await?;

    //ASSERT
    assert_eq!(2, result.len(), "number of seed todos");

    println!("\n\n-->> {result:#?}");

    assert_eq!(101, result[0].id);
    assert_eq!(123, result[0].cid);
    assert_eq!("todo 101", result[0].title);
    // the other todo
    assert_eq!(101, result[0].id);
    assert_eq!(123, result[0].cid);
    assert_eq!("todo 100", result[0].title);

    Ok(())
}
