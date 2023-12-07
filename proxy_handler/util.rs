use core::panic;
use std::{sync::Arc, time::Duration};

use anyhow::Error;
use axum::{routing::{get, post, patch, delete}, Router, response::{IntoResponse, Response}, http::{StatusCode, HeaderValue}, extract::{Request, State}, Json, body::{Body, to_bytes}};
use hyper::header;
use reqwest::Client;
use shared::{init::init_tracing, header_helper::get_logid_blocking, layer::tracing_layer};
use tokio::sync::OnceCell;
use tracing::{info, error, warn};

pub fn convert_status(req_status: reqwest::StatusCode) -> StatusCode  {
    match req_status {
        reqwest::StatusCode::OK => StatusCode::OK,
        reqwest::StatusCode::CREATED => StatusCode::CREATED,
        reqwest::StatusCode::ACCEPTED => StatusCode::ACCEPTED,
        reqwest::StatusCode::NO_CONTENT => StatusCode::NO_CONTENT,
        reqwest::StatusCode::BAD_REQUEST => StatusCode::BAD_REQUEST,
        reqwest::StatusCode::UNAUTHORIZED => StatusCode::UNAUTHORIZED,
        reqwest::StatusCode::FORBIDDEN => StatusCode::FORBIDDEN,
        reqwest::StatusCode::NOT_FOUND => StatusCode::NOT_FOUND,
        reqwest::StatusCode::METHOD_NOT_ALLOWED => StatusCode::METHOD_NOT_ALLOWED,
        reqwest::StatusCode::CONFLICT => StatusCode::CONFLICT,
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => StatusCode::INTERNAL_SERVER_ERROR,
        reqwest::StatusCode::SERVICE_UNAVAILABLE => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR
    }

}

pub async fn get_service_name() -> String {
    OnceCell::<String>::new()
        .get_or_init(|| async {
            let service_key = "SERVICE_NAME";

            info!("getting key={service_key}");

            match std::env::var(service_key) {
                Ok(service) => {
                    info!("got service={service}");
                    service
                }
                Err(_) => {
                    panic!("{} not set", service_key);
                }
            }
        })
        .await
        .clone()
}

pub async fn get_port_for_service(service: &str) -> String {
    OnceCell::<String>::new()
        .get_or_init(|| async {
            let service_key = format!("{}_port", service).to_uppercase();

            info!("got service_key={service_key}");

            match std::env::var(service_key) {
                Ok(port) => {
                    info!("got service={service}, port={port}");
                    port
                }
                Err(_) => {
                    warn!("using default port for service={service}");
                    "8080".to_string()
                }
            }
        })
        .await
        .clone()
}

pub async fn get_canonical_name_for_service(service: &str) -> String {
    OnceCell::<String>::new()
        .get_or_init(|| async {
            let service_key = format!("{}_address", service).to_uppercase();

            info!("got service_key={service_key}");

            match std::env::var(service_key) {
                Ok(address) => {
                    info!("got service={service}, address={address}");
                    address
                }
                Err(_) => {
                    warn!("using default address for service={service}");
                    service.to_string()
                }
            }
        })
        .await
        .clone()
}

pub async fn get_backing_addres() -> String {
    let service = get_service_name().await;

    let port = get_port_for_service("service").await;

    let address = get_canonical_name_for_service("service").await;

    format!("http://{}:{}", address, port)
}

pub async fn rewrite_uri(req: &mut Request<Body>) {
    let backing_address = get_backing_addres().await;

    let uri = req.uri_mut();

    let path = uri.path_and_query().unwrap().as_str();

    let new_path = format!("{}{}", backing_address, path);

    *uri = new_path.parse().unwrap();
}