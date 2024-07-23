use sqlx::postgres::PgQueryResult;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgConnection;
use crate::configs::errors::app_error::RepositoryLayerError;
use crate::models::entities::email_join_verifications::EmailJoinVerifications;

pub async fn select_email_join_veri_for_code_dup_chk(
    conn: &mut PgConnection,
    code: &str,
) -> Result<Option<EmailJoinVerifications>, RepositoryLayerError> {
    Ok(
        sqlx::query_as!(
            EmailJoinVerifications,
            r#"
                SELECT 
                    tejv.sn ,
                    tejv.user_sn ,
                    tejv.code ,
                    tejv.is_verified ,
                    tejv.expires_at ,
                    tejv.created_at ,
                    tejv.created_by ,
                    tejv.updated_at ,
                    tejv.updated_by ,
                    tejv.is_deleted 
                FROM tb_email_join_verifications tejv
                WHERE 1 = 1
                    AND tejv.code = $1
                    AND tejv.is_deleted = FALSE
            "#,
            code
        )
        .fetch_optional(conn)
        .await?
    )
}

/// 
/// AND tejv.user_sn = $1  
/// AND tejv.expires_at < now()  
/// AND tejv.is_deleted = FALSE  
pub async fn select_email_join_veri_by_user_sn(
    conn: &mut PgConnection,
    user_sn: i32
) -> Result<Option<EmailJoinVerifications>, RepositoryLayerError> {
    Ok(
        sqlx::query_as!(
            EmailJoinVerifications,
            r#"
                SELECT 
                    tejv.sn ,
                    tejv.user_sn ,
                    tejv.code ,
                    tejv.is_verified ,
                    tejv.expires_at ,
                    tejv.created_at ,
                    tejv.created_by ,
                    tejv.updated_at ,
                    tejv.updated_by ,
                    tejv.is_deleted 
                FROM tb_email_join_verifications tejv
                WHERE 1 = 1
                    AND tejv.user_sn = $1
                    AND tejv.expires_at < now()
                    AND tejv.is_deleted = FALSE
            "#,
            user_sn
        )
        .fetch_optional(conn)
        .await?
    )
}

pub async fn insert_email_join_veri(
    conn: &mut PgConnection,
    user_sn: i32,
    code: &str,
    expires_at: DateTime<Utc>
) -> Result<PgQueryResult, RepositoryLayerError> {
    let now = Utc::now();
    Ok(
        sqlx::query!(
            r#"
                INSERT INTO tb_email_join_verifications 
                (
                    user_sn, code, expires_at,
                    created_at, created_by, updated_at, updated_by
                )
                VALUES
                (
                    $1, $2, $3,
                    $4, $5, $6, $7
                )
            "#,
            user_sn, code, expires_at,
            now, user_sn, now, user_sn
        )
        .execute(conn)
        .await?
    )
}

pub async fn update_email_join_veri_is_verified_to_true_by_sn(
    conn: &mut PgConnection,
    sn: i32
) -> Result<PgQueryResult, RepositoryLayerError> {
    Ok(
        sqlx::query!(
            r#"
                UPDATE tb_email_join_verifications
                SET
                    is_verified = FALSE,
                    updated_at = now(),
                    updated_by = $1
                WHERE sn = $1
            "#,
            sn
        )
        .execute(conn)
        .await?
    )
}