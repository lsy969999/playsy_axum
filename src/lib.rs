use std::{net::SocketAddr, time::Duration};
use axum::{middleware::{self}, response::IntoResponse, routing::get};
use axum_csrf::{CsrfConfig, Key};
use bb8_redis::RedisConnectionManager;
use configs::{app_config::APP_CONFIG, into_responses::{errors::ErrorTemplate, html_template::HtmlTemplate}, middlewares::etc::add_original_content_length, models::app_state::{AppState, ArcAppState}};
use controller::routes::{auth::get_auth_router, chat::get_chat_router, game::get_game_router, home::get_home_router, openapi::get_openapi_route, user::get_user_router };
use hyper::StatusCode;
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::{compression::CompressionLayer, limit::RequestBodyLimitLayer, services::ServeDir, timeout::TimeoutLayer, trace::TraceLayer};

pub mod utils;
pub mod configs;
pub mod services;
pub mod repositories;
pub mod controller;

// #[cfg(test)]
pub mod tests;

pub async fn play_sy_main() {
    configs::app_config::load_settings().await;
    let app_config = APP_CONFIG.get().unwrap();

    // tracing setting
    let file_appender = tracing_appender::rolling::daily("logs", "web.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "web=debug,sqlx=debug,tower_http=debug,axum=debug,axum::rejection=trace,bb8=debug,bb8-redis=debug,redis=debug,reqwest=debug,oauth2=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();
    debug!("app_config: {:?}", app_config);

    let db_pool = configs::db_config::init_db_pool(&app_config.settings.database.database_url).await;
    let redis_manager = RedisConnectionManager::new(format!("{}", &app_config.settings.redis.redis_url)).unwrap();
    let redis_pool = bb8::Pool::builder().build(redis_manager).await.unwrap();
    let csrf_key = Key::from(&app_config.settings.app.csrf_key.as_bytes());
    let csrf_config = CsrfConfig::default().with_key(Some(csrf_key));
    let app_state = AppState::new(db_pool, redis_pool, csrf_config);
    debug!("app_state: {:?}", app_state);
    let arc_app_state = ArcAppState::new(app_state);

    let app = axum::Router::new()
        .nest_service("/static", ServeDir::new("./static"))
        .route("/health", get(|| async { "OK" }))
        .nest("/", get_openapi_route())
        .nest("/", get_home_router(arc_app_state.clone()))
        .nest("/", get_auth_router())
        .nest("/", get_user_router(arc_app_state.clone()))
        .nest("/", get_game_router(arc_app_state.clone()))
        .nest("/", get_chat_router(arc_app_state.clone()))
        .with_state(arc_app_state.clone())
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(add_original_content_length))
                // 요청 로깅
                .layer(TraceLayer::new_for_http())
                // 요청 바디 크기 제한 (1MB)
                .layer(RequestBodyLimitLayer::new(1024 * 1024))
                .layer(TimeoutLayer::new(Duration::from_secs(5)))
                // 압축스
                // ios safari에서 gzip 사용하면 webkit error가 발생함;; 일단 nogzip으로 가자고
                .layer(CompressionLayer::new().no_gzip())
                
        );

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
            let bind_ip = format!("0.0.0.0:{}", app_config.settings.app.server_port);
            info!("bind_ip: {}", bind_ip);
            tokio::net::TcpListener::bind(bind_ip).await.unwrap()
        },
    };

    axum::serve(
            listener,
            app
                .fallback(global_404_handler)
                .into_make_service_with_connect_info::<SocketAddr>()
        )
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn global_404_handler() -> impl IntoResponse {
    HtmlTemplate(
        ErrorTemplate {
            error_code: StatusCode::NOT_FOUND.to_string(),
            error_message: format!("Page Not Found"),
        }
    )
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