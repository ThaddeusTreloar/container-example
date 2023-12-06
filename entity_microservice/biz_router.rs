use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Router,
    routing::{delete, get, patch, post},
};
use shared::{
    prelude::*,
    state::entity::{Entity, PartialEntity}
};

use tracing::{error, info, span, Level};

pub fn get_router() -> Router<Arc<AppState<Entity>>> {
    Router::new()
        .route("/entity/:name", get(get_entity))
        .route("/entity/:name", delete(delete_entity))
        .route("/entity/:name", post(post_entity))
        .route("/entity/:name", patch(patch_entity))
}

async fn get_entity(
    Path(name): Path<String>,
    State(state): State<Arc<AppState<Entity>>>,
) -> Result<impl IntoResponse, StatusCode> {
    let span = span!(Level::INFO, "get_property", id = generate_trace_id());
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

async fn post_entity(
    Path(name): Path<String>,
    State(state): State<Arc<AppState<Entity>>>,
    Json(payload): Json<Entity>,
) -> Result<impl IntoResponse, StatusCode> {
    let span = span!(Level::INFO, "post_property", id = generate_trace_id());
    let _enter = span.enter();

    info!("req: payload={:?}", payload);

    state.set(name.as_str(), &payload).await;

    Ok(StatusCode::CREATED.into_response())
}

async fn patch_entity(
    Path(name): Path<String>,
    State(state): State<Arc<AppState<Entity>>>,
    Json(payload): Json<PartialEntity>,
) -> Result<impl IntoResponse, StatusCode> {
    let span = span!(Level::INFO, "patch_property", id = generate_trace_id());
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

async fn delete_entity(
    Path(name): Path<String>,
    State(state): State<Arc<AppState<Entity>>>,
) -> Result<impl IntoResponse, StatusCode> {
    let span = span!(Level::INFO, "delete_property", id = generate_trace_id());
    let _enter = span.enter();

    info!("req: name={}", name);

    state.rm(&name).await;

    Ok(StatusCode::NO_CONTENT.into_response())
}