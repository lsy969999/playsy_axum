use chrono::Utc;
use sqlx::{postgres::PgQueryResult, PgConnection};
use crate::{configs::errors::app_error::RepositoryLayerError, models::{entities::inquery::Inquery, request::pagination::PaginationReq}};

pub async fn insert_inquery(
    conn: &mut PgConnection,
    user_sn: Option<i32>,
    email: Option<&str>,
    subject: Option<&str>,
    message: Option<&str>,
) -> Result<PgQueryResult, RepositoryLayerError> {
    let user_sn = user_sn.unwrap_or(1);
    let now = Utc::now();
    Ok(
        sqlx::query!(
            r#"
                INSERT INTO tb_inquery
                (user_sn, email, subject, message,  created_at, created_by, updated_at, updated_by)
                VALUES
                ($1, $2, $3, $4, $5, $6, $7 ,$8)
            "#,
            user_sn, email, subject, message,
            now, user_sn, now, user_sn
        )
        .execute(conn)
        .await?
    )
}

pub async fn select_inquery_by_sn(
    conn: &mut PgConnection,
    sn: i32,
    pagination: &PaginationReq
) -> Result<Vec<Inquery>, RepositoryLayerError> {
    let p = pagination.get_db_param();
    let inqs = sqlx::query_as!(
        Inquery,
        r#"
            SELECT 
                sn,
                user_sn,
                email,
                subject,
                message,
                answered_at,
                answer,
                created_at,
                created_by,
                updated_at,
                updated_by,
                is_deleted
            FROM tb_inquery
            WHERE user_sn = $1
            LIMIT $2
            OFFSET $3
        "#,
        sn,
        p.limit,
        p.offset,
    )
        .fetch_all(conn)
        .await?;
    Ok(inqs)
}

