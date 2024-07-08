use crate::{extractors::{database_connection::DatabaseConnection, validated_form::ValidatedForm}, models::test::NameInput, myconfig::error::AppError};

pub async fn test_validate_handler(
    ValidatedForm(input): ValidatedForm<NameInput>
) -> Result<String, AppError> {
    Ok(format!("input name: {}", input.name))
}

pub async fn using_connection_pool_extractor2(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<String, AppError> {
    // return Err(AppError::TestError("asdf".to_string()).into());
    let aa = sqlx::query!("select 1 as haha")
    .fetch_all(&mut *conn)
    .await
    .map_err(|err|AppError::TestError(format!("testerror: {}", err)));

    sqlx::query_scalar("selecdt 'sample'")
        .fetch_one(&mut *conn)
        .await
        .map_err(|err|AppError::TestError(format!("testerror: {}", err)))
}

// async fn using_connection_pool_extractor(
//     State(s): State<Arc<AppState>>,
// ) -> Result<String, (StatusCode, String)> {
//     sqlx::query_scalar("select '1'")
//         .fetch_one(&s.db_pool)
//         .await
//         .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
// }