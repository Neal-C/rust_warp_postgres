use super::ModelAccessController;
use crate::model::db::initialize_database;

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
