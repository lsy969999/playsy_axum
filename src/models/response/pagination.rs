#[derive(Debug)]
pub struct PaginationRes {
    pub page: i64,
    pub limit: i64,
    pub total_cnt: i64,
}