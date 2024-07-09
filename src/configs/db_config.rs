use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async  fn init_db_pool(
    database_url: &str
) -> Pool<Postgres> {
    let dp_pool_max_connections_cnt = 5;
    let dp_pool_acquire_timeout_sec = std::time::Duration::from_secs(3);

    PgPoolOptions::new()
        .max_connections(dp_pool_max_connections_cnt)
        .acquire_timeout(dp_pool_acquire_timeout_sec)
        .connect(database_url)
        .await
        .expect("can't connect to database")
}