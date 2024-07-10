use sqlx::{postgres::PgQueryResult, types::chrono::{DateTime, Utc}, PgConnection};

use crate::configs::errors::app_error::RepositoryLayerError;

use super::entities::refresh_token::RefreshToken;

/**
 * refresh_token 인서트
 */
pub async fn insert_refresh_token(
    conn: &mut PgConnection,
    user_sn: i32,
    hash: String,
    refresh_token: String,
    expires_at: DateTime<Utc>
) -> Result<PgQueryResult, RepositoryLayerError> {
    Ok(
        sqlx::query!(
            r#"
                INSERT INTO tb_refresh_token 
                    ( user_sn, hash, refresh_token, expires_at, created_by, updated_at, updated_by )
                VALUES
                    ( $1, $2, $3, $4, $5, $6, $7 )
            "#,
            user_sn,
            hash,
            refresh_token,
            expires_at,
            user_sn,
            Utc::now(),
            user_sn,
        )
        .execute(conn)
        .await?
    )
}

/**
 * refresh_token이 존재하는지
 * user_sn과 hash를 통해서 조회
 */
pub async fn select_refresh_token_by_user_sn_and_hash(
    conn: &mut PgConnection,
    user_sn: i32,
    hash: String
) -> Result<Option<RefreshToken>, RepositoryLayerError> {
    Ok(
        sqlx::query_as!(
            RefreshToken,
            r#"
                SELECT *
                FROM tb_refresh_token trt 
                WHERE 1 = 1
                    AND trt.user_sn     = $1
                    AND trt.hash        = $2
                    AND trt.is_deleted  = FALSE 
            "#,
            user_sn,
            hash,
        )
        .fetch_optional(conn)
        .await?
    )
}