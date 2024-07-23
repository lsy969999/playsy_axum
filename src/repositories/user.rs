use sqlx::{postgres::PgQueryResult, types::chrono::Utc, PgConnection};
use super::{entities::user::User, enums::{user::ProviderTyEnum, user::UserSttEnum, user::UserTyEnum}};
use crate::{configs::errors::app_error::RepositoryLayerError, repositories::entities::sequence::Sequence};

pub async fn select_user_by_sn(
    conn: &mut PgConnection,
    sn: i32,
) -> Result<Option<User>, RepositoryLayerError> {
    Ok(
        sqlx::query_as!(
            User,
            r#"
                SELECT 
                    sn,
                    nick_name,
                    avatar_url,
                    email,
                    password,
                    user_stt_enum AS "user_stt_enum: UserSttEnum",
                    user_ty_enum AS "user_ty_enum: UserTyEnum",
                    provider_ty_enum AS "provider_ty_enum: ProviderTyEnum",
                    provider_id,
                    provider_access_token,
                    provider_refresh_token,
                    provider_etc,
                    created_at,
                    created_by,
                    updated_at,
                    updated_by,
                    is_deleted
                FROM tb_user tu
                WHERE tu.sn = $1 AND tu.is_deleted = FALSE
            "#, 
            sn
        )
        .fetch_optional(conn)
        .await?
    )
}

pub async fn select_user_by_nick_name(
    conn: &mut PgConnection,
    nick_name: &str,
) -> Result<Option<User>, RepositoryLayerError> {
    Ok(
        sqlx::query_as!(
            User,
            r#"
                SELECT 
                    sn,
                    nick_name,
                    avatar_url,
                    email,
                    password,
                    user_stt_enum AS "user_stt_enum: UserSttEnum",
                    user_ty_enum AS "user_ty_enum: UserTyEnum",
                    provider_ty_enum AS "provider_ty_enum: ProviderTyEnum",
                    provider_id,
                    provider_access_token,
                    provider_refresh_token,
                    provider_etc,
                    created_at,
                    created_by,
                    updated_at,
                    updated_by,
                    is_deleted
                FROM tb_user tu
                WHERE tu.nick_name = $1
            "#, 
            nick_name
        )
        .fetch_optional(conn)
        .await?
    )
}

pub async fn select_user_by_email_and_login_ty_cd(
    conn: &mut PgConnection,
    email: &str,
    provider_ty_enum: ProviderTyEnum,
) -> Result<Option<User>, RepositoryLayerError> {
    Ok(
        sqlx::query_as!(
            User,
            r#"
                SELECT 
                    sn,
                    nick_name,
                    avatar_url,
                    email,
                    password,
                    user_stt_enum AS "user_stt_enum: UserSttEnum",
                    user_ty_enum AS "user_ty_enum: UserTyEnum",
                    provider_ty_enum AS "provider_ty_enum: ProviderTyEnum",
                    provider_id,
                    provider_access_token,
                    provider_refresh_token,
                    provider_etc,
                    created_at,
                    created_by,
                    updated_at,
                    updated_by,
                    is_deleted
                FROM tb_user tu
                WHERE 1 = 1
                    AND tu.email = $1
                    AND tu.provider_ty_enum = $2
                    AND tu.is_deleted = FALSE
            "#,
            email,
            provider_ty_enum as ProviderTyEnum
        )
        .fetch_optional(conn)
        .await?
    )
}

pub async fn select_next_user_seq(conn: &mut PgConnection) -> Result<Sequence, RepositoryLayerError> {
    let sequence = sqlx::query_as!(
        Sequence,
        r#"
            SELECT nextval('tb_user_seq') AS "nextval!: i64"
        "#
    )
    .fetch_one(conn)
    .await?;
    Ok(sequence)
}

pub async fn insert_user(
    conn: &mut PgConnection,
    avatar_url: Option<&str>,
    user_sn: i32,
    nick_name: &str,
    email: Option<&str>,
    password: Option<&str>,
    provider_ty_enum: ProviderTyEnum,
    provider_id: &str,
    provider_access_token: Option<&str>,
    provider_refresh_token: Option<&str>,
    provider_etc: Option<serde_json::Value>,
    user_stt_enum: UserSttEnum,
) -> Result<PgQueryResult, RepositoryLayerError> {
    let now = Utc::now();
    Ok(
        sqlx::query!(
            r#"
                INSERT INTO tb_user
                (
                    sn, avatar_url, nick_name, email, password,
                    provider_ty_enum , provider_id, provider_access_token, provider_refresh_token, provider_etc,
                    user_stt_enum, user_ty_enum, created_at, created_by, updated_at, updated_by
                )
                VALUES
                (
                    $1, $2, $3, $4, $5,
                    $6, $7, $8, $9, $10,
                    $11, $12, $13, $14, $15, $16
                )
            "#,
            user_sn, avatar_url, nick_name, email, password,
            provider_ty_enum as ProviderTyEnum, provider_id, provider_access_token, provider_refresh_token, provider_etc,
            user_stt_enum as UserSttEnum, UserTyEnum::User as UserTyEnum, now, user_sn, now, user_sn
        )
        .execute(conn)
        .await?
    )
}

pub async fn update_user_stt_enum(
    conn: &mut PgConnection,
    user_sn: i32,
    user_stt_enum: UserSttEnum,
) -> Result<PgQueryResult, RepositoryLayerError> {
    Ok(
        sqlx::query!(
            r#"
                UPDATE tb_user
                SET
                    user_stt_enum = $1,
                    updated_at = now(),
                    updated_by = $2
                WHERE sn = $2
            "#,
            user_stt_enum as UserSttEnum,
            user_sn,
        )
        .execute(conn)
        .await?
    )
}

pub async fn delete_user_by_sn(
    conn: &mut PgConnection,
    user_sn: i32,
) -> Result<PgQueryResult, RepositoryLayerError> {
    Ok(
        sqlx::query!(
            r#"
                UPDATE tb_user
                SET
                    is_deleted = TRUE,
                    user_stt_enum = $1,
                    updated_at = now(),
                    updated_by = $2
                WHERE sn = $2
            "#,
            UserSttEnum::Quit as UserSttEnum,
            user_sn
        )
        .execute(conn)
        .await?
    )
}