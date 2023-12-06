use std::{fmt::Display, sync::Arc};

use axum::Router;
use tracing::info;
use tracing_subscriber::prelude::*;

use crate::{state::AppState, util_router};

pub fn init_tracing() {
    let filter_layer = tracing_subscriber::filter::LevelFilter::INFO;
    
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_file(true)
        .with_line_number(true);

    let subscriber = tracing_subscriber::Registry::default()
        .with(filter_layer)
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

pub async fn start_server<T>(router: Router<Arc<AppState<T>>>) -> Result<(), anyhow::Error>
where
T: Clone + Display + Send + Sync + 'static
{
    init_tracing();

    info!("Starting microservice");

    info!("Creating app_state");

    let app_state = Arc::new(AppState::<T>::new());

    info!("Creating routers");

    let router = Router::new()
        .merge(util_router::get_router())
        .merge(router)
        .with_state(app_state.clone());

    info!("Creating listener");

    let port = std::env::var("PORT").unwrap_or("8080".to_string());

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
