use sqlx::PgConnection;
use validator::ValidationError;

pub trait AdditionalValidate {
    fn additional_db_validate(&self, conn: &mut PgConnection) -> impl std::future::Future<Output = anyhow::Result<Vec<ValidationError>>> + Send;
}