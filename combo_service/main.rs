use std::{fmt::Display, fmt::Formatter, sync::Arc, time::Duration};

use anyhow::Error;
use axum::Router;
use reqwest::Client;
use shared::{init::{start_server, init_tracing}, util_router};
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
        .with_state(app_state.clone());

    info!("Creating listener");
    let listener = tokio::net::TcpListener::
        bind("0.0.0.0:8080").await?;

    info!("Starting server");
    Ok(
        axum::serve(
            listener, 
            router
        ).await?
    )
}
