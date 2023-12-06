use axum::http::StatusCode;
use axum::{routing::get, response::IntoResponse};
use axum::Router;
use crate::trace::generate_trace_id;
use tracing::{span, Level, info};

pub fn get_router<T>() -> Router<T> 
where T: Clone + Send + Sync + 'static
{
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
}

pub async fn root() -> Result<impl IntoResponse, StatusCode> {
    let span = span!(
        Level::INFO,
        "root",
        id = generate_trace_id()
    );

    let _enter = span.enter();

    info!("Request from root directory");

    Ok("Hello, World!")
}

pub async fn health_check() -> Result<impl IntoResponse, StatusCode> {
    let span = span!(
        Level::INFO,
        "health_check",
        id = generate_trace_id()
    );

    let _enter = span.enter();

    info!("Request from health check");
    info!("Health check passed");

    Ok("OK")
}
