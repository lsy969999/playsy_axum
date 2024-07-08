use sqlx::{postgres::PgQueryResult, types::chrono::Utc, PgConnection};
use crate::models::db::user::User;
use crate::models::db::code::DB_CODE;

#[derive(Debug, Clone)]
pub struct UserRepo;

impl UserRepo {
    pub fn new() -> Self {
        Self
    }
}

impl UserRepo {
    pub async fn select_user_by_nick_name(
        &self,
        conn: &mut PgConnection,
        nick_name: String,
    ) -> Result<Option<User>, sqlx::Error> {

        Ok(
            sqlx::query_as!(
                User,
                r#"
                    SELECT *
                    FROM tb_user
                    WHERE nick_name = $1
                "#, 
                nick_name
            )
            .fetch_optional(conn)
            .await?
        )
    }

    pub async fn insert_user(
        &self,
        conn: &mut PgConnection,
        nick_name: String,
        email: String,
        password: String,
    ) -> Result<PgQueryResult, sqlx::Error> {
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
}

