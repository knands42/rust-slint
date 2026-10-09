use thiserror::Error;

#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("could not prepare database directory {path}: {source}")]
    PrepareDirectory {
        path: String,
        #[source]
        source: std::io::Error,
    },
}
