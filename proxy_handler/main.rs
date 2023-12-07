use core::panic;
use std::{sync::Arc, time::Duration};

use anyhow::Error;
use axum::{routing::{get, post, patch, delete}, Router, response::{IntoResponse, Response}, http::{StatusCode, HeaderValue}, extract::{Request, State}, Json, body::{Body, to_bytes}};
use hyper::header;
use reqwest::Client;
use shared::{init::init_tracing, header_helper::get_logid_blocking, layer::{tracing_layer, logid_layer}};
use tokio::sync::OnceCell;
use tracing::{info, error, warn};
use handlers::{
    handle_get,
    handle_post,
    handle_patch,
    handle_delete
};

pub mod handlers;
pub mod util;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> Result<(), Error> {
    init_tracing();

    info!("Starting proxy_handler");

    info!("Creating client pool");

    let app_state = Arc::new(
        Client::builder()
        .connect_timeout(Duration::from_millis(1000))
        .build()?
    );

    info!("Creating routers");

    let router = Router::new()
        .route("/*path", get(handle_get))
        .route("/*path", post(handle_post))
        .route("/*path", patch(handle_patch))
        .route("/*path", delete(handle_delete))
        .layer(tracing_layer())
        .layer(logid_layer())
        .with_state(app_state.clone());

    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    info!("Creating listener");
    let listener = tokio::net::TcpListener::
        bind(format!("0.0.0.0:{port}")).await?;

    info!("Starting server");
    Ok(
        axum::serve(
            listener, 
            router
        ).await?
    )
}