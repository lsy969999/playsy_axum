use sqlx::{Acquire, PgConnection, Postgres, Transaction};
use crate::configs::errors::app_error::RepositoryLayerError;

pub async fn begin(conn: &mut PgConnection)
    -> Result<Transaction<Postgres>, RepositoryLayerError > {
    Ok(conn.begin().await?)
}

pub async fn commit(tx: Transaction<'_, sqlx::Postgres>)
    -> Result<(), RepositoryLayerError> {
    Ok(tx.commit().await?)
}

pub async fn rollback(tx: Transaction<'_, sqlx::Postgres>)
    -> Result<(), RepositoryLayerError> {
    Ok(tx.rollback().await?)
}