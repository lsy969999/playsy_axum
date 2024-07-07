use std::sync::Arc;
use once_cell::sync::OnceCell;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::{limit::RequestBodyLimitLayer, services::ServeDir, trace::TraceLayer};
use web::{myconfig::middleward::test_log_and_modify, routes::{auth::get_auth_router, home::get_home_router, openapi::get_openapi_route, test::get_test_router}, shutdown_signal, AppState, JwtKeys, Settings};

static SETTINGS: OnceCell<Arc<Settings>> = OnceCell::new();

async fn load_settings() -> Arc<Settings> {
    SETTINGS.get_or_init(|| {
        let settings: Settings = Settings::new().unwrap(); //settings issue시 panic
        Arc::new(settings)
    }).clone()
}

#[tokio::main]
async fn main() {
    load_settings().await;
    let settings = SETTINGS.get().unwrap();

    // tracing setting
    let file_appender = tracing_appender::rolling::daily("logs", "web.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "web=debug,sqlx=debug,tower_http=debug,axum=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();
    debug!("settings: {:?}", settings);

    let dp_pool_max_connections_cnt = 5;
    let dp_pool_acquire_timeout_sec = std::time::Duration::from_secs(3);
    let db_pool = PgPoolOptions::new()
        .max_connections(dp_pool_max_connections_cnt)
        .acquire_timeout(dp_pool_acquire_timeout_sec)
        .connect(&settings.database_url)
        .await
        .expect("can't connect to database");
    let jwt_secret = (&settings.jwt_secret).to_string();
    let jwt_keys = JwtKeys::new(&jwt_secret);
    let app_state = Arc::new(AppState {db_pool, jwt_keys });
    debug!("app_state: {:?}", app_state);

    let logging_middleware = ServiceBuilder::new()
        // 요청 로깅
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(test_log_and_modify))
        // 요청 바디 크기 제한 (1MB)
        .layer(RequestBodyLimitLayer::new(1024 * 1024));

    let app = axum::Router::new()
        .nest_service("/static", ServeDir::new("./static"))
        .nest("/test", get_test_router())
        .nest("/", get_openapi_route())
        .nest("/", get_home_router())
        .nest("/", get_auth_router())
        .layer(logging_middleware)
        .with_state(Arc::clone(&app_state));

    let bind_ip = format!("0.0.0.0:{}", settings.server_port);
    let listener = tokio::net::TcpListener::bind(bind_ip)
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    info!("bind_ip: {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}