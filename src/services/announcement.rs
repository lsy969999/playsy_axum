use sqlx::PgConnection;

use crate::{configs::errors::app_error::ServiceLayerError, models::{entities::announcement::Announcement, request::pagination::PaginationReq, response::pagination::PaginationRes}, repositories};

pub async fn get_announcements(
    conn: &mut PgConnection,
    pagination: &PaginationReq
) -> Result<(Vec<Announcement>, PaginationRes), ServiceLayerError> {
    let mut tx = repositories::tx::begin(conn).await?;
    let tc = repositories::announcement::select_announcement_total_cnt(&mut tx).await?;
    let anns = repositories::announcement::select_announcement_by_pagination(&mut tx, pagination).await?;
    let p_res = pagination.get_pagination_res(tc.cnt);
    repositories::tx::commit(tx).await?;
    Ok((anns, p_res))
}

pub async fn get_announcement(conn: &mut PgConnection, announcement_sn: i32) -> Result<Announcement, ServiceLayerError> {
    let mut tx = repositories::tx::begin(conn).await?;
    let ann = repositories::announcement::select_announcement_by_sn(&mut tx, announcement_sn).await?;
    repositories::tx::commit(tx).await?;
    Ok(ann)
}