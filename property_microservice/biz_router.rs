use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Router,
    routing::{delete, get, patch, post},
};
use shared::{
    prelude::*,
    state::property::{Property, PartialProperty}, header_helper::get_logid
};

use tracing::{error, info, span, Level};

pub fn get_router() -> Router<Arc<AppState<Property>>> {
    Router::new()
        .route("/property/:name", get(get_property))
        .route("/property/:name", delete(delete_property))
        .route("/property/:name", post(post_property))
        .route("/property/:name", patch(patch_property))
}

async fn get_property(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<AppState<Property>>>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = get_logid(headers).await;
    let span = span!(Level::INFO, "get_property", id = id);
    let _enter = span.enter();

    info!("req: name={}", name);

    match state.get(&name).await {
        Some(value) => {
            info!("resp: value={}", value);
            Ok(Json(value))
        }
        None => {
            error!("resp: status={}", StatusCode::NOT_FOUND);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn post_property(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<AppState<Property>>>,
    Json(payload): Json<Property>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = get_logid(headers).await;
    let span = span!(Level::INFO, "post_property", id = id);
    let _enter = span.enter();

    info!("req: payload={:?}", payload);

    state.set(name.as_str(), &payload).await;

    Ok(StatusCode::CREATED.into_response())
}

async fn patch_property(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<AppState<Property>>>,
    Json(payload): Json<PartialProperty>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = get_logid(headers).await;
    let span = span!(Level::INFO, "patch_property", id = id);
    let _enter = span.enter();

    info!("req: payload={:?}", payload);

    match state.get(&name).await {
        Some(value) => {
            let merged = payload.merge(&value);
            state.set(name.as_str(), &merged).await;
            Ok(StatusCode::NO_CONTENT.into_response())
        }
        None => {
            error!("resp: status={}", StatusCode::NOT_FOUND);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn delete_property(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<AppState<Property>>>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = get_logid(headers).await;
    let span = span!(Level::INFO, "delete_property", id = id);
    let _enter = span.enter();

    info!("req: name={}", name);

    state.rm(&name).await;

    Ok(StatusCode::NO_CONTENT.into_response())
}