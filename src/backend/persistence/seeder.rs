use sqlx::{Sqlite, SqlitePool};
use uuid::Uuid;

use crate::backend::{
    domain::sample::Sample,
    persistence::{db::open_pool, error::PersistenceError},
};

pub struct SeedSample {}

pub const SEED_SAMPLES: &[SeedSample] = &[SeedSample {}, SeedSample {}];

pub async fn run_seeds(db_path: &str) -> Result<Vec<Uuid>, PersistenceError> {
    let pool = open_pool(db_path).await?;

    let mut created = Vec::with_capacity(SEED_SAMPLES.len());
    for seed in SEED_SAMPLES {
        let id = seed_pool(&pool, seed).await?;
        created.push(id);
    }

    pool.close().await;
    Ok(created)
}

async fn seed_pool(pool: &SqlitePool, seed: &SeedSample) -> Result<Uuid, PersistenceError> {
    let mut tx = pool.begin().await?;
    let id = seed_connection(&mut tx, seed).await?;
    tx.commit().await?;

    Ok(id)
}

async fn seed_connection(
    tx: &mut sqlx::Transaction<'static, Sqlite>,
    seed: &SeedSample,
) -> Result<Uuid, PersistenceError> {
    let sample = Sample::new(Uuid::new_v4());
    Ok(sample.id)
}
