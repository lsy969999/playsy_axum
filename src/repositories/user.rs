use sqlx::{postgres::PgQueryResult, types::chrono::Utc, PgConnection};
use super::{entities::user::User, enums::{user::ProviderTyEnum, user::UserSttEnum, user::UserTyEnum}};
use crate::configs::errors::app_error::RepositoryLayerError;

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
                    avatar_url,
                    nick_name,
                    email,
                    password,
                    provider_id,
                    provider_secret,
                    provider_ty_enum AS "provider_ty_enum: ProviderTyEnum",
                    user_stt_enum AS "user_stt_enum: UserSttEnum",
                    user_ty_enum AS "user_ty_enum: UserTyEnum",
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
                    avatar_url,
                    nick_name,
                    email,
                    password,
                    provider_id,
                    provider_secret,
                    provider_ty_enum AS "provider_ty_enum: ProviderTyEnum",
                    user_stt_enum AS "user_stt_enum: UserSttEnum",
                    user_ty_enum AS "user_ty_enum: UserTyEnum",
                    created_at,
                    created_by,
                    updated_at,
                    updated_by,
                    is_deleted
                FROM tb_user tu
                WHERE tu.email = $1 AND tu.provider_ty_enum = $2
            "#,
            email,
            provider_ty_enum as ProviderTyEnum
        )
        .fetch_optional(conn)
        .await?
    )
}

pub async fn insert_user(
    conn: &mut PgConnection,
    nick_name: &str,
    email: &str,
    password: &str,
    provider_ty_enum: ProviderTyEnum,
    user_stt_enum: UserSttEnum,
) -> Result<PgQueryResult, RepositoryLayerError> {
    let now = Utc::now();
    Ok(
        sqlx::query!(
            r#"
                INSERT INTO tb_user
                (
                    nick_name, email, password,
                    provider_ty_enum , user_stt_enum, user_ty_enum,
                    created_at, created_by, updated_at, updated_by
                )
                VALUES
                (
                    $1, $2, $3,
                    $4, $5, $6,
                    $7, $8, $9, $10
                )
            "#,
            nick_name, email, password,
            provider_ty_enum as ProviderTyEnum,
                user_stt_enum as UserSttEnum,
                    UserTyEnum::User as UserTyEnum,
            now, 1, now, 1
        )
        .execute(conn)
        .await?
    )
}