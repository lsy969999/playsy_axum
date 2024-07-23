use sqlx::{postgres::PgQueryResult, types::chrono::Utc, PgConnection};

use crate::{configs::errors::app_error::RepositoryLayerError, models::{entities::{refresh_token::{RefreshToken, RefreshTokenUser}, sequence::Sequence}, fn_args::repo::InsertRefreshTokenArgs}};


pub async fn select_next_refresh_token_seq(conn: &mut PgConnection) -> Result<Sequence, RepositoryLayerError> {
    let sequence = sqlx::query_as!(
        Sequence,
        r#"
            SELECT nextval('tb_refresh_token_seq')  AS "nextval!: i64"
        "#
    )
    .fetch_one(conn)
    .await?;
    Ok(sequence)
}

/**
 * refresh_token 인서트
 */
pub async fn insert_refresh_token<'a>(
    conn: &mut PgConnection,
    args: InsertRefreshTokenArgs<'a>,
) -> Result<PgQueryResult, RepositoryLayerError> {
    Ok(
        sqlx::query!(
            r#"
                INSERT INTO tb_refresh_token 
                    ( sn, user_sn, hash, refresh_token, expires_at, forwarded_ip, client_ip, user_agent, created_by, updated_at, updated_by )
                VALUES
                    ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11 )
            "#,
            args.sn,
            args.user_sn,
            args.hash,
            args.refresh_token,
            args.expires_at,
            args.forwarded_id,
            args.addr,
            args.user_agent,
            args.user_sn,
            Utc::now(),
            args.user_sn,
        )
        .execute(conn)
        .await?
    )
}

pub async fn select_refresh_token_user_by_sn(
    conn: &mut PgConnection,
    sn: i32,
) -> Result<Option<RefreshTokenUser>, RepositoryLayerError> {
    Ok(
        sqlx::query_as!(
            RefreshTokenUser,
            r#"
                SELECT 
                    trt.sn AS refresh_token_sn,
                    tu.sn AS user_sn,
                    tu.avatar_url,
                    tu.nick_name 
                FROM tb_refresh_token trt 
                    INNER JOIN tb_user tu ON tu.sn = trt.user_sn
                WHERE 1=1
                    AND	trt.sn = $1
                    AND trt.is_deleted = FALSE 
            "#,
            sn
        )
        .fetch_optional(conn)
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