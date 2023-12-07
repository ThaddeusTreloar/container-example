use core::panic;
use std::{sync::Arc, time::Duration};

use anyhow::Error;
use axum::{routing::{get, post, patch, delete}, Router, response::{IntoResponse, Response}, http::{StatusCode, HeaderValue}, extract::{Request, State}, Json, body::{Body, to_bytes}};
use hyper::header;
use reqwest::Client;
use shared::{init::init_tracing, header_helper::get_logid_blocking, layer::tracing_layer};
use tokio::sync::OnceCell;
use tracing::{info, error, warn};

use super::util::{rewrite_uri, convert_status};

pub async fn handle_get(State(state): State<Arc<Client>>, mut req: Request) -> Result<Response, StatusCode> {
    let id = get_logid_blocking(req.headers());

    info!("Rewriting uri");
    rewrite_uri(&mut req).await;
    
    let req_uri = req.uri().to_string();
    info!("Sending request: {req_uri}");

    let res = state
        .get(req_uri)
        .header("logid", &id)
        .send()
        .await
        .map_err(|e| {
            error!("Error sending request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("Got response: {res:?}");

    let headers = res.headers().clone();

    let status = res.status();

    let mut response = match res.text().await {
        Ok(body) => Response::new(Body::from(body)),
        Err(_) => Response::new(Body::from(""))
    };

    let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(content_type) => content_type.as_bytes(),
        None => b""
    };

    response.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_bytes(content_type).unwrap());
    response.headers_mut().insert("logid", HeaderValue::from_str(&id).unwrap());

    *response.status_mut() = convert_status(status);

    info!("{response:?}");

    Ok(response)
}


pub async fn handle_delete(State(state): State<Arc<Client>>, mut req: Request) -> Result<Response, StatusCode> {
    let id = get_logid_blocking(req.headers());

    info!("Rewriting uri");
    rewrite_uri(&mut req).await;
    
    let req_uri = req.uri().to_string();
    info!("Sending request: {req_uri}");

    let res = state
        .delete(req_uri)
        .header("logid", &id)
        .send()
        .await
        .map_err(|e| {
            error!("Error sending request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("Got response: {res:?}");

    let headers = res.headers().clone();

    let status = res.status();

    let mut response = match res.text().await {
        Ok(body) => Response::new(Body::from(body)),
        Err(_) => Response::new(Body::from(""))
    };

    let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(content_type) => content_type.as_bytes(),
        None => b""
    };

    response.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_bytes(content_type).unwrap());
    response.headers_mut().insert("logid", HeaderValue::from_str(&id).unwrap());

    *response.status_mut() = convert_status(status);

    info!("{response:?}");

    Ok(response)
}

pub async fn handle_post(State(state): State<Arc<Client>>, mut req: Request) -> Result<Response, StatusCode> {
    let id = get_logid_blocking(req.headers());

    info!("Rewriting uri");

    rewrite_uri(&mut req).await;
    
    let req_uri = req.uri().to_string();
    info!("Sending request: {req_uri}");
    
    let req_headers = req.headers().clone();
    let req_content_type = req_headers.get(header::CONTENT_TYPE).unwrap().as_bytes();
    info!("Got content type: {req_content_type:?}");
    let body = match to_bytes(req.into_body(), 1024).await {
        Ok(body) => body,
        Err(e) => {
            error!("Error reading body: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let res = state
        .post(req_uri)
        .body(body)
        .header("logid", &id)
        .header(reqwest::header::CONTENT_TYPE, req_content_type)
        .send()
        .await
        .map_err(|e| {
            error!("Error sending request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("Got response: {res:?}");

    let headers = res.headers().clone();

    let status = res.status();

    let mut response = match res.text().await {
        Ok(body) => Response::new(Body::from(body)),
        Err(_) => Response::new(Body::from(""))
    };

    let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(content_type) => content_type.as_bytes(),
        None => b""
    };

    response.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_bytes(content_type).unwrap());
    response.headers_mut().insert("logid", HeaderValue::from_str(&id).unwrap());

    *response.status_mut() = convert_status(status);

    info!("{response:?}");

    Ok(response)
}

pub async fn handle_patch(State(state): State<Arc<Client>>, mut req: Request) -> Result<Response, StatusCode> {
    let id = get_logid_blocking(req.headers());

    info!("Rewriting uri");

    rewrite_uri(&mut req).await;
    
    let req_uri = req.uri().to_string();
    info!("Sending request: {req_uri}");
    
    let req_headers = req.headers().clone();
    let req_content_type = req_headers.get(header::CONTENT_TYPE).unwrap().as_bytes();
    info!("Got content type: {req_content_type:?}");
    let body = match to_bytes(req.into_body(), 1024).await {
        Ok(body) => body,
        Err(e) => {
            error!("Error reading body: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let res = state
        .patch(req_uri)
        .body(body)
        .header("logid", &id)
        .header(reqwest::header::CONTENT_TYPE, req_content_type)
        .send()
        .await
        .map_err(|e| {
            error!("Error sending request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("Got response: {res:?}");

    let headers = res.headers().clone();

    let status = res.status();

    let mut response = match res.text().await {
        Ok(body) => Response::new(Body::from(body)),
        Err(_) => Response::new(Body::from(""))
    };

    let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(content_type) => content_type.as_bytes(),
        None => b""
    };

    response.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_bytes(content_type).unwrap());
    response.headers_mut().insert("logid", HeaderValue::from_str(&id).unwrap());

    *response.status_mut() = convert_status(status);

    info!("{response:?}");

    Ok(response)
}