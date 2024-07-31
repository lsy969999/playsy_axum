use sqlx::PgConnection;

use crate::{configs::errors::app_error::ServiceLayerError, repositories};

pub async fn inquery_insert(
    conn: &mut PgConnection,
    user_sn: Option<i32>,
    email: Option<&str>,
    subject: Option<&str>,
    message: Option<&str>,
) -> Result<(), ServiceLayerError> {
    repositories::inquery::insert_inquery(conn, user_sn, email, subject, message).await?;
    Ok(())
}