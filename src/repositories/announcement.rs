use sqlx::PgConnection;

use crate::{configs::errors::app_error::RepositoryLayerError, models::{entities::{announcement::Announcement, total_count::TotalCount}, request::pagination::PaginationReq}};

pub async fn select_announcement_by_pagination(
    conn: &mut PgConnection,
    pagination: &PaginationReq
) -> Result<Vec<Announcement>, RepositoryLayerError> {
    let pd = pagination.get_db_param();
    let limit = pd.limit;
    let offset = pd.offset;
    let anns = sqlx::query_as!(
        Announcement,
        r#"
            SELECT 
                ta.sn,
                ta.user_sn ,
                ta.title ,
                ta.content,
                ta.created_at ,
                ta.created_by ,
                ta.updated_at ,
                ta.updated_by ,
                ta.is_deleted 
            FROM tb_announcement ta
            WHERE ta.is_deleted = FALSE
            ORDER BY created_at DESC
            LIMIT $1
            OFFSET $2
        "#,
        limit,
        offset,
    )
    .fetch_all(conn)
    .await?;
    Ok(anns)
}

pub async fn select_announcement_total_cnt(
    conn: &mut PgConnection
) -> Result<TotalCount, RepositoryLayerError> {
    let total_count = sqlx::query_as!(
        TotalCount,
        r#"
            SELECT COUNT(*) as "cnt!: i64" FROM tb_announcement
        "#
    )
    .fetch_one(conn)
    .await?;
    Ok(total_count)
}

pub async fn select_announcement_by_sn(
    conn: &mut PgConnection,
    sn: i32
) -> Result<Announcement, RepositoryLayerError> {
    let ann = sqlx::query_as!(
        Announcement,
        r#"
            SELECT 
                ta.sn,
                ta.user_sn ,
                ta.title ,
                ta.content,
                ta.created_at ,
                ta.created_by ,
                ta.updated_at ,
                ta.updated_by ,
                ta.is_deleted 
            FROM tb_announcement ta
            WHERE ta.sn = $1 AND ta.is_deleted = FALSE
        "#,
        sn
    )
    .fetch_one(conn)
    .await?;
    Ok(ann)
}