use thiserror::Error as ThisError;

mod db;
mod todo;
pub use db::PostgresDatabase;

#[allow(clippy::enum_variant_names)]
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity Not Found _ {0}{1}")]
    EntityNotFound(&'static str, String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
