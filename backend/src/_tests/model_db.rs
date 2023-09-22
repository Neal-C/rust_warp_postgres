use super::initialize_database;
#[tokio::test]
async fn model_database_initialize_database() -> Result<(), Box<dyn std::error::Error>> {
    // ARRANGE
    let database = initialize_database().await?;

    //ACT
    let result = sqlx::query("SELECT * FROM todo")
        .fetch_all(&database)
        .await?;

    assert_eq!(2, result.len(), "number of seed todos");

    Ok(())
}
