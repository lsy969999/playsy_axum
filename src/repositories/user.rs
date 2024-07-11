use sqlx::{postgres::PgQueryResult, types::chrono::Utc, PgConnection};
use super::entities::user::User;
use crate::configs::{consts::DB_CODE, errors::app_error::RepositoryLayerError};

pub async fn select_user_by_nick_name(
    conn: &mut PgConnection,
    nick_name: &str,
) -> Result<Option<User>, RepositoryLayerError> {
    Ok(
        sqlx::query_as!(
            User,
            r#"
                SELECT *
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
    login_ty_cd: &str,
) -> Result<Option<User>, RepositoryLayerError> {
    Ok(
        sqlx::query_as!(
            User,
            r#"
                SELECT *
                FROM tb_user tu
                WHERE tu.email = $1 AND tu.login_ty_cd = $2
            "#,
            email,
            login_ty_cd
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
) -> Result<PgQueryResult, RepositoryLayerError> {
    let now = Utc::now();
    Ok(
        sqlx::query!(
            r#"
                INSERT INTO tb_user
                (
                    nick_name, login_ty_cd, email, password, user_stt_cd, 
                    user_ty_cd, created_at, created_by, updated_at, updated_by
                )
                VALUES
                (
                    $1, $2, $3, $4, $5,
                    $6, $7, $8, $9, $10
                )
            "#,
            nick_name,
            DB_CODE.login_ty_cd.email,
            email,
            password,
            DB_CODE.user_stt_cd.ok,
            DB_CODE.user_ty_cd.user,
            now,
            1,
            now,
            1
        )
        .execute(conn)
        .await?
    )
}