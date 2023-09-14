use super::initialize_database;
#[tokio::test]
async fn model_database_initialize_database() -> Result<(), Box<dyn std::error::Error>> {
    initialize_database().await;

    Ok(())
}
