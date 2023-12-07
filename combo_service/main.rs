use std::{fmt::Display, fmt::Formatter, sync::Arc, time::Duration};

use anyhow::Error;
use axum::Router;
use reqwest::Client;
use shared::{init::{start_server, init_tracing}, util_router, layer::{tracing_layer, logid_layer}};
use tracing::info;

mod biz_router;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_tracing();

    info!("Starting combo_service");

    info!("Creating client pool");

    let app_state = Arc::new(
        Client::builder()
        .connect_timeout(Duration::from_millis(1000))
        .build()?
    );

    info!("Creating routers");

    let router = Router::new()
        .merge(util_router::get_router())
        .merge(biz_router::get_router())
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
