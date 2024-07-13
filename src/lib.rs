use std::{sync::Arc, time::Duration};
use bb8_redis::RedisConnectionManager;
use configs::{middlewares::test::test_log_and_modify, models::app_state::AppState, settings::SETTINGS};
use controller::routes::{auth::get_auth_router, home::get_home_router, openapi::get_openapi_route, user::get_user_router};
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::{limit::RequestBodyLimitLayer, services::ServeDir, timeout::TimeoutLayer, trace::TraceLayer};

pub mod utils;
pub mod configs;
pub mod services;
pub mod repositories;
pub mod controller;

// #[cfg(test)]
pub mod tests;

pub async fn play_sy_main() {
    configs::settings::load_settings().await;
    let settings = SETTINGS.get().unwrap();

    // tracing setting
    let file_appender = tracing_appender::rolling::daily("logs", "web.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "web=debug,sqlx=debug,tower_http=debug,axum=debug,axum::rejection=trace,bb8=debug,bb8-redis=debug,redis=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();
    debug!("settings: {:?}", settings);

    let db_pool = configs::db_config::init_db_pool(&settings.database_url).await;
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let redis_pool = bb8::Pool::builder().build(manager).await.unwrap();

    let app_state = Arc::new(
        AppState::new(
            db_pool, redis_pool,
        )
    );
    debug!("app_state: {:?}", app_state);

    let logging_middleware = ServiceBuilder::new()
        // 요청 로깅
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(test_log_and_modify))
        // 요청 바디 크기 제한 (1MB)
        .layer(RequestBodyLimitLayer::new(1024 * 1024))
        .layer(TimeoutLayer::new(Duration::from_secs(5)));

    let app = axum::Router::new()
        .nest_service("/static", ServeDir::new("./static"))
        .nest("/", get_openapi_route())
        .nest("/", get_home_router())
        .nest("/", get_auth_router())
        .nest("/", get_user_router())
        .layer(logging_middleware)
        .with_state(Arc::clone(&app_state));

    // reload
    // https://github.com/tokio-rs/axum/tree/main/examples/auto-reload
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            info!("reload bind_ip: {:?}", listener.local_addr());
            TcpListener::from_std(listener).unwrap()
        }
        None => {
            let bind_ip = format!("0.0.0.0:{}", settings.server_port);
            info!("bind_ip: {}", bind_ip);
            tokio::net::TcpListener::bind(bind_ip).await.unwrap()
        },
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}


/// 그레이스풀 셧다운
/// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to install signal handler")
        .recv()
        .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {
            println!("shoutdown ctrl_c")
        },
        _ = terminate => {
            println!("shoutdown terminate")
        },
    }
}