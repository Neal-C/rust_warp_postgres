use thiserror::Error as ThisError;

use crate::model::PostgresDatabase;

pub struct UserContext {
    pub user_id: i64,
}

pub async fn user_context_from_token(
    _database: &PostgresDatabase,
    user_token: &str,
) -> Result<UserContext, Error> {
    // TODO : real validation needed
    // fetch user informations from database
    match user_token.parse::<i64>() {
        Ok(value) => Ok(UserContext { user_id: value }),
        Err(_) => Err(Error::InvalidToken(String::from(user_token))),
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Invalid Token {0}")]
    InvalidToken(String),
}
