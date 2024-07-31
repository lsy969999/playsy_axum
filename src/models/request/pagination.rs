use serde::Deserialize;

use crate::models::response::pagination::PaginationRes;

const DEFAULT_PAGE: i64 = 1;
const DEFAULT_LIMIT: i64 = 10;

#[derive(Debug, Deserialize)]
pub struct PaginationReq{
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub struct PaginationDbParam {
    pub page: i64,
    pub limit: i64,
    pub offset: i64,
}

impl PaginationReq {
    pub fn get_db_param(&self) -> PaginationDbParam {
        let page = self.page.unwrap_or(DEFAULT_PAGE);
        let limit = self.limit.unwrap_or(DEFAULT_LIMIT);
        let offset = (page - 1) * limit;
        PaginationDbParam {
            page, limit, offset,
        }
    }

    pub fn get_pagination_res(&self, total_cnt: i64) -> PaginationRes {
        PaginationRes { page: self.page.unwrap_or(DEFAULT_PAGE), limit: self.limit.unwrap_or(DEFAULT_LIMIT), total_cnt }
    }
}

// impl Default for PaginationReq {
//     fn default() -> Self {
//         Self {
//             page: Some(1),
//             limit: Some(10),
//         }
//     }
// }