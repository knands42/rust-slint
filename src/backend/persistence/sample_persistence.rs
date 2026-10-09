use sqlx::SqliteConnection;

use crate::backend::{domain::sample::Sample, persistence::error::PersistenceError};

pub async fn insert_sample(
    conn: &mut SqliteConnection,
    sample: &Sample,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "INSERT INTO sample (id)
         VALUES (?1)",
    )
    .bind(sample.id.to_string())
    .execute(conn)
    .await?;

    Ok(())
}
